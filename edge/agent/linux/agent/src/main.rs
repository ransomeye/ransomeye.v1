// Path and File Name : /home/ransomeye/rebuild/ransomeye_linux_agent/agent/src/main.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Linux Agent main entry point - standalone host telemetry sensor

use std::sync::Arc;
use tracing::{info, error};
use std::time::{SystemTime, UNIX_EPOCH};

mod errors;
mod process;
mod filesystem;
mod network;
mod syscalls;
mod features;
mod envelope;
mod backpressure;
mod rate_limit;
mod health;
mod hardening;

#[path = "../security/mod.rs"]
mod security;

#[path = "../../config/validation.rs"]
mod config_validation;

use errors::AgentError;
use process::ProcessMonitor;
use filesystem::FilesystemMonitor;
use network::NetworkMonitor;
use syscalls::SyscallMonitor;
use features::FeatureExtractor;
use envelope::EnvelopeBuilder;
use backpressure::BackpressureManager;
use rate_limit::RateLimiter;
use health::HealthMonitor;
use hardening::RuntimeHardening;
use security::{IdentityManager, EventSigner};
use config_validation::AgentConfig;

fn main() -> Result<(), AgentError> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("RansomEye Linux Agent starting...");
    
    // Get binary path for integrity verification
    let binary_path = std::env::current_exe()
        .map_err(|e| AgentError::ConfigurationError(format!("Failed to get binary path: {}", e)))?
        .to_string_lossy()
        .to_string();
    
    // Initialize runtime hardening (FAIL-CLOSED on integrity failure)
    let config_path = std::env::var("AGENT_CONFIG_PATH").ok();
    let hardening = hardening::RuntimeHardening::new(
        binary_path.clone(),
        config_path.clone(),
        30, // 30 second watchdog interval
    ).map_err(|e| AgentError::ConfigurationError(format!("Hardening initialization failed: {}", e)))?;
    
    // Verify binary integrity at startup (FAIL-CLOSED)
    hardening.verify_binary_integrity()
        .map_err(|e| AgentError::ConfigurationError(format!("Binary integrity check failed: {}", e)))?;
    
    // Verify config integrity at startup (FAIL-CLOSED)
    hardening.verify_config_integrity()
        .map_err(|e| AgentError::ConfigurationError(format!("Config integrity check failed: {}", e)))?;
    
    // Perform runtime tamper checks (FAIL-CLOSED)
    hardening.perform_runtime_checks()
        .map_err(|e| AgentError::ConfigurationError(format!("Runtime check failed: {}", e)))?;
    
    // Start watchdog timer
    hardening.start_watchdog()
        .map_err(|e| AgentError::ConfigurationError(format!("Watchdog start failed: {}", e)))?;
    
    // Load configuration (ENV-only, fail-closed)
    let config = AgentConfig::from_env()
        .map_err(|e| AgentError::ConfigurationError(e))?;
    
    config.validate()
        .map_err(|e| AgentError::ConfigurationError(e))?;
    
    info!("Configuration loaded: max_processes={}, max_connections={}", 
        config.max_processes, config.max_connections);
    
    // Initialize identity (fail-closed on failure)
    let identity_path = config.identity_path.as_ref().map(|p| std::path::Path::new(p));
    let identity = IdentityManager::load_or_create(identity_path)
        .map_err(|e| AgentError::IdentityVerificationFailed(format!("{}", e)))?;
    
    info!("Component identity: {}", identity.component_id());
    
    // Initialize event signer (fail-closed on failure)
    let signer = if let Some(ref key_path) = config.signing_key_path {
        EventSigner::from_key_file(std::path::Path::new(key_path))
            .map_err(|e| AgentError::SigningFailed(format!("{}", e)))?
    } else {
        EventSigner::new()
            .map_err(|e| AgentError::SigningFailed(format!("{}", e)))?
    };
    
    info!("Event signer initialized");
    
    // Initialize components
    let process_monitor = Arc::new(ProcessMonitor::new(config.max_processes));
    let fs_monitor = Arc::new(FilesystemMonitor::new(config.mass_write_threshold));
    let network_monitor = Arc::new(NetworkMonitor::new(config.max_connections));
    let syscall_monitor = Arc::new(SyscallMonitor::new());
    let feature_extractor = Arc::new(FeatureExtractor::new());
    let mut envelope_builder = EnvelopeBuilder::new(
        "linux_agent".to_string(),
        identity.component_id().to_string(),
    );
    let backpressure = Arc::new(BackpressureManager::new(config.max_queue_size));
    let rate_limiter = Arc::new(RateLimiter::new(config.rate_limit_tokens, config.rate_limit_refill));
    let health_monitor = Arc::new(HealthMonitor::new(300)); // 5 minute max idle
    
    // Initialize syscall monitoring
    if config.enable_ebpf {
        if let Err(e) = syscall_monitor.init_ebpf() {
            error!("eBPF initialization failed: {}", e);
            if config.enable_auditd {
                info!("Falling back to auditd");
                syscall_monitor.init_auditd()?;
            } else {
                return Err(e);
            }
        } else {
            info!("eBPF syscall monitoring initialized");
        }
    } else if config.enable_auditd {
        syscall_monitor.init_auditd()?;
        info!("auditd syscall monitoring initialized");
    }
    
    // Start monitoring
    syscall_monitor.start()?;
    
    info!("Linux Agent started successfully");
    
    // Main processing loop
    let mut event_count = 0u64;
    loop {
        // Record watchdog heartbeat
        hardening.heartbeat();
        
        // Perform periodic runtime checks (every 1000 events)
        if event_count % 1000 == 0 {
            if let Err(e) = hardening.perform_runtime_checks() {
                error!("Runtime check failed: {}, stopping", e);
                hardening.stop_watchdog();
                return Err(AgentError::ConfigurationError(format!("Runtime hardening violation: {}", e)));
            }
            
            // Check for tamper detection
            if hardening.is_tampered() {
                error!("Tamper detected, stopping immediately");
                hardening.stop_watchdog();
                return Err(AgentError::ConfigurationError("Tamper detected - fail-closed".to_string()));
            }
        }
        
        // Check health
        if !health_monitor.check_health()? {
            error!("Health check failed, stopping");
            hardening.stop_watchdog();
            break;
        }
        
        // Check backpressure
        let queue_size = 0; // Would be actual queue size in production
        backpressure.update_queue_size(queue_size);
        
        if backpressure.should_drop(queue_size) {
            backpressure.signal();
            continue;
        }
        
        // Check rate limit
        if !rate_limiter.allow()? {
            continue;
        }
        
        // In production, would:
        // 1. Read syscall events from eBPF/auditd
        // 2. Parse events into process/filesystem/network events
        // 3. Extract features
        // 4. Create and sign envelopes
        // 5. Emit to Phase 4 pipeline
        
        // For demonstration, simulate an event
        if event_count % 1000 == 0 {
            // Simulate process exec event
            let process_event = process_monitor.record_exec(
                1234,
                Some(1000),
                1000,
                1000,
                "/usr/bin/test".to_string(),
                Some("test --arg".to_string()),
            )?;
            
            let features = feature_extractor.extract_from_process(&process_event)?;
            
            let envelope_data = serde_json::to_vec(&process_event)
                .map_err(|e| AgentError::EnvelopeCreationFailed(format!("{}", e)))?;
            
            let signature = signer.sign(&envelope_data)
                .map_err(|e| AgentError::SigningFailed(format!("{}", e)))?;
            
            let envelope = envelope_builder.build_from_process(&process_event, &features, signature)?;
            
            health_monitor.record_event();
            
            info!("Event envelope created: {} (sequence: {})", 
                envelope.event_id, envelope.sequence);
        }
        
        event_count += 1;
        
        // Periodic stats
        if event_count % 10000 == 0 {
            let process_count = process_monitor.process_count();
            let connection_count = network_monitor.connection_count();
            let bp_stats = backpressure.stats();
            let health_stats = health_monitor.stats();
            
            info!("Stats: events={}, processes={}, connections={}, dropped={}, healthy={}", 
                event_count, process_count, connection_count, bp_stats.events_dropped, health_stats.healthy);
        }
    }
    
    syscall_monitor.stop();
    hardening.stop_watchdog();
    info!("Linux Agent stopped");
    Ok(())
}

