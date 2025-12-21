// Path and File Name : /home/ransomeye/rebuild/ransomeye_policy/src/engine.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Policy engine - deterministic policy evaluation authority

/*
 * Policy Engine
 * 
 * The ONLY authority that can decide:
 * - What constitutes a violation
 * - What action is allowed
 * - What is forbidden
 * 
 * Unsigned policy → ENGINE REFUSES TO START
 * Ambiguity → DENY
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tracing::{error, info, warn, debug};

use crate::errors::PolicyError;
use crate::context::EvaluationContext;
use crate::decision::PolicyDecision;
use crate::policy::PolicyLoader;
use crate::evaluator::PolicyEvaluator;
use crate::security::revocation::PolicyRevocationChecker;

pub struct PolicyEngine {
    evaluator: Arc<PolicyEvaluator>,
    revocation_checker: Arc<PolicyRevocationChecker>,
    started: Arc<AtomicBool>,
    engine_version: String,
}

impl PolicyEngine {
    pub fn new(
        policies_path: &str,
        engine_version: &str,
    ) -> Result<Self, PolicyError> {
        info!("Initializing Policy Engine (version: {})", engine_version);
        
        // Load policies (will fail if unsigned)
        let policy_loader = Arc::new(PolicyLoader::new(policies_path)?);
        
        // Check for unsigned policies
        let policies = policy_loader.get_all_policies();
        for policy in policies {
            if policy.signature.is_none() {
                error!("Unsigned policy found: {}", policy.id);
                return Err(PolicyError::EngineRefusedToStart(
                    format!("Unsigned policy: {}", policy.id)
                ));
            }
        }
        
        // Initialize revocation checker
        let revocation_checker = Arc::new(PolicyRevocationChecker::new()?);
        
        // Create evaluator
        let evaluator = Arc::new(PolicyEvaluator::new(policy_loader.clone())?);
        
        info!("Policy Engine initialized successfully");
        
        Ok(Self {
            evaluator,
            revocation_checker,
            started: Arc::new(AtomicBool::new(true)),
            engine_version: engine_version.to_string(),
        })
    }
    
    /// Evaluate policy for alert
    /// Returns PolicyDecision on success, PolicyError on failure
    /// Ambiguity → DENY
    pub fn evaluate(&self, context: EvaluationContext) -> Result<PolicyDecision, PolicyError> {
        if !self.started.load(Ordering::Relaxed) {
            return Err(PolicyError::EngineRefusedToStart(
                "Engine is not started".to_string()
            ));
        }
        
        debug!("Evaluating policy for alert: {}", context.alert_id);
        
        // Check if any policies are revoked
        // (In production, would check against revocation list)
        
        // Evaluate policies
        match self.evaluator.evaluate(&context) {
            Ok(decision) => {
                // Verify decision integrity
                if !decision.verify() {
                    error!("Decision integrity verification failed");
                    return Err(PolicyError::EvaluationError(
                        "Decision integrity verification failed".to_string()
                    ));
                }
                
                info!("Policy decision: {} (action: {:?})", decision.decision_id, decision.decision);
                Ok(decision)
            }
            Err(PolicyError::PolicyAmbiguity(msg)) => {
                // Ambiguity → DENY
                error!("Policy ambiguity: {}", msg);
                Err(PolicyError::PolicyAmbiguity(msg))
            }
            Err(PolicyError::NoMatchingPolicy(_)) => {
                // No matching policy → DENY
                warn!("No matching policy, defaulting to DENY");
                self.create_deny_decision(&context)
            }
            Err(e) => {
                error!("Policy evaluation error: {}", e);
                Err(e)
            }
        }
    }
    
    /// Create default DENY decision
    fn create_deny_decision(&self, context: &EvaluationContext) -> Result<PolicyDecision, PolicyError> {
        use crate::decision::AllowedAction;
        
        let decision = PolicyDecision::new(
            &context.alert_id,
            "default_deny",
            "1.0.0",
            AllowedAction::Deny,
            vec![AllowedAction::Deny],
            Vec::new(),
            &context.evidence_reference,
            &context.kill_chain_stage,
            &context.alert_severity,
            context.asset_class.clone(),
            "No matching policy found, defaulting to DENY",
            "",
        );
        
        Ok(decision)
    }
    
    /// Check if engine is started
    pub fn is_started(&self) -> bool {
        self.started.load(Ordering::Relaxed)
    }
    
    /// Get engine version
    pub fn version(&self) -> &str {
        &self.engine_version
    }
}

