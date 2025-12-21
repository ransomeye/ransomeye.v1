// Path and File Name : /home/ransomeye/rebuild/ransomeye_correlation/src/kill_chain.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Kill-chain stage inference - deterministic kill-chain mapping

/*
 * Kill-Chain Inference
 * 
 * Maps detections to kill-chain stages deterministically.
 * Never skip stages.
 * Never regress stages.
 * Produces evidence per stage.
 */

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use crate::state::State;
use crate::errors::CorrelationError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KillChainMapping {
    pub event_type: String,
    pub stage: String,
    pub indicators: Vec<String>,
    pub confidence: String, // "high", "medium", "low"
}

#[derive(Debug, Clone)]
pub struct KillChainInferencer {
    mappings: HashMap<String, State>,
    event_type_to_stage: HashMap<String, State>,
}

impl KillChainInferencer {
    pub fn new() -> Result<Self, CorrelationError> {
        let mut inferencer = Self {
            mappings: HashMap::new(),
            event_type_to_stage: HashMap::new(),
        };
        
        // Initialize default mappings
        inferencer.initialize_default_mappings();
        
        Ok(inferencer)
    }
    
    fn initialize_default_mappings(&mut self) {
        // Reconnaissance
        self.event_type_to_stage.insert("port_scan".to_string(), State::Reconnaissance);
        self.event_type_to_stage.insert("network_scan".to_string(), State::Reconnaissance);
        self.event_type_to_stage.insert("dns_enumeration".to_string(), State::Reconnaissance);
        
        // Weaponization
        self.event_type_to_stage.insert("malware_download".to_string(), State::Weaponization);
        self.event_type_to_stage.insert("payload_creation".to_string(), State::Weaponization);
        
        // Delivery
        self.event_type_to_stage.insert("email_attachment".to_string(), State::Delivery);
        self.event_type_to_stage.insert("web_download".to_string(), State::Delivery);
        self.event_type_to_stage.insert("usb_insertion".to_string(), State::Delivery);
        
        // Exploitation
        self.event_type_to_stage.insert("exploit_execution".to_string(), State::Exploitation);
        self.event_type_to_stage.insert("code_injection".to_string(), State::Exploitation);
        self.event_type_to_stage.insert("buffer_overflow".to_string(), State::Exploitation);
        
        // Installation
        self.event_type_to_stage.insert("persistence_mechanism".to_string(), State::Installation);
        self.event_type_to_stage.insert("backdoor_installation".to_string(), State::Installation);
        self.event_type_to_stage.insert("service_creation".to_string(), State::Installation);
        
        // Command & Control
        self.event_type_to_stage.insert("c2_communication".to_string(), State::CommandControl);
        self.event_type_to_stage.insert("beacon".to_string(), State::CommandControl);
        self.event_type_to_stage.insert("data_exfiltration".to_string(), State::CommandControl);
        
        // Actions on Objectives
        self.event_type_to_stage.insert("ransomware_execution".to_string(), State::ActionsOnObjectives);
        self.event_type_to_stage.insert("data_encryption".to_string(), State::ActionsOnObjectives);
        self.event_type_to_stage.insert("file_deletion".to_string(), State::ActionsOnObjectives);
    }
    
    /// Infer kill-chain stage from event
    /// Returns State on success, None if stage cannot be determined
    pub fn infer_stage(&self, event_type: &str, event_data: &serde_json::Value) -> Option<State> {
        // First, try direct event type mapping
        if let Some(stage) = self.event_type_to_stage.get(event_type) {
            debug!("Inferred stage {:?} from event type: {}", stage, event_type);
            return Some(stage.clone());
        }
        
        // Try to extract event type from event data
        if let Some(evt_type) = event_data.get("event_type").and_then(|v| v.as_str()) {
            if let Some(stage) = self.event_type_to_stage.get(evt_type) {
                debug!("Inferred stage {:?} from event data type: {}", stage, evt_type);
                return Some(stage.clone());
            }
        }
        
        // Try to infer from event data patterns
        if let Some(stage) = self.infer_from_patterns(event_data) {
            debug!("Inferred stage {:?} from event patterns", stage);
            return Some(stage);
        }
        
        warn!("Could not infer kill-chain stage for event type: {}", event_type);
        None
    }
    
    fn infer_from_patterns(&self, event_data: &serde_json::Value) -> Option<State> {
        // Check for reconnaissance patterns
        if let Some(ports) = event_data.get("ports_scanned") {
            if ports.as_array().map(|a| a.len()).unwrap_or(0) > 10 {
                return Some(State::Reconnaissance);
            }
        }
        
        // Check for exploitation patterns
        if event_data.get("exploit_technique").is_some() {
            return Some(State::Exploitation);
        }
        
        // Check for C2 patterns
        if event_data.get("c2_domain").is_some() || event_data.get("beacon_interval").is_some() {
            return Some(State::CommandControl);
        }
        
        // Check for ransomware patterns
        if event_data.get("encryption_algorithm").is_some() || 
           event_data.get("ransom_note").is_some() {
            return Some(State::ActionsOnObjectives);
        }
        
        None
    }
    
    /// Get stage name as string
    pub fn stage_to_string(stage: &State) -> String {
        match stage {
            State::Initial => "initial".to_string(),
            State::Reconnaissance => "reconnaissance".to_string(),
            State::Weaponization => "weaponization".to_string(),
            State::Delivery => "delivery".to_string(),
            State::Exploitation => "exploitation".to_string(),
            State::Installation => "installation".to_string(),
            State::CommandControl => "command_control".to_string(),
            State::ActionsOnObjectives => "actions_on_objectives".to_string(),
            State::Alerted => "alerted".to_string(),
        }
    }
    
    /// Load mappings from file (for extensibility)
    pub fn load_mappings(&mut self, mappings: Vec<KillChainMapping>) -> Result<(), CorrelationError> {
        for mapping in mappings {
            let stage = self.string_to_stage(&mapping.stage)
                .ok_or_else(|| CorrelationError::ConfigurationError(
                    format!("Invalid kill-chain stage: {}", mapping.stage)
                ))?;
            
            self.event_type_to_stage.insert(mapping.event_type.clone(), stage);
            debug!("Loaded kill-chain mapping: {} -> {}", mapping.event_type, mapping.stage);
        }
        
        Ok(())
    }
    
    fn string_to_stage(&self, stage_str: &str) -> Option<State> {
        match stage_str {
            "initial" => Some(State::Initial),
            "reconnaissance" => Some(State::Reconnaissance),
            "weaponization" => Some(State::Weaponization),
            "delivery" => Some(State::Delivery),
            "exploitation" => Some(State::Exploitation),
            "installation" => Some(State::Installation),
            "command_control" => Some(State::CommandControl),
            "actions_on_objectives" => Some(State::ActionsOnObjectives),
            "alerted" => Some(State::Alerted),
            _ => None,
        }
    }
}

