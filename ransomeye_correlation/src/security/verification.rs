// Path and File Name : /home/ransomeye/rebuild/ransomeye_correlation/src/security/verification.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Evidence verification - verifies evidence bundle integrity

/*
 * Evidence Verification
 * 
 * Verifies evidence bundle integrity.
 * Tampered evidence → verification failure
 */

use serde_json::Value;
use sha2::{Sha256, Digest};
use hex;
use tracing::{error, debug};

use crate::security::evidence_hash::EvidenceHasher;

pub struct EvidenceVerifier {
    hasher: EvidenceHasher,
}

impl EvidenceVerifier {
    pub fn new() -> Self {
        Self {
            hasher: EvidenceHasher::new(),
        }
    }
    
    /// Verify evidence bundle integrity
    /// Returns true if evidence is valid, false otherwise
    pub fn verify(&self, evidence: &Value, expected_hash: &str) -> bool {
        let computed_hash = self.hasher.hash_evidence(evidence);
        
        if computed_hash == expected_hash {
            debug!("Evidence bundle verification successful");
            true
        } else {
            error!("Evidence bundle verification failed (hash mismatch)");
            false
        }
    }
    
    /// Verify event hash
    pub fn verify_event(&self, event: &Value, expected_hash: &str) -> bool {
        let computed_hash = self.hasher.hash_event(event);
        
        if computed_hash == expected_hash {
            debug!("Event verification successful");
            true
        } else {
            error!("Event verification failed (hash mismatch)");
            false
        }
    }
    
    /// Verify events hash
    pub fn verify_events(&self, events: &[Value], expected_hash: &str) -> bool {
        let computed_hash = self.hasher.hash_events(events);
        
        if computed_hash == expected_hash {
            debug!("Events verification successful");
            true
        } else {
            error!("Events verification failed (hash mismatch)");
            false
        }
    }
}

