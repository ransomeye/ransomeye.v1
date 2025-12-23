// Path and File Name : /home/ransomeye/rebuild/core/governor/src/lib.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Core governor - fail-closed trust initialization

use std::path::Path;
use std::fs;
use thiserror::Error;
use tracing::{error, info};

#[derive(Debug, Error)]
pub enum GovernorError {
    #[error("Trust material missing: {0}")]
    TrustMaterialMissing(String),
    #[error("Failed to initialize trust: {0}")]
    TrustInitFailed(String),
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

/// Core governor with fail-closed trust enforcement
pub struct Governor {
    policy_key_path: String,
    trust_store_path: String,
    initialized: bool,
}

impl Governor {
    /// Initialize governor with fail-closed trust enforcement
    /// 
    /// FAIL-CLOSED: Returns error if trust material is missing
    pub fn new() -> Result<Self, GovernorError> {
        // Require policy signing key from environment
        let policy_key_path = std::env::var("RANSOMEYE_GOVERNOR_POLICY_KEY_PATH")
            .map_err(|_| GovernorError::TrustMaterialMissing(
                "RANSOMEYE_GOVERNOR_POLICY_KEY_PATH environment variable not set".to_string()
            ))?;
        
        // Require trust store path from environment
        let trust_store_path = std::env::var("RANSOMEYE_GOVERNOR_TRUST_STORE_PATH")
            .map_err(|_| GovernorError::TrustMaterialMissing(
                "RANSOMEYE_GOVERNOR_TRUST_STORE_PATH environment variable not set".to_string()
            ))?;
        
        // Verify policy key file exists
        if !Path::new(&policy_key_path).exists() {
            return Err(GovernorError::TrustMaterialMissing(
                format!("Policy key file not found: {}", policy_key_path)
            ));
        }
        
        // Verify trust store directory exists
        if !Path::new(&trust_store_path).exists() {
            return Err(GovernorError::TrustMaterialMissing(
                format!("Trust store directory not found: {}", trust_store_path)
            ));
        }
        
        // Verify policy key file is not empty
        let key_size = fs::metadata(&policy_key_path)
            .map_err(|e| GovernorError::TrustInitFailed(
                format!("Failed to read policy key metadata: {}", e)
            ))?
            .len();
        
        if key_size == 0 {
            return Err(GovernorError::TrustMaterialMissing(
                format!("Policy key file is empty: {}", policy_key_path)
            ));
        }
        
        // Verify trust store contains at least one key
        let trust_store = Path::new(&trust_store_path);
        let mut has_keys = false;
        if trust_store.is_dir() {
            for entry in fs::read_dir(trust_store)
                .map_err(|e| GovernorError::TrustInitFailed(
                    format!("Failed to read trust store directory: {}", e)
                ))? {
                let entry = entry.map_err(|e| GovernorError::TrustInitFailed(
                    format!("Failed to read trust store entry: {}", e)
                ))?;
                let path = entry.path();
                if path.is_file() {
                    let ext = path.extension().and_then(|s| s.to_str());
                    if ext == Some("pem") || ext == Some("der") || ext == Some("pub") {
                        has_keys = true;
                        break;
                    }
                }
            }
        }
        
        if !has_keys {
            return Err(GovernorError::TrustMaterialMissing(
                format!("Trust store contains no keys: {}", trust_store_path)
            ));
        }
        
        info!("Governor initialized with policy key: {} and trust store: {}", 
              policy_key_path, trust_store_path);
        
        Ok(Self {
            policy_key_path,
            trust_store_path,
            initialized: true,
        })
    }
    
    /// Verify governor is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    /// Get policy key path
    pub fn policy_key_path(&self) -> &str {
        &self.policy_key_path
    }
    
    /// Get trust store path
    pub fn trust_store_path(&self) -> &str {
        &self.trust_store_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_governor_fails_without_trust_material() {
        // Unset required environment variables
        std::env::remove_var("RANSOMEYE_GOVERNOR_POLICY_KEY_PATH");
        std::env::remove_var("RANSOMEYE_GOVERNOR_TRUST_STORE_PATH");
        
        let result = Governor::new();
        assert!(result.is_err());
        
        if let Err(GovernorError::TrustMaterialMissing(_)) = result {
            // Expected error
        } else {
            panic!("Expected TrustMaterialMissing error");
        }
    }
}
