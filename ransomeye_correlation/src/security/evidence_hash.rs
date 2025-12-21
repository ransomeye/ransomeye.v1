// Path and File Name : /home/ransomeye/rebuild/ransomeye_correlation/src/security/evidence_hash.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Evidence hashing - creates cryptographic hashes of evidence bundles

/*
 * Evidence Hashing
 * 
 * Creates cryptographic hashes of evidence bundles.
 * Ensures evidence integrity and verifiability.
 */

use sha2::{Sha256, Digest};
use serde_json::Value;
use hex;

pub struct EvidenceHasher;

impl EvidenceHasher {
    pub fn new() -> Self {
        Self
    }
    
    /// Hash evidence bundle
    /// Returns hex-encoded SHA-256 hash
    pub fn hash_evidence(&self, evidence: &Value) -> String {
        let json_bytes = serde_json::to_vec(evidence)
            .expect("Failed to serialize evidence");
        
        let mut hasher = Sha256::new();
        hasher.update(&json_bytes);
        let hash = hasher.finalize();
        
        hex::encode(hash)
    }
    
    /// Hash event data
    pub fn hash_event(&self, event_data: &Value) -> String {
        let json_bytes = serde_json::to_vec(event_data)
            .expect("Failed to serialize event");
        
        let mut hasher = Sha256::new();
        hasher.update(&json_bytes);
        let hash = hasher.finalize();
        
        hex::encode(hash)
    }
    
    /// Hash multiple events
    pub fn hash_events(&self, events: &[Value]) -> String {
        let mut hasher = Sha256::new();
        
        for event in events {
            let json_bytes = serde_json::to_vec(event)
                .expect("Failed to serialize event");
            hasher.update(&json_bytes);
        }
        
        let hash = hasher.finalize();
        hex::encode(hash)
    }
}

