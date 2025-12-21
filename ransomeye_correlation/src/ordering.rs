// Path and File Name : /home/ransomeye/rebuild/ransomeye_correlation/src/ordering.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Event ordering validation - ensures events are processed in correct order

/*
 * Event Ordering
 * 
 * Validates event ordering per producer.
 * Ordering violation → DROP EVENT
 * Deterministic ordering checks only.
 */

use std::sync::Arc;
use dashmap::DashMap;
use parking_lot::RwLock;
use chrono::{DateTime, Utc};
use tracing::{error, debug, warn};

use crate::errors::CorrelationError;

#[derive(Clone, Debug)]
pub struct EventOrder {
    pub producer_id: String,
    pub sequence_number: u64,
    pub timestamp: DateTime<Utc>,
}

pub struct OrderingValidator {
    last_sequence: Arc<DashMap<String, RwLock<u64>>>,
    last_timestamp: Arc<DashMap<String, RwLock<DateTime<Utc>>>>,
}

impl OrderingValidator {
    pub fn new() -> Self {
        Self {
            last_sequence: Arc::new(DashMap::new()),
            last_timestamp: Arc::new(DashMap::new()),
        }
    }
    
    /// Validate event ordering
    /// Returns Ok(()) if ordering is valid, CorrelationError if violation detected
    /// Ordering violation → DROP EVENT (fail-closed)
    pub fn validate(&self, event: &EventOrder) -> Result<(), CorrelationError> {
        let producer_id = &event.producer_id;
        
        // Get or create producer state
        let seq_lock = self.last_sequence
            .entry(producer_id.clone())
            .or_insert_with(|| RwLock::new(0));
        
        let ts_lock = self.last_timestamp
            .entry(producer_id.clone())
            .or_insert_with(|| RwLock::new(DateTime::from_timestamp(0, 0).unwrap()));
        
        let mut last_seq = seq_lock.write();
        let mut last_ts = ts_lock.write();
        
        // Check sequence number monotonicity
        if event.sequence_number <= *last_seq && *last_seq > 0 {
            error!("Ordering violation: sequence number regression for producer {} (last: {}, current: {})",
                producer_id, *last_seq, event.sequence_number);
            return Err(CorrelationError::OrderingViolation(
                format!("Sequence number regression: {} <= {}", event.sequence_number, *last_seq)
            ));
        }
        
        // Check timestamp monotonicity (allow small tolerance for clock skew)
        let time_diff = (event.timestamp - *last_ts).num_seconds();
        if time_diff < -5 && *last_ts.timestamp() > 0 {
            // Allow 5 second backward tolerance for clock skew
            warn!("Timestamp regression detected for producer {} (diff: {}s), allowing due to clock skew tolerance",
                producer_id, time_diff);
        } else if time_diff < -60 {
            // More than 60 seconds backward is a violation
            error!("Ordering violation: timestamp regression for producer {} (last: {}, current: {})",
                producer_id, *last_ts, event.timestamp);
            return Err(CorrelationError::OrderingViolation(
                format!("Timestamp regression: {} < {}", event.timestamp, *last_ts)
            ));
        }
        
        // Update state
        *last_seq = event.sequence_number;
        *last_ts = event.timestamp;
        
        debug!("Ordering validated for producer: {} (seq: {}, ts: {})",
            producer_id, event.sequence_number, event.timestamp);
        
        Ok(())
    }
    
    /// Reset ordering state for a producer (for testing)
    pub fn reset(&self, producer_id: &str) {
        self.last_sequence.remove(producer_id);
        self.last_timestamp.remove(producer_id);
    }
}

