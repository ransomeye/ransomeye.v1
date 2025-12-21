// Path and File Name : /home/ransomeye/rebuild/ransomeye_ingestion/src/normalization.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Event normalization - normalizes events to standard format

/*
 * Event Normalization
 * 
 * Normalizes events to standard format.
 * Ensures consistent event structure.
 */

use crate::protocol::EventEnvelope;

pub struct EventNormalizer;

impl EventNormalizer {
    pub fn normalize(envelope: &EventEnvelope) -> Result<EventEnvelope, Box<dyn std::error::Error>> {
        // Normalize envelope fields
        let mut normalized = envelope.clone();
        
        // Normalize component type to lowercase
        normalized.component_type = normalized.component_type.to_lowercase();
        
        // Normalize producer ID (trim whitespace)
        normalized.producer_id = normalized.producer_id.trim().to_string();
        
        Ok(normalized)
    }
}

