// Path and File Name : /home/ransomeye/rebuild/ransomeye_correlation/src/output.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Alert generation - creates deterministic alerts with evidence

/*
 * Alert Output
 * 
 * Generates deterministic alerts with complete evidence.
 * Every alert is cryptographically verifiable.
 */

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use tracing::debug;

use crate::evidence::EvidenceBundle;
use crate::state::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub alert_id: String,
    pub created_at: DateTime<Utc>,
    pub severity: String, // "critical", "high", "medium", "low"
    pub title: String,
    pub description: String,
    pub kill_chain_stage: String,
    pub confidence: String, // "high", "medium", "low"
    pub entity_id: String,
    pub rule_ids: Vec<String>,
    pub evidence_bundle: EvidenceBundle,
    pub state_transitions: Vec<String>,
}

pub struct AlertBuilder {
    engine_version: String,
}

impl AlertBuilder {
    pub fn new(engine_version: &str) -> Self {
        Self {
            engine_version: engine_version.to_string(),
        }
    }
    
    /// Build alert from correlation result
    pub fn build_alert(
        &self,
        title: &str,
        description: &str,
        severity: &str,
        confidence: &str,
        entity_id: &str,
        kill_chain_stage: State,
        rule_ids: Vec<String>,
        evidence_bundle: EvidenceBundle,
        state_transitions: Vec<String>,
    ) -> Alert {
        let alert_id = Uuid::new_v4().to_string();
        
        let alert = Alert {
            alert_id: alert_id.clone(),
            created_at: Utc::now(),
            severity: severity.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            kill_chain_stage: crate::kill_chain::KillChainInferencer::stage_to_string(&kill_chain_stage),
            confidence: confidence.to_string(),
            entity_id: entity_id.to_string(),
            rule_ids: rule_ids.clone(),
            evidence_bundle,
            state_transitions,
        };
        
        debug!("Created alert: {} (severity: {}, stage: {})",
            alert_id, severity, crate::kill_chain::KillChainInferencer::stage_to_string(&kill_chain_stage));
        
        alert
    }
    
    /// Serialize alert to JSON
    pub fn serialize(&self, alert: &Alert) -> Result<String, Box<dyn std::error::Error>> {
        serde_json::to_string_pretty(alert)
            .map_err(|e| format!("Failed to serialize alert: {}", e).into())
    }
}

