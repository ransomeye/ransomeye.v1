// Path and File Name : /home/ransomeye/rebuild/ransomeye_policy/src/policy.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Policy loading and validation - signed policies only

/*
 * Policy Loading
 * 
 * Loads and validates signed policies.
 * Unsigned policy → ENGINE REFUSES TO START
 * Policies must be deterministic and replayable.
 */

use std::path::Path;
use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use tracing::{error, info, warn, debug};

use crate::errors::PolicyError;
use crate::security::signature::PolicySignatureVerifier;
use crate::decision::AllowedAction;
use crate::matcher::PolicyMatchCondition;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub version: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub priority: u32, // Higher priority = evaluated first
    pub match_conditions: Vec<PolicyMatchCondition>,
    pub decision: PolicyDecisionRule,
    pub required_approvals: Vec<String>,
    pub signature: Option<String>, // Base64 encoded signature
    pub signature_hash: Option<String>, // SHA-256 hash of policy content
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDecisionRule {
    pub action: String, // "allow", "deny", "quarantine", "isolate", "block", "monitor", "escalate", "require_approval"
    pub allowed_actions: Vec<String>,
    pub reasoning: String, // Deterministic reasoning
}

#[derive(Debug, Clone)]
pub struct PolicyRule {
    pub id: String,
    pub version: String,
    pub priority: u32,
    pub match_conditions: Vec<PolicyMatchCondition>,
    pub decision: AllowedAction,
    pub allowed_actions: Vec<AllowedAction>,
    pub required_approvals: Vec<String>,
    pub reasoning: String,
}

pub struct PolicyLoader {
    policies: HashMap<String, Policy>,
    signature_verifier: PolicySignatureVerifier,
    policies_path: String,
}

impl PolicyLoader {
    pub fn new(policies_path: &str) -> Result<Self, PolicyError> {
        let signature_verifier = PolicySignatureVerifier::new()
            .map_err(|e| PolicyError::ConfigurationError(
                format!("Failed to initialize signature verifier: {}", e)
            ))?;
        
        let mut loader = Self {
            policies: HashMap::new(),
            signature_verifier,
            policies_path: policies_path.to_string(),
        };
        
        loader.load_policies()?;
        
        Ok(loader)
    }
    
    /// Load policies from directory
    /// Unsigned policy → ENGINE REFUSES TO START
    pub fn load_policies(&mut self) -> Result<(), PolicyError> {
        info!("Loading policies from: {}", self.policies_path);
        
        let policies_dir = Path::new(&self.policies_path);
        if !policies_dir.exists() {
            return Err(PolicyError::ConfigurationError(
                format!("Policies directory not found: {}", self.policies_path)
            ));
        }
        
        let entries = fs::read_dir(policies_dir)
            .map_err(|e| PolicyError::ConfigurationError(
                format!("Failed to read policies directory: {}", e)
            ))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| PolicyError::ConfigurationError(
                format!("Failed to read directory entry: {}", e)
            ))?;
            
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") || 
               path.extension().and_then(|s| s.to_str()) == Some("yml") {
                match self.load_policy_file(&path) {
                    Ok(policy) => {
                        info!("Loaded policy: {} (version: {})", policy.id, policy.version);
                        self.policies.insert(policy.id.clone(), policy);
                    }
                    Err(e) => {
                        error!("Failed to load policy from {}: {}", path.display(), e);
                        return Err(e);
                    }
                }
            }
        }
        
        if self.policies.is_empty() {
            return Err(PolicyError::ConfigurationError(
                "No valid policies loaded".to_string()
            ));
        }
        
        info!("Loaded {} policies", self.policies.len());
        Ok(())
    }
    
    fn load_policy_file(&self, path: &Path) -> Result<Policy, PolicyError> {
        let content = fs::read_to_string(path)
            .map_err(|e| PolicyError::ConfigurationError(
                format!("Failed to read policy file {}: {}", path.display(), e)
            ))?;
        
        // Parse YAML
        let mut policy: Policy = serde_yaml::from_str(&content)
            .map_err(|e| PolicyError::ConfigurationError(
                format!("Failed to parse policy file {}: {}", path.display(), e)
            ))?;
        
        // Verify signature if present
        if let Some(ref signature) = policy.signature {
            let verified = self.signature_verifier.verify(&content, signature)
                .map_err(|e| PolicyError::PolicySignatureInvalid(
                    format!("Policy {} signature verification failed: {}", policy.id, e)
                ))?;
            
            if !verified {
                return Err(PolicyError::PolicySignatureInvalid(
                    format!("Policy {} has invalid signature", policy.id)
                ));
            }
            
            debug!("Policy {} signature verified", policy.id);
        } else {
            // Policy MUST be signed
            return Err(PolicyError::UnsignedPolicy(
                format!("Policy {} is not signed", policy.id)
            ));
        }
        
        // Verify signature hash
        if let Some(ref expected_hash) = policy.signature_hash {
            let mut hasher = Sha256::new();
            hasher.update(&content);
            let computed_hash = hex::encode(hasher.finalize());
            
            if computed_hash != *expected_hash {
                return Err(PolicyError::PolicyTampered(
                    format!("Policy {} hash mismatch", policy.id)
                ));
            }
        }
        
        Ok(policy)
    }
    
    /// Get policy by ID
    pub fn get_policy(&self, policy_id: &str) -> Result<&Policy, PolicyError> {
        self.policies.get(policy_id)
            .ok_or_else(|| PolicyError::PolicyNotFound(policy_id.to_string()))
    }
    
    /// Get all policies (sorted by priority)
    pub fn get_all_policies(&self) -> Vec<&Policy> {
        let mut policies: Vec<&Policy> = self.policies.values().collect();
        policies.sort_by(|a, b| b.priority.cmp(&a.priority)); // Higher priority first
        policies
    }
    
    /// Convert Policy to PolicyRule
    pub fn to_policy_rule(&self, policy: &Policy) -> Result<PolicyRule, PolicyError> {
        let decision = self.parse_action(&policy.decision.action)?;
        let allowed_actions: Result<Vec<AllowedAction>, _> = policy.decision.allowed_actions
            .iter()
            .map(|a| self.parse_action(a))
            .collect();
        
        Ok(PolicyRule {
            id: policy.id.clone(),
            version: policy.version.clone(),
            priority: policy.priority,
            match_conditions: policy.match_conditions.clone(),
            decision,
            allowed_actions: allowed_actions?,
            required_approvals: policy.required_approvals.clone(),
            reasoning: policy.decision.reasoning.clone(),
        })
    }
    
    fn parse_action(&self, action: &str) -> Result<AllowedAction, PolicyError> {
        match action.to_lowercase().as_str() {
            "allow" => Ok(AllowedAction::Allow),
            "deny" => Ok(AllowedAction::Deny),
            "quarantine" => Ok(AllowedAction::Quarantine),
            "isolate" => Ok(AllowedAction::Isolate),
            "block" => Ok(AllowedAction::Block),
            "monitor" => Ok(AllowedAction::Monitor),
            "escalate" => Ok(AllowedAction::Escalate),
            "require_approval" => Ok(AllowedAction::RequireApproval),
            _ => Err(PolicyError::EvaluationError(
                format!("Unknown action: {}", action)
            )),
        }
    }
}

