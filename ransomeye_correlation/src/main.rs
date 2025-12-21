// Path and File Name : /home/ransomeye/rebuild/ransomeye_correlation/src/main.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Main entry point for correlation engine

/*
 * RansomEye Correlation Engine
 * 
 * Deterministic correlation engine for RansomEye.
 * Processes events and generates alerts based on rule-based correlation.
 */

use std::env;
use tracing::{info, error};

mod engine;
mod pipeline;
mod correlator;
mod rules;
mod state;
mod window;
mod kill_chain;
mod evidence;
mod ordering;
mod output;
mod errors;
mod security;

use engine::CorrelationEngine;

const ENGINE_VERSION: &str = "1.0.0";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting RansomEye Correlation Engine (version: {})", ENGINE_VERSION);
    
    // Get configuration from environment
    let rules_path = env::var("RANSOMEYE_CORRELATION_RULES_PATH")
        .unwrap_or_else(|_| "/etc/ransomeye/correlation/rules".to_string());
    
    let window_size_seconds = env::var("RANSOMEYE_CORRELATION_WINDOW_SECONDS")
        .unwrap_or_else(|_| "300".to_string())
        .parse::<i64>()?;
    
    let max_events = env::var("RANSOMEYE_CORRELATION_MAX_EVENTS")
        .unwrap_or_else(|_| "1000".to_string())
        .parse::<usize>()?;
    
    // Initialize engine
    let engine = CorrelationEngine::new(
        &rules_path,
        window_size_seconds,
        max_events,
        ENGINE_VERSION,
    )?;
    
    info!("Correlation Engine initialized successfully");
    info!("Rules path: {}", rules_path);
    info!("Window size: {} seconds", window_size_seconds);
    info!("Max events per window: {}", max_events);
    
    // Engine is ready
    // In production, would connect to event stream and process events
    info!("Correlation Engine ready");
    
    // Keep running (in production, would process events from queue)
    tokio::signal::ctrl_c().await?;
    
    info!("Shutting down Correlation Engine");
    Ok(())
}

