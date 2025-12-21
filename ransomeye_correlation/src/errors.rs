// Path and File Name : /home/ransomeye/rebuild/ransomeye_correlation/src/errors.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Error types for correlation engine - all errors are fail-closed

/*
 * Correlation Engine Errors
 * 
 * All errors are fail-closed.
 * Ambiguity → NO ALERT
 * Missing rule → NO ALERT
 * Ordering violation → DROP EVENT
 * State corruption → ENGINE HALT
 */

use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum CorrelationError {
    #[error("Rule not found: {0}")]
    RuleNotFound(String),
    
    #[error("Rule signature invalid: {0}")]
    RuleSignatureInvalid(String),
    
    #[error("Rule version mismatch: expected {0}, got {1}")]
    RuleVersionMismatch(String, String),
    
    #[error("Ambiguous correlation: {0}")]
    AmbiguousCorrelation(String),
    
    #[error("Ordering violation: {0}")]
    OrderingViolation(String),
    
    #[error("State corruption detected: {0}")]
    StateCorruption(String),
    
    #[error("Event dropped: {0}")]
    EventDropped(String),
    
    #[error("Window overflow: {0}")]
    WindowOverflow(String),
    
    #[error("Kill-chain stage violation: {0}")]
    KillChainStageViolation(String),
    
    #[error("Evidence bundle creation failed: {0}")]
    EvidenceBundleFailed(String),
    
    #[error("Signature verification failed: {0}")]
    SignatureVerificationFailed(String),
    
    #[error("Invalid event: {0}")]
    InvalidEvent(String),
    
    #[error("Engine halted: {0}")]
    EngineHalted(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

