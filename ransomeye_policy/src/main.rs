// Path and File Name : /home/ransomeye/rebuild/ransomeye_policy/src/main.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Main entry point for policy engine

/*
 * RansomEye Policy Engine
 * 
 * Deterministic policy evaluation engine for RansomEye.
 * Evaluates alerts against signed policies and emits enforcement decisions.
 */

use std::env;
use tracing::{info, error};

mod engine;
mod evaluator;
mod policy;
mod decision;
mod context;
mod matcher;
mod output;
mod errors;
mod security;

use engine::PolicyEngine;

const ENGINE_VERSION: &str = "1.0.0";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting RansomEye Policy Engine (version: {})", ENGINE_VERSION);
    
    // Get configuration from environment
    let policies_path = env::var("RANSOMEYE_POLICY_PATH")
        .unwrap_or_else(|_| "/etc/ransomeye/policy/policies".to_string());
    
    // Initialize engine
    let engine = PolicyEngine::new(
        &policies_path,
        ENGINE_VERSION,
    )?;
    
    info!("Policy Engine initialized successfully");
    info!("Policies path: {}", policies_path);
    
    // Engine is ready
    // In production, would receive alerts and evaluate policies
    info!("Policy Engine ready");
    
    // Keep running (in production, would process alerts from queue)
    tokio::signal::ctrl_c().await?;
    
    info!("Shutting down Policy Engine");
    Ok(())
}

