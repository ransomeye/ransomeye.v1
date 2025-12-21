// Path and File Name : /home/ransomeye/rebuild/ransomeye_correlation/src/engine.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Core correlation engine - deterministic correlation authority

/*
 * Correlation Engine
 * 
 * The ONLY deterministic detection authority in RansomEye.
 * Same input → same output (always).
 * Ambiguous correlation → NO ALERT
 * State corruption → ENGINE HALT
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use serde_json::Value;
use chrono::{DateTime, Utc};
use tracing::{error, info, warn, debug};

use crate::errors::CorrelationError;
use crate::pipeline::{EventPipeline, ProcessedEvent};
use crate::output::Alert;

pub struct CorrelationEngine {
    pipeline: Arc<EventPipeline>,
    halted: Arc<AtomicBool>,
    engine_version: String,
}

impl CorrelationEngine {
    pub fn new(
        rules_path: &str,
        window_size_seconds: i64,
        max_events: usize,
        engine_version: &str,
    ) -> Result<Self, CorrelationError> {
        info!("Initializing Correlation Engine (version: {})", engine_version);
        
        let pipeline = Arc::new(EventPipeline::new(
            rules_path,
            window_size_seconds,
            max_events,
            engine_version,
        )?);
        
        Ok(Self {
            pipeline,
            halted: Arc::new(AtomicBool::new(false)),
            engine_version: engine_version.to_string(),
        })
    }
    
    /// Process event and generate alert if correlation detected
    /// Returns Option<Alert> if alert generated, None otherwise
    /// Ordering violation → DROP EVENT
    /// Ambiguous correlation → NO ALERT
    /// State corruption → ENGINE HALT
    pub fn process_event(
        &self,
        event_id: &str,
        producer_id: &str,
        sequence_number: u64,
        timestamp: DateTime<Utc>,
        event_type: &str,
        event_data: Value,
    ) -> Result<Option<Alert>, CorrelationError> {
        // Check if engine is halted
        if self.halted.load(Ordering::Relaxed) {
            return Err(CorrelationError::EngineHalted(
                "Engine is halted due to state corruption".to_string()
            ));
        }
        
        let processed_event = ProcessedEvent {
            event_id: event_id.to_string(),
            producer_id: producer_id.to_string(),
            sequence_number,
            timestamp,
            event_type: event_type.to_string(),
            event_data,
        };
        
        match self.pipeline.process_event(processed_event) {
            Ok(alert) => {
                if let Some(ref alert) = alert {
                    info!("Alert generated: {} (severity: {})", alert.alert_id, alert.severity);
                }
                Ok(alert)
            }
            Err(CorrelationError::StateCorruption(msg)) => {
                error!("State corruption detected: {}", msg);
                self.halt();
                Err(CorrelationError::EngineHalted(
                    format!("Engine halted due to state corruption: {}", msg)
                ))
            }
            Err(CorrelationError::OrderingViolation(_)) => {
                // Ordering violation → DROP EVENT (already logged)
                warn!("Event dropped due to ordering violation");
                Ok(None)
            }
            Err(e) => {
                error!("Error processing event: {}", e);
                Err(e)
            }
        }
    }
    
    /// Halt engine (called on state corruption)
    fn halt(&self) {
        self.halted.store(true, Ordering::Relaxed);
        error!("Correlation Engine HALTED - requires manual intervention");
    }
    
    /// Check if engine is halted
    pub fn is_halted(&self) -> bool {
        self.halted.load(Ordering::Relaxed)
    }
    
    /// Get engine version
    pub fn version(&self) -> &str {
        &self.engine_version
    }
    
    /// Reset engine (for testing only)
    #[cfg(test)]
    pub fn reset(&self) {
        self.halted.store(false, Ordering::Relaxed);
    }
}

