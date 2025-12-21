// Path and File Name : /home/ransomeye/rebuild/ransomeye_policy/src/matcher.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Policy matcher - deterministic policy matching

/*
 * Policy Matcher
 * 
 * Matches policies against evaluation context.
 * Deterministic matching only.
 * Ambiguity → DENY
 */

use serde_json::Value;
use tracing::{debug, warn};

use crate::errors::PolicyError;
use crate::context::EvaluationContext;
use crate::policy::PolicyRule;

pub struct PolicyMatcher;

impl PolicyMatcher {
    pub fn new() -> Self {
        Self
    }
    
    /// Match policy against context
    /// Returns true if policy matches, false otherwise
    pub fn matches(&self, policy: &PolicyRule, context: &EvaluationContext) -> Result<bool, PolicyError> {
        debug!("Matching policy {} against context", policy.id);
        
        // Check all match conditions
        for condition in &policy.match_conditions {
            if !self.evaluate_condition(condition, context)? {
                debug!("Policy {} does not match: condition failed", policy.id);
                return Ok(false);
            }
        }
        
        debug!("Policy {} matches context", policy.id);
        Ok(true)
    }
    
    fn evaluate_condition(&self, condition: &PolicyMatchCondition, context: &EvaluationContext) -> Result<bool, PolicyError> {
        let field_value = context.get_field(&condition.field)
            .ok_or_else(|| PolicyError::MissingContext(
                format!("Field {} not found in context", condition.field)
            ))?;
        
        match condition.operator.as_str() {
            "equals" => {
                Ok(field_value == condition.value)
            }
            "contains" => {
                if let (Some(s), Some(v)) = (field_value.as_str(), condition.value.as_str()) {
                    Ok(s.contains(v))
                } else {
                    Ok(false)
                }
            }
            "matches" => {
                // Simple string match (in production, would support regex)
                Ok(field_value == condition.value)
            }
            "in" => {
                if let Some(arr) = condition.value.as_array() {
                    Ok(arr.contains(&field_value))
                } else {
                    Ok(false)
                }
            }
            "greater_than" => {
                if let (Some(a), Some(b)) = (field_value.as_f64(), condition.value.as_f64()) {
                    Ok(a > b)
                } else {
                    Ok(false)
                }
            }
            "less_than" => {
                if let (Some(a), Some(b)) = (field_value.as_f64(), condition.value.as_f64()) {
                    Ok(a < b)
                } else {
                    Ok(false)
                }
            }
            _ => {
                warn!("Unknown operator: {}", condition.operator);
                Err(PolicyError::EvaluationError(
                    format!("Unknown operator: {}", condition.operator)
                ))
            }
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PolicyMatchCondition {
    pub field: String,
    pub operator: String,
    pub value: Value,
}

