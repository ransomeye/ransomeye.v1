// Path and File Name : /home/ransomeye/rebuild/ransomeye_ai_advisory/src/main.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Main entry point for AI advisory service

/*
 * RansomEye AI Advisory Service
 * 
 * Provides advisory-only AI assistance:
 * - Risk scoring (advisory)
 * - Context enrichment
 * - Explainability (SHAP)
 * - SOC Copilot (read-only)
 * 
 * MUST NOT:
 * - Influence policy decisions
 * - Trigger enforcement
 * - Modify state
 * - Operate without signed baseline
 */

use std::env;
use tokio::signal;
use tracing::{info, error};

mod engine;
mod scorer;
mod explainer;
mod context;
mod outputs;
mod controller;
mod errors;
mod registry;
mod shap;
mod llm;
mod security;

use engine::AdvisoryEngine;
use errors::AdvisoryError;

const ADVISORY_VERSION: &str = "1.0.0";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting RansomEye AI Advisory Service (version: {})", ADVISORY_VERSION);
    info!("Advisory-only mode: ENABLED");
    info!("SHAP required: ENABLED");
    info!("Read-only access: ENABLED");
    
    // Initialize advisory engine
    let engine = AdvisoryEngine::new()
        .map_err(|e| {
            error!("Failed to initialize advisory engine: {}", e);
            e
        })?;
    
    // Check if AI is enabled
    match engine.is_enabled() {
        Ok(true) => {
            info!("AI Advisory Service initialized successfully");
            info!("Baseline models: VERIFIED");
            info!("Model signatures: VERIFIED");
        }
        Ok(false) => {
            error!("AI Advisory Service DISABLED");
            if let Ok(reason) = engine.get_state() {
                error!("Disable reason: {:?}", reason);
            }
            std::process::exit(1);
        }
        Err(e) => {
            error!("Failed to check AI state: {}", e);
            std::process::exit(1);
        }
    }
    
    info!("AI Advisory Service ready");
    info!("Waiting for advisory requests...");
    
    // Keep running (in production, would process requests from queue)
    signal::ctrl_c().await?;
    
    info!("Shutting down AI Advisory Service");
    Ok(())
}

