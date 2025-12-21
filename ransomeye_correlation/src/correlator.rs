// Path and File Name : /home/ransomeye/rebuild/ransomeye_correlation/src/correlator.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Rule-based correlator - deterministic rule matching

/*
 * Rule-Based Correlator
 * 
 * Matches events against rules deterministically.
 * Ambiguous correlation → NO ALERT
 */

use std::sync::Arc;
use serde_json::Value;
use chrono::{DateTime, Utc};
use tracing::{error, debug, warn};

use crate::errors::CorrelationError;
use crate::rules::{Rule, Condition};
use crate::window::SlidingWindow;

#[derive(Debug, Clone)]
pub struct CorrelationResult {
    pub rule_id: String,
    pub matched: bool,
    pub confidence: String,
    pub matched_events: Vec<Value>,
    pub correlation_window_start: DateTime<Utc>,
    pub correlation_window_end: DateTime<Utc>,
}

pub struct Correlator {
    window: Arc<SlidingWindow>,
}

impl Correlator {
    pub fn new(window: Arc<SlidingWindow>) -> Self {
        Self {
            window,
        }
    }
    
    /// Correlate events against rule
    /// Returns CorrelationResult on success, CorrelationError on ambiguity
    pub fn correlate(
        &self,
        rule: &Rule,
        entity_id: &str,
        events: &[Value],
    ) -> Result<CorrelationResult, CorrelationError> {
        if !rule.enabled {
            return Err(CorrelationError::RuleNotFound(
                format!("Rule {} is disabled", rule.id)
            ));
        }
        
        debug!("Correlating events against rule: {}", rule.id);
        
        // Check all conditions
        let mut matched_events = Vec::new();
        let mut all_conditions_met = true;
        
        for condition in &rule.conditions {
            let condition_met = self.evaluate_condition(condition, events)?;
            
            if !condition_met {
                if condition.required {
                    all_conditions_met = false;
                    debug!("Required condition not met: {} {}", condition.field, condition.operator);
                    break;
                } else {
                    warn!("Optional condition not met: {} {}", condition.field, condition.operator);
                }
            } else {
                // Find matching events
                for event in events {
                    if self.event_matches_condition(event, condition) {
                        matched_events.push(event.clone());
                    }
                }
            }
        }
        
        if !all_conditions_met {
            return Err(CorrelationError::AmbiguousCorrelation(
                format!("Not all required conditions met for rule {}", rule.id)
            ));
        }
        
        // Determine correlation window
        let window_start = self.window.get_events_in_window(entity_id, Utc::now())
            .first()
            .map(|e| e.timestamp)
            .unwrap_or_else(Utc::now);
        
        let window_end = Utc::now();
        
        let result = CorrelationResult {
            rule_id: rule.id.clone(),
            matched: all_conditions_met,
            confidence: rule.confidence.clone(),
            matched_events,
            correlation_window_start: window_start,
            correlation_window_end: window_end,
        };
        
        debug!("Correlation result for rule {}: matched={}", rule.id, all_conditions_met);
        
        Ok(result)
    }
    
    fn evaluate_condition(&self, condition: &Condition, events: &[Value]) -> Result<bool, CorrelationError> {
        // Check if any event matches the condition
        for event in events {
            if self.event_matches_condition(event, condition) {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    fn event_matches_condition(&self, event: &Value, condition: &Condition) -> bool {
        // Get field value from event
        let field_value = event.get(&condition.field);
        
        if field_value.is_none() {
            return false;
        }
        
        let field_value = field_value.unwrap();
        
        // Evaluate operator
        match condition.operator.as_str() {
            "equals" => {
                field_value == &condition.value
            }
            "contains" => {
                if let Some(s) = field_value.as_str() {
                    if let Some(v) = condition.value.as_str() {
                        s.contains(v)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            "matches" => {
                // Simple string match (in production, would support regex)
                field_value == &condition.value
            }
            "greater_than" => {
                if let (Some(a), Some(b)) = (field_value.as_f64(), condition.value.as_f64()) {
                    a > b
                } else {
                    false
                }
            }
            "less_than" => {
                if let (Some(a), Some(b)) = (field_value.as_f64(), condition.value.as_f64()) {
                    a < b
                } else {
                    false
                }
            }
            "in" => {
                if let Some(arr) = condition.value.as_array() {
                    arr.contains(field_value)
                } else {
                    false
                }
            }
            _ => {
                warn!("Unknown operator: {}", condition.operator);
                false
            }
        }
    }
}

