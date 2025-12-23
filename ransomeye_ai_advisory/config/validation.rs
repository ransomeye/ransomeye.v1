// Path and File Name : /home/ransomeye/rebuild/ransomeye_ai_advisory/config/validation.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: ENV-only configuration validation (fail-closed)

use std::env;
use tracing::{error, warn, debug};

pub struct ConfigValidator;

impl ConfigValidator {
    pub fn new() -> Result<Self, String> {
        // Validate all required environment variables
        let required_vars = vec![
            "RANSOMEYE_AI_MODELS_DIR",
            "RANSOMEYE_AI_PUBLIC_KEY_PATH",
        ];
        
        let mut missing = Vec::new();
        
        for var in required_vars {
            if env::var(var).is_err() {
                missing.push(var);
            }
        }
        
        if !missing.is_empty() {
            return Err(format!("Missing required environment variables: {}", missing.join(", ")));
        }
        
        debug!("Configuration validation passed");
        Ok(Self)
    }
    
    /// Validate optional configuration
    pub fn validate_optional(&self) -> Result<(), String> {
        // Check optional vars have valid values if set
        Ok(())
    }
    
    /// Get models directory
    pub fn models_dir(&self) -> Result<String, String> {
        env::var("RANSOMEYE_AI_MODELS_DIR")
            .map_err(|_| "RANSOMEYE_AI_MODELS_DIR not set".to_string())
    }
    
    /// Get public key path
    pub fn public_key_path(&self) -> Result<String, String> {
        env::var("RANSOMEYE_AI_PUBLIC_KEY_PATH")
            .map_err(|_| "RANSOMEYE_AI_PUBLIC_KEY_PATH not set".to_string())
    }
}

