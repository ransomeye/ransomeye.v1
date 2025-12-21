// Path and File Name : /home/ransomeye/rebuild/ransomeye_correlation/src/evidence.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Evidence bundling - creates cryptographically verifiable evidence bundles

/*
 * Evidence Bundle
 * 
 * Creates cryptographically verifiable evidence bundles.
 * Every alert includes complete evidence chain.
 * Evidence must be verifiable offline.
 */

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use tracing::{error, debug};

use crate::errors::CorrelationError;
use crate::security::evidence_hash::EvidenceHasher;
use crate::state::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceBundle {
    pub bundle_id: String,
    pub alert_id: String,
    pub created_at: DateTime<Utc>,
    pub engine_version: String,
    pub input_event_hashes: Vec<String>,
    pub rule_ids: Vec<String>,
    pub kill_chain_stage: String,
    pub correlation_window_start: DateTime<Utc>,
    pub correlation_window_end: DateTime<Utc>,
    pub events: Vec<serde_json::Value>,
    pub state_transitions: Vec<StateTransition>,
    pub evidence_hash: String,
    pub signature: Option<String>, // Base64 encoded signature
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    pub from: String,
    pub to: String,
    pub timestamp: DateTime<Utc>,
    pub trigger: String,
}

pub struct EvidenceBuilder {
    hasher: EvidenceHasher,
    engine_version: String,
}

impl EvidenceBuilder {
    pub fn new(engine_version: &str) -> Self {
        Self {
            hasher: EvidenceHasher::new(),
            engine_version: engine_version.to_string(),
        }
    }
    
    /// Build evidence bundle
    /// Returns EvidenceBundle on success, CorrelationError on failure
    pub fn build(
        &self,
        alert_id: &str,
        rule_ids: Vec<String>,
        kill_chain_stage: State,
        events: Vec<serde_json::Value>,
        state_transitions: Vec<StateTransition>,
        window_start: DateTime<Utc>,
        window_end: DateTime<Utc>,
    ) -> Result<EvidenceBundle, CorrelationError> {
        let bundle_id = Uuid::new_v4().to_string();
        
        // Hash all input events
        let input_event_hashes: Vec<String> = events.iter()
            .map(|e| self.hasher.hash_event(e))
            .collect();
        
        // Create evidence bundle
        let mut bundle = EvidenceBundle {
            bundle_id: bundle_id.clone(),
            alert_id: alert_id.to_string(),
            created_at: Utc::now(),
            engine_version: self.engine_version.clone(),
            input_event_hashes,
            rule_ids: rule_ids.clone(),
            kill_chain_stage: crate::kill_chain::KillChainInferencer::stage_to_string(&kill_chain_stage),
            correlation_window_start: window_start,
            correlation_window_end: window_end,
            events: events.clone(),
            state_transitions: state_transitions.clone(),
            evidence_hash: String::new(), // Will be computed
            signature: None,
        };
        
        // Compute evidence hash
        let evidence_json = serde_json::to_value(&bundle)
            .map_err(|e| CorrelationError::EvidenceBundleFailed(
                format!("Failed to serialize evidence: {}", e)
            ))?;
        
        bundle.evidence_hash = self.hasher.hash_evidence(&evidence_json);
        
        debug!("Created evidence bundle: {} (hash: {})", bundle_id, bundle.evidence_hash);
        
        Ok(bundle)
    }
    
    /// Sign evidence bundle (for production)
    pub fn sign_bundle(&self, bundle: &mut EvidenceBundle) -> Result<(), CorrelationError> {
        // In production, would sign with private key
        // For now, signature is optional
        debug!("Evidence bundle signing (would be performed in production)");
        Ok(())
    }
    
    /// Verify evidence bundle
    pub fn verify_bundle(&self, bundle: &EvidenceBundle) -> Result<bool, CorrelationError> {
        // Recompute hash
        let evidence_json = serde_json::to_value(bundle)
            .map_err(|e| CorrelationError::EvidenceBundleFailed(
                format!("Failed to serialize evidence: {}", e)
            ))?;
        
        let computed_hash = self.hasher.hash_evidence(&evidence_json);
        
        if computed_hash != bundle.evidence_hash {
            error!("Evidence bundle hash mismatch");
            return Ok(false);
        }
        
        // Verify event hashes
        for (i, event) in bundle.events.iter().enumerate() {
            let computed_event_hash = self.hasher.hash_event(event);
            if i < bundle.input_event_hashes.len() {
                if computed_event_hash != bundle.input_event_hashes[i] {
                    error!("Event hash mismatch at index {}", i);
                    return Ok(false);
                }
            }
        }
        
        debug!("Evidence bundle verification successful");
        Ok(true)
    }
}

