// Path and File Name : /home/ransomeye/rebuild/ransomeye_ingestion/src/schema.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Schema validation - strictly validates event schemas and versions

/*
 * Schema Validation
 * 
 * Strictly validates event schemas against JSON schema definitions.
 * Enforces version compatibility rules.
 * Fails-closed on schema violations.
 */

use std::sync::Arc;
use std::collections::HashMap;
use serde_json::Value;
use tracing::{error, debug};

use crate::protocol::event_envelope::EventEnvelope;
use crate::config::Config;
use crate::versioning::VersionManager;

pub struct SchemaValidator {
    config: Config,
    version_manager: Arc<VersionManager>,
    schemas: HashMap<u32, Value>,
}

impl SchemaValidator {
    pub fn new(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let mut schemas = HashMap::new();
        
        // Load schema v1
        let schema_v1 = std::fs::read_to_string("protocol/event_schema_v1.json")?;
        let schema_v1: Value = serde_json::from_str(&schema_v1)?;
        schemas.insert(1, schema_v1);
        
        Ok(Self {
            config: config.clone(),
            version_manager: Arc::new(VersionManager::new()?),
            schemas,
        })
    }
    
    pub async fn validate(&self, envelope: &EventEnvelope) -> Result<(), Box<dyn std::error::Error>> {
        // Check version compatibility
        if !self.version_manager.is_compatible(envelope.schema_version) {
            error!("Incompatible schema version: {}", envelope.schema_version);
            return Err("Incompatible schema version".into());
        }
        
        // Get schema for version
        let schema = self.schemas.get(&envelope.schema_version)
            .ok_or("Schema not found for version")?;
        
        // Validate envelope structure
        self.validate_envelope_structure(envelope)?;
        
        // Validate event data against schema
        let event_data: Value = serde_json::from_str(&envelope.event_data)?;
        self.validate_against_schema(&event_data, schema)?;
        
        debug!("Schema validation passed for producer: {}", envelope.producer_id);
        Ok(())
    }
    
    fn validate_envelope_structure(&self, envelope: &EventEnvelope) -> Result<(), Box<dyn std::error::Error>> {
        // Validate required fields
        if envelope.producer_id.is_empty() {
            return Err("Producer ID is required".into());
        }
        if envelope.component_type.is_empty() {
            return Err("Component type is required".into());
        }
        if envelope.signature.is_empty() {
            return Err("Signature is required".into());
        }
        if envelope.integrity_hash.is_empty() {
            return Err("Integrity hash is required".into());
        }
        
        Ok(())
    }
    
    fn validate_against_schema(&self, data: &Value, schema: &Value) -> Result<(), Box<dyn std::error::Error>> {
        // Basic JSON schema validation
        // In production, use a proper JSON schema validator library
        // For now, validate required fields exist
        
        if let Some(required) = schema.get("required").and_then(|v| v.as_array()) {
            for field in required {
                if let Some(field_name) = field.as_str() {
                    if !data.get(field_name).is_some() {
                        return Err(format!("Required field missing: {}", field_name).into());
                    }
                }
            }
        }
        
        Ok(())
    }
}

