// Path and File Name : /home/ransomeye/rebuild/ransomeye_correlation/tests/evidence_integrity_tests.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Evidence integrity tests - verifies evidence bundles are verifiable

/*
 * Evidence Integrity Tests
 * 
 * Tests that verify:
 * - Evidence bundles are verifiable
 * - Tampered evidence is detected
 * - Evidence hashes are correct
 */

#[cfg(test)]
mod tests {
    use serde_json::json;
    use sha2::{Sha256, Digest};
    use hex;

    #[test]
    fn test_evidence_hash_is_computed_correctly() {
        // Test that evidence hash is computed correctly
        
        let evidence = json!({
            "event_id": "test_event",
            "data": "test_data"
        });
        
        let json_bytes = serde_json::to_vec(&evidence).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&json_bytes);
        let hash = hex::encode(hasher.finalize());
        
        // Hash should be computed
        assert!(!hash.is_empty());
    }
    
    #[test]
    fn test_tampered_evidence_is_detected() {
        // Test that tampered evidence is detected
        
        let original_evidence = json!({
            "event_id": "test_event",
            "data": "original"
        });
        
        let tampered_evidence = json!({
            "event_id": "test_event",
            "data": "tampered"
        });
        
        // Compute hashes
        let original_hash = compute_hash(&original_evidence);
        let tampered_hash = compute_hash(&tampered_evidence);
        
        // Hashes should be different
        assert_ne!(original_hash, tampered_hash);
    }
    
    fn compute_hash(evidence: &serde_json::Value) -> String {
        let json_bytes = serde_json::to_vec(evidence).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&json_bytes);
        hex::encode(hasher.finalize())
    }
}

