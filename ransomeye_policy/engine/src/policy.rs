// Path and File Name : /home/ransomeye/rebuild/ransomeye_policy/engine/src/policy.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Policy loading and validation - signed policies only with real cryptography

use std::path::Path;
use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use hex;
use tracing::{error, info, warn, debug};
use std::sync::Arc;

use crate::errors::PolicyError;
use crate::decision::AllowedAction;

#[path = "../../security/signature.rs"]
mod signature;
#[path = "../../security/verification.rs"]
mod verification;

use signature::PolicySignatureVerifier;
use verification::PolicyVerifier;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub version: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub priority: u32,
    pub match_conditions: Vec<PolicyMatchCondition>,
    pub decision: PolicyDecisionRule,
    pub required_approvals: Vec<String>,
    pub signature: Option<String>,
    pub signature_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDecisionRule {
    pub action: String,
    pub allowed_actions: Vec<String>,
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyMatchCondition {
    pub field: String,
    pub operator: String,
    pub value: serde_json::Value,
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
    hash_verifier: PolicyVerifier,
    policies_path: String,
}

impl PolicyLoader {
    pub fn new(policies_path: &str, trust_store_path: Option<&str>) -> Result<Self, PolicyError> {
        let signature_verifier = PolicySignatureVerifier::new()
            .map_err(|e| PolicyError::ConfigurationError(
                format!("Failed to initialize signature verifier: {}", e)
            ))?;

        if let Some(trust_path) = trust_store_path {
            PolicySignatureVerifier::load_trust_store(trust_path)
                .map_err(|e| PolicyError::ConfigurationError(
                    format!("Failed to load trust store: {}", e)
                ))?;
        }

        let mut loader = Self {
            policies: HashMap::new(),
            signature_verifier,
            hash_verifier: PolicyVerifier::new(),
            policies_path: policies_path.to_string(),
        };

        loader.load_policies()?;

        Ok(loader)
    }

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

        let mut policy: Policy = serde_yaml::from_str(&content)
            .map_err(|e| PolicyError::ConfigurationError(
                format!("Failed to parse policy file {}: {}", path.display(), e)
            ))?;

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
            return Err(PolicyError::UnsignedPolicy(
                format!("Policy {} is not signed", policy.id)
            ));
        }

        if let Some(ref expected_hash) = policy.signature_hash {
            if !self.hash_verifier.verify_hash(&content, expected_hash) {
                return Err(PolicyError::PolicyTampered(
                    format!("Policy {} hash mismatch", policy.id)
                ));
            }
        }

        Ok(policy)
    }

    pub fn get_policy(&self, policy_id: &str) -> Result<&Policy, PolicyError> {
        self.policies.get(policy_id)
            .ok_or_else(|| PolicyError::PolicyNotFound(policy_id.to_string()))
    }

    pub fn get_all_policies(&self) -> Vec<&Policy> {
        let mut policies: Vec<&Policy> = self.policies.values().collect();
        policies.sort_by(|a, b| b.priority.cmp(&a.priority));
        policies
    }

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

