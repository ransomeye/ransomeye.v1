// Path and File Name : /home/ransomeye/rebuild/ransomeye_correlation/src/pipeline.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Event processing pipeline - deterministic event processing

/*
 * Event Processing Pipeline
 * 
 * Processes events through deterministic pipeline.
 * Ordering violation → DROP EVENT
 * Ambiguous correlation → NO ALERT
 */

use std::sync::Arc;
use serde_json::Value;
use chrono::{DateTime, Utc};
use tracing::{error, debug, warn};

use crate::errors::CorrelationError;
use crate::ordering::{OrderingValidator, EventOrder};
use crate::window::{SlidingWindow, WindowedEvent};
use crate::rules::RuleEngine;
use crate::correlator::Correlator;
use crate::state::{StateMachine, State};
use crate::kill_chain::KillChainInferencer;
use crate::evidence::{EvidenceBuilder, StateTransition as EvidenceStateTransition};
use crate::output::{AlertBuilder, Alert};

#[derive(Debug, Clone)]
pub struct ProcessedEvent {
    pub event_id: String,
    pub producer_id: String,
    pub sequence_number: u64,
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub event_data: Value,
}

pub struct EventPipeline {
    ordering_validator: Arc<OrderingValidator>,
    window: Arc<SlidingWindow>,
    rule_engine: Arc<RuleEngine>,
    correlator: Arc<Correlator>,
    state_machine: Arc<StateMachine>,
    kill_chain: Arc<KillChainInferencer>,
    evidence_builder: Arc<EvidenceBuilder>,
    alert_builder: Arc<AlertBuilder>,
}

impl EventPipeline {
    pub fn new(
        rules_path: &str,
        window_size_seconds: i64,
        max_events: usize,
        engine_version: &str,
    ) -> Result<Self, CorrelationError> {
        let ordering_validator = Arc::new(OrderingValidator::new());
        let window = Arc::new(SlidingWindow::new(window_size_seconds, max_events));
        let rule_engine = Arc::new(RuleEngine::new(rules_path)?);
        let correlator = Arc::new(Correlator::new(window.clone()));
        let state_machine = Arc::new(StateMachine::new());
        let kill_chain = Arc::new(KillChainInferencer::new()?);
        let evidence_builder = Arc::new(EvidenceBuilder::new(engine_version));
        let alert_builder = Arc::new(AlertBuilder::new(engine_version));
        
        Ok(Self {
            ordering_validator,
            window,
            rule_engine,
            correlator,
            state_machine,
            kill_chain,
            evidence_builder,
            alert_builder,
        })
    }
    
    /// Process event through pipeline
    /// Returns Option<Alert> if alert generated, None otherwise
    /// Ordering violation → DROP EVENT
    /// Ambiguous correlation → NO ALERT
    pub fn process_event(&self, event: ProcessedEvent) -> Result<Option<Alert>, CorrelationError> {
        debug!("Processing event: {} from producer: {}", event.event_id, event.producer_id);
        
        // Step 1: Validate ordering
        let event_order = EventOrder {
            producer_id: event.producer_id.clone(),
            sequence_number: event.sequence_number,
            timestamp: event.timestamp,
        };
        
        self.ordering_validator.validate(&event_order)
            .map_err(|e| {
                error!("Ordering violation, dropping event: {}", e);
                e
            })?;
        
        // Step 2: Add to window
        let windowed_event = WindowedEvent {
            event_id: event.event_id.clone(),
            timestamp: event.timestamp,
            data: event.event_data.clone(),
        };
        
        let window_key = format!("{}:{}", event.producer_id, event.event_type);
        self.window.add_event(&window_key, windowed_event)
            .map_err(|e| {
                error!("Window overflow, dropping event: {}", e);
                e
            })?;
        
        // Step 3: Get events in window
        let window_events: Vec<Value> = self.window.get_events_in_window(&window_key, Utc::now())
            .iter()
            .map(|e| e.data.clone())
            .collect();
        
        // Step 4: Infer kill-chain stage
        let kill_chain_stage = self.kill_chain.infer_stage(&event.event_type, &event.event_data)
            .unwrap_or(State::Initial);
        
        // Step 5: Try to match against rules
        let mut matched_rules = Vec::new();
        let mut correlation_results = Vec::new();
        
        for (rule_id, rule) in self.rule_engine.get_all_rules() {
            match self.correlator.correlate(rule, &event.producer_id, &window_events) {
                Ok(result) if result.matched => {
                    matched_rules.push(rule_id.clone());
                    correlation_results.push(result);
                    debug!("Rule matched: {}", rule_id);
                }
                Ok(_) => {
                    // Rule didn't match, continue
                }
                Err(CorrelationError::AmbiguousCorrelation(_)) => {
                    // Ambiguous correlation → NO ALERT
                    warn!("Ambiguous correlation for rule {}, no alert generated", rule_id);
                    return Ok(None);
                }
                Err(e) => {
                    // Other error, log and continue
                    warn!("Error correlating rule {}: {}", rule_id, e);
                }
            }
        }
        
        // Step 6: If no rules matched, no alert
        if matched_rules.is_empty() {
            return Ok(None);
        }
        
        // Step 7: Update state machine
        let entity_id = event.producer_id.clone();
        self.state_machine.transition(&entity_id, kill_chain_stage.clone(), matched_rules[0].clone())
            .map_err(|e| {
                error!("State transition failed: {}", e);
                e
            })?;
        
        // Step 8: Build evidence bundle
        let state_history = self.state_machine.get_state_history(&entity_id);
        let evidence_transitions: Vec<EvidenceStateTransition> = state_history.iter()
            .map(|t| EvidenceStateTransition {
                from: crate::kill_chain::KillChainInferencer::stage_to_string(&t.from),
                to: crate::kill_chain::KillChainInferencer::stage_to_string(&t.to),
                timestamp: t.timestamp,
                trigger: t.trigger.clone(),
            })
            .collect();
        
        let correlation_result = &correlation_results[0];
        let evidence_bundle = self.evidence_builder.build(
            &event.event_id,
            matched_rules.clone(),
            kill_chain_stage.clone(),
            window_events.clone(),
            evidence_transitions,
            correlation_result.correlation_window_start,
            correlation_result.correlation_window_end,
        )?;
        
        // Step 9: Build alert
        let rule = self.rule_engine.get_rule(&matched_rules[0])?;
        let alert = self.alert_builder.build_alert(
            &format!("Correlation Alert: {}", rule.name),
            &rule.description,
            &self.determine_severity(&kill_chain_stage),
            &rule.confidence,
            &entity_id,
            kill_chain_stage.clone(),
            matched_rules.clone(),
            evidence_bundle,
            state_history.iter()
                .map(|t| format!("{:?} -> {:?}", t.from, t.to))
                .collect(),
        );
        
        debug!("Alert generated: {} (severity: {})", alert.alert_id, alert.severity);
        
        Ok(Some(alert))
    }
    
    fn determine_severity(&self, stage: &State) -> String {
        match stage {
            State::ActionsOnObjectives => "critical".to_string(),
            State::CommandControl => "high".to_string(),
            State::Installation => "high".to_string(),
            State::Exploitation => "medium".to_string(),
            State::Delivery => "medium".to_string(),
            State::Weaponization => "low".to_string(),
            State::Reconnaissance => "low".to_string(),
            _ => "medium".to_string(),
        }
    }
}

