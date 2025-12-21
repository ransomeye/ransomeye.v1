// Path and File Name : /home/ransomeye/rebuild/ransomeye_correlation/src/rules.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Rule engine - loads and validates signed YAML rules

/*
 * Rule Engine
 * 
 * Loads and validates signed YAML rules.
 * Unsigned rule → engine refuses to start.
 * Rules must be declarative, versioned, and signed.
 */

use std::path::Path;
use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use tracing::{error, info, warn, debug};

use crate::errors::CorrelationError;
use crate::security::signature::RuleSignatureVerifier;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub version: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub conditions: Vec<Condition>,
    pub actions: Vec<Action>,
    pub kill_chain_stage: Option<String>,
    pub confidence: String, // "high", "medium", "low"
    pub signature: Option<String>, // Base64 encoded signature
    pub signature_hash: Option<String>, // SHA-256 hash of rule content
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub field: String,
    pub operator: String, // "equals", "contains", "matches", "greater_than", "less_than", "in"
    pub value: serde_json::Value,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action_type: String, // "alert", "escalate", "block"
    pub parameters: HashMap<String, serde_json::Value>,
}

pub struct RuleEngine {
    rules: HashMap<String, Rule>,
    signature_verifier: RuleSignatureVerifier,
    rules_path: String,
}

impl RuleEngine {
    pub fn new(rules_path: &str) -> Result<Self, CorrelationError> {
        let signature_verifier = RuleSignatureVerifier::new()
            .map_err(|e| CorrelationError::ConfigurationError(
                format!("Failed to initialize signature verifier: {}", e)
            ))?;
        
        let mut engine = Self {
            rules: HashMap::new(),
            signature_verifier,
            rules_path: rules_path.to_string(),
        };
        
        engine.load_rules()?;
        
        Ok(engine)
    }
    
    /// Load rules from directory
    /// Unsigned rule → engine refuses to start
    pub fn load_rules(&mut self) -> Result<(), CorrelationError> {
        info!("Loading rules from: {}", self.rules_path);
        
        let rules_dir = Path::new(&self.rules_path);
        if !rules_dir.exists() {
            return Err(CorrelationError::ConfigurationError(
                format!("Rules directory not found: {}", self.rules_path)
            ));
        }
        
        let entries = fs::read_dir(rules_dir)
            .map_err(|e| CorrelationError::ConfigurationError(
                format!("Failed to read rules directory: {}", e)
            ))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| CorrelationError::ConfigurationError(
                format!("Failed to read directory entry: {}", e)
            ))?;
            
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") || 
               path.extension().and_then(|s| s.to_str()) == Some("yml") {
                match self.load_rule_file(&path) {
                    Ok(rule) => {
                        info!("Loaded rule: {} (version: {})", rule.id, rule.version);
                        self.rules.insert(rule.id.clone(), rule);
                    }
                    Err(e) => {
                        error!("Failed to load rule from {}: {}", path.display(), e);
                        return Err(e);
                    }
                }
            }
        }
        
        if self.rules.is_empty() {
            return Err(CorrelationError::ConfigurationError(
                "No valid rules loaded".to_string()
            ));
        }
        
        info!("Loaded {} rules", self.rules.len());
        Ok(())
    }
    
    fn load_rule_file(&self, path: &Path) -> Result<Rule, CorrelationError> {
        let content = fs::read_to_string(path)
            .map_err(|e| CorrelationError::ConfigurationError(
                format!("Failed to read rule file {}: {}", path.display(), e)
            ))?;
        
        // Parse YAML
        let mut rule: Rule = serde_yaml::from_str(&content)
            .map_err(|e| CorrelationError::ConfigurationError(
                format!("Failed to parse rule file {}: {}", path.display(), e)
            ))?;
        
        // Verify signature if present
        if let Some(ref signature) = rule.signature {
            let verified = self.signature_verifier.verify(&content, signature)
                .map_err(|e| CorrelationError::RuleSignatureInvalid(
                    format!("Rule {} signature verification failed: {}", rule.id, e)
                ))?;
            
            if !verified {
                return Err(CorrelationError::RuleSignatureInvalid(
                    format!("Rule {} has invalid signature", rule.id)
                ));
            }
            
            debug!("Rule {} signature verified", rule.id);
        } else {
            // Rule must be signed
            return Err(CorrelationError::RuleSignatureInvalid(
                format!("Rule {} is not signed", rule.id)
            ));
        }
        
        // Verify signature hash
        if let Some(ref expected_hash) = rule.signature_hash {
            let mut hasher = Sha256::new();
            hasher.update(&content);
            let computed_hash = hex::encode(hasher.finalize());
            
            if computed_hash != *expected_hash {
                return Err(CorrelationError::RuleSignatureInvalid(
                    format!("Rule {} hash mismatch", rule.id)
                ));
            }
        }
        
        Ok(rule)
    }
    
    /// Get rule by ID
    pub fn get_rule(&self, rule_id: &str) -> Result<&Rule, CorrelationError> {
        self.rules.get(rule_id)
            .ok_or_else(|| CorrelationError::RuleNotFound(rule_id.to_string()))
    }
    
    /// Get all rules
    pub fn get_all_rules(&self) -> &HashMap<String, Rule> {
        &self.rules
    }
    
    /// Check if rule exists
    pub fn has_rule(&self, rule_id: &str) -> bool {
        self.rules.contains_key(rule_id)
    }
    
    /// Reload rules (for testing)
    pub fn reload(&mut self) -> Result<(), CorrelationError> {
        self.rules.clear();
        self.load_rules()
    }
}

