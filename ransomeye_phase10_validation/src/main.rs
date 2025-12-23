// Path and File Name : /home/ransomeye/rebuild/ransomeye_phase10_validation/src/main.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Phase 10 validation main entry point

use std::path::PathBuf;
use tracing::{info, error};
use tracing_subscriber;

use ransomeye_phase10_validation::{Phase10Validator, ValidationError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    // Get output directory from environment or use default
    let output_dir = std::env::var("PHASE10_VALIDATION_OUTPUT_DIR")
        .map(|s| PathBuf::from(s))
        .unwrap_or_else(|_| PathBuf::from("./phase10_validation_reports"));
    
    info!("Phase 10 Validation Output Directory: {:?}", output_dir);
    
    // Create validator
    let validator = Phase10Validator::new(output_dir);
    
    // Run validation
    match validator.run_full_validation().await {
        Ok(result) => {
            match result.overall_decision {
                ransomeye_phase10_validation::reports::GoNoGoDecision::Go => {
                    info!("Phase 10 validation completed successfully - GO");
                    std::process::exit(0);
                }
                ransomeye_phase10_validation::reports::GoNoGoDecision::NoGo { reason, .. } => {
                    error!("Phase 10 validation failed - NO-GO: {}", reason);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            error!("Phase 10 validation error: {}", e);
            std::process::exit(1);
        }
    }
}

