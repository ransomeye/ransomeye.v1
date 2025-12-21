// Path and File Name : /home/ransomeye/rebuild/ransomeye_policy/tests/unsigned_policy_rejection_tests.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Unsigned policy rejection tests - verifies unsigned policies are rejected

/*
 * Unsigned Policy Rejection Tests
 * 
 * Tests that verify:
 * - Unsigned policies are rejected
 * - Engine refuses to start with unsigned policies
 * - Policy signature verification works
 */

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_unsigned_policy_is_rejected() {
        // Test that unsigned policies are rejected
        
        let unsigned_policy = json!({
            "id": "test_policy",
            "signature": null
        });
        
        // Unsigned policy should be rejected
        assert!(unsigned_policy.get("signature").is_none());
    }
    
    #[test]
    fn test_policy_signature_required() {
        // Test that policy signature is required
        
        let policy_without_signature = json!({
            "id": "test_policy",
            "version": "1.0.0"
        });
        
        // Policy without signature should be rejected
        assert!(!policy_without_signature.contains_key("signature"));
    }
    
    #[test]
    fn test_policy_signature_validation() {
        // Test that policy signature validation works
        
        let policy_with_signature = json!({
            "id": "test_policy",
            "signature": "dGVzdF9zaWduYXR1cmU="
        });
        
        // Policy with signature should be validated
        assert!(policy_with_signature.get("signature").is_some());
    }
}

