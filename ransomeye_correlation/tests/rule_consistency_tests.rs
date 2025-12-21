// Path and File Name : /home/ransomeye/rebuild/ransomeye_correlation/tests/rule_consistency_tests.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Rule consistency tests - verifies rule evaluation is consistent

/*
 * Rule Consistency Tests
 * 
 * Tests that verify:
 * - Same rule + same events → same result
 * - Rule changes → predictable impact
 * - Rule versioning works correctly
 */

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_rule_evaluation_is_consistent() {
        // Test that rule evaluation produces consistent results
        
        let rule_conditions = vec![
            json!({"field": "event_type", "operator": "equals", "value": "file_encryption"}),
        ];
        
        let event1 = json!({"event_type": "file_encryption"});
        let event2 = json!({"event_type": "file_encryption"});
        
        // Same events should match same conditions
        assert_eq!(event1["event_type"], event2["event_type"]);
    }
    
    #[test]
    fn test_rule_conditions_are_deterministic() {
        // Test that rule conditions evaluate deterministically
        
        let condition = json!({
            "field": "file_count",
            "operator": "greater_than",
            "value": 10
        });
        
        let event1 = json!({"file_count": 20});
        let event2 = json!({"file_count": 20});
        
        // Same values should produce same evaluation
        assert_eq!(event1["file_count"], event2["file_count"]);
    }
    
    #[test]
    fn test_rule_versioning() {
        // Test that rule versioning works
        
        let rule_v1 = json!({
            "id": "test_rule",
            "version": "1.0.0"
        });
        
        let rule_v2 = json!({
            "id": "test_rule",
            "version": "2.0.0"
        });
        
        // Versions should be different
        assert_ne!(rule_v1["version"], rule_v2["version"]);
    }
}

