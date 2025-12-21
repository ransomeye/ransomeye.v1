// Path and File Name : /home/ransomeye/rebuild/ransomeye_policy/src/output.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Decision output - serializes policy decisions

/*
 * Decision Output
 * 
 * Serializes policy decisions for output.
 * No execution happens here - only decision output.
 */

use serde_json;
use tracing::debug;

use crate::decision::PolicyDecision;
use crate::errors::PolicyError;

pub struct DecisionOutput;

impl DecisionOutput {
    pub fn new() -> Self {
        Self
    }
    
    /// Serialize decision to JSON
    pub fn serialize(&self, decision: &PolicyDecision) -> Result<String, PolicyError> {
        serde_json::to_string_pretty(decision)
            .map_err(|e| PolicyError::InternalError(
                format!("Failed to serialize decision: {}", e)
            ))
    }
    
    /// Serialize decision to JSON (compact)
    pub fn serialize_compact(&self, decision: &PolicyDecision) -> Result<String, PolicyError> {
        serde_json::to_string(decision)
            .map_err(|e| PolicyError::InternalError(
                format!("Failed to serialize decision: {}", e)
            ))
    }
    
    /// Validate decision before output
    pub fn validate(&self, decision: &PolicyDecision) -> Result<(), PolicyError> {
        if !decision.verify() {
            return Err(PolicyError::EvaluationError(
                "Decision integrity verification failed".to_string()
            ));
        }
        
        if decision.decision_id.is_empty() {
            return Err(PolicyError::EvaluationError(
                "Decision ID is empty".to_string()
            ));
        }
        
        if decision.policy_id.is_empty() {
            return Err(PolicyError::EvaluationError(
                "Policy ID is empty".to_string()
            ));
        }
        
        debug!("Decision validated: {}", decision.decision_id);
        Ok(())
    }
}

