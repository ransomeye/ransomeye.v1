// Path and File Name : /home/ransomeye/rebuild/ransomeye_windows_agent/src/main.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Main entry point for Windows Agent - endpoint telemetry collection

/*
 * RansomEye Windows Agent
 * 
 * Stand-alone sensor for endpoint telemetry collection.
 * UNTRUSTED component - never enforces policy, never runs AI, never stores long-term state.
 * All telemetry is signed and sent to Control Plane via mTLS.
 */

use std::sync::Arc;
use tokio::signal;
use tracing::{info, error};

mod telemetry;
mod signing;
mod transport;
mod backpressure;
mod health;
mod config;
mod identity;

use telemetry::TelemetryCollector;
use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting RansomEye Windows Agent");
    
    // Load configuration
    let config = Config::load()?;
    info!("Configuration loaded");
    
    // Initialize identity
    let identity = Arc::new(identity::Identity::load_or_create(&config)?);
    info!("Identity initialized: {}", identity.producer_id());
    
    // Create telemetry collector
    let collector = Arc::new(TelemetryCollector::new(config.clone(), identity.clone()).await?);
    
    // Start telemetry collection
    let collector_handle = {
        let collector = collector.clone();
        tokio::spawn(async move {
            if let Err(e) = collector.start().await {
                error!("Telemetry collector error: {}", e);
                std::process::exit(1);
            }
        })
    };
    
    // Wait for shutdown signal
    signal::ctrl_c().await?;
    info!("Shutdown signal received");
    
    // Graceful shutdown
    collector.shutdown().await;
    collector_handle.await?;
    
    info!("RansomEye Windows Agent stopped");
    Ok(())
}

