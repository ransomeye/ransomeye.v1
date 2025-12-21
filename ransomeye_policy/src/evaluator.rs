// Path and File Name : /home/ransomeye/rebuild/ransomeye_policy/src/evaluator.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Policy evaluator - deterministic policy evaluation

/*
 * Policy Evaluator
 * 
 * Evaluates policies against context deterministically.
 * Ambiguity → DENY
 * Missing context → DENY
 */

use std::sync::Arc;
use tracing::{error, debug, warn};

use crate::errors::PolicyError;
use crate::context::EvaluationContext;
use crate::policy::{PolicyLoader, PolicyRule};
use crate::matcher::PolicyMatcher;
use crate::decision::{PolicyDecision, AllowedAction};

pub struct PolicyEvaluator {
    policy_loader: Arc<PolicyLoader>,
    matcher: Arc<PolicyMatcher>,
}

impl PolicyEvaluator {
    pub fn new(policy_loader: Arc<PolicyLoader>) -> Result<Self, PolicyError> {
        Ok(Self {
            policy_loader,
            matcher: Arc::new(PolicyMatcher::new()),
        })
    }
    
    /// Evaluate policies against context
    /// Returns PolicyDecision on success, PolicyError on failure
    /// Ambiguity → DENY
    pub fn evaluate(&self, context: &EvaluationContext) -> Result<PolicyDecision, PolicyError> {
        // Validate context
        context.validate()?;
        
        debug!("Evaluating policies for alert: {}", context.alert_id);
        
        // Get all policies (sorted by priority)
        let policies = self.policy_loader.get_all_policies();
        
        // Find matching policies
        let mut matching_policies = Vec::new();
        
        for policy in policies {
            if !policy.enabled {
                continue;
            }
            
            let policy_rule = self.policy_loader.to_policy_rule(policy)?;
            
            match self.matcher.matches(&policy_rule, context) {
                Ok(true) => {
                    matching_policies.push((policy_rule, policy.signature.clone().unwrap_or_default()));
                    debug!("Policy {} matches context", policy.id);
                }
                Ok(false) => {
                    // Policy doesn't match, continue
                }
                Err(e) => {
                    warn!("Error matching policy {}: {}", policy.id, e);
                    // Continue evaluation
                }
            }
        }
        
        // Handle matching policies
        if matching_policies.is_empty() {
            // No matching policy → DENY
            warn!("No matching policy found for alert: {}", context.alert_id);
            return Err(PolicyError::NoMatchingPolicy(
                format!("No matching policy for alert: {}", context.alert_id)
            ));
        }
        
        if matching_policies.len() > 1 {
            // Multiple policies match → check for ambiguity
            let highest_priority = matching_policies[0].0.priority;
            let same_priority: Vec<_> = matching_policies.iter()
                .filter(|(p, _)| p.priority == highest_priority)
                .collect();
            
            if same_priority.len() > 1 {
                // Ambiguity → DENY
                error!("Policy ambiguity: {} policies match with same priority", same_priority.len());
                return Err(PolicyError::PolicyAmbiguity(
                    format!("Multiple policies match with same priority: {}", same_priority.len())
                ));
            }
        }
        
        // Use highest priority matching policy
        let (policy_rule, policy_signature) = matching_policies.remove(0);
        
        // Build decision
        let decision = PolicyDecision::new(
            &context.alert_id,
            &policy_rule.id,
            &policy_rule.version,
            policy_rule.decision.clone(),
            policy_rule.allowed_actions.clone(),
            policy_rule.required_approvals.clone(),
            &context.evidence_reference,
            &context.kill_chain_stage,
            &context.alert_severity,
            context.asset_class.clone(),
            &policy_rule.reasoning,
            &policy_signature,
        );
        
        debug!("Policy decision created: {} (action: {:?})", decision.decision_id, decision.decision);
        
        Ok(decision)
    }
}

