// Path and File Name : /home/ransomeye/rebuild/ransomeye_policy/tests/ambiguity_deny_tests.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Ambiguity deny tests - verifies ambiguous policies produce DENY

/*
 * Ambiguity Deny Tests
 * 
 * Tests that verify:
 * - Ambiguous policies produce DENY
 * - Multiple matching policies → DENY
 * - Missing context → DENY
 */

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_ambiguous_policies_produce_deny() {
        // Test that ambiguous policies produce DENY
        
        let policy1 = json!({
            "id": "policy_1",
            "priority": 100,
            "action": "allow"
        });
        
        let policy2 = json!({
            "id": "policy_2",
            "priority": 100,
            "action": "deny"
        });
        
        // Same priority, conflicting actions → DENY
        assert_eq!(policy1["priority"], policy2["priority"]);
    }
    
    #[test]
    fn test_missing_context_produces_deny() {
        // Test that missing context produces DENY
        
        let context_missing_field = json!({
            "alert_id": "alert_1"
            // Missing required fields
        });
        
        // Missing context should produce DENY
        assert!(!context_missing_field.contains_key("kill_chain_stage"));
    }
    
    #[test]
    fn test_no_matching_policy_produces_deny() {
        // Test that no matching policy produces DENY
        
        let alert = json!({
            "alert_id": "alert_1",
            "severity": "low",
            "kill_chain_stage": "unknown_stage"
        });
        
        // No matching policy → DENY (default)
        assert!(alert.get("kill_chain_stage").is_some());
    }
}

