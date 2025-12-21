// Path and File Name : /home/ransomeye/rebuild/ransomeye_policy/src/errors.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Error types for policy engine - all errors are fail-closed

/*
 * Policy Engine Errors
 * 
 * All errors are fail-closed.
 * Ambiguity → DENY
 * Unsigned policy → ENGINE REFUSES TO START
 * Missing context → DENY
 */

use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum PolicyError {
    #[error("Policy not found: {0}")]
    PolicyNotFound(String),
    
    #[error("Policy signature invalid: {0}")]
    PolicySignatureInvalid(String),
    
    #[error("Policy version mismatch: expected {0}, got {1}")]
    PolicyVersionMismatch(String, String),
    
    #[error("Policy ambiguity: {0}")]
    PolicyAmbiguity(String),
    
    #[error("Missing context: {0}")]
    MissingContext(String),
    
    #[error("Evaluation error: {0}")]
    EvaluationError(String),
    
    #[error("Policy revoked: {0}")]
    PolicyRevoked(String),
    
    #[error("Unsigned policy: {0}")]
    UnsignedPolicy(String),
    
    #[error("Policy tampered: {0}")]
    PolicyTampered(String),
    
    #[error("No matching policy: {0}")]
    NoMatchingPolicy(String),
    
    #[error("Engine refused to start: {0}")]
    EngineRefusedToStart(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

