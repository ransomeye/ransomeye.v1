// Path and File Name : /home/ransomeye/rebuild/ransomeye_validation/src/main.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Main validation orchestrator - coordinates all validation suites and generates release decision

use std::path::PathBuf;
use std::time::Instant;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::{info, error, warn};
use thiserror::Error;

pub mod chaos;
pub mod replay;
pub mod verifier;
pub mod auditor;
mod suites;

use suites::{
    security::SecuritySuite,
    performance::PerformanceSuite,
    stress::StressSuite,
    fault_injection::FaultInjectionSuite,
    compliance::ComplianceSuite,
    regression::RegressionSuite,
};

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Security validation failed: {0}")]
    SecurityFailed(String),
    #[error("Performance validation failed: {0}")]
    PerformanceFailed(String),
    #[error("Stress validation failed: {0}")]
    StressFailed(String),
    #[error("Fault injection validation failed: {0}")]
    FaultInjectionFailed(String),
    #[error("Compliance validation failed: {0}")]
    ComplianceFailed(String),
    #[error("Regression validation failed: {0}")]
    RegressionFailed(String),
    #[error("Report generation failed: {0}")]
    ReportFailed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub suite_name: String,
    pub passed: bool,
    pub duration_ms: u64,
    pub findings: Vec<Finding>,
    pub timestamp: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub severity: Severity,
    pub category: String,
    pub description: String,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseDecision {
    pub decision: Decision,
    pub timestamp: chrono::DateTime<Utc>,
    pub validation_results: Vec<ValidationResult>,
    pub justification: String,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Decision {
    Allow,
    Block,
    Hold,
}

pub struct ValidationOrchestrator {
    reports_dir: PathBuf,
}

impl ValidationOrchestrator {
    pub fn new(reports_dir: PathBuf) -> Self {
        Self { reports_dir }
    }

    pub async fn run_full_validation(&self) -> Result<ReleaseDecision, ValidationError> {
        info!("Starting full validation suite");
        let start_time = Instant::now();
        
        let mut results = Vec::new();
        
        // Security validation
        info!("Running security validation suite");
        let security_result = SecuritySuite::new().run().await
            .map_err(|e| ValidationError::SecurityFailed(e.to_string()))?;
        results.push(security_result);
        
        // Performance validation
        info!("Running performance validation suite");
        let perf_result = PerformanceSuite::new().run().await
            .map_err(|e| ValidationError::PerformanceFailed(e.to_string()))?;
        results.push(perf_result);
        
        // Stress validation
        info!("Running stress validation suite");
        let stress_result = StressSuite::new().run().await
            .map_err(|e| ValidationError::StressFailed(e.to_string()))?;
        results.push(stress_result);
        
        // Fault injection validation
        info!("Running fault injection validation suite");
        let fault_result = FaultInjectionSuite::new().run().await
            .map_err(|e| ValidationError::FaultInjectionFailed(e.to_string()))?;
        results.push(fault_result);
        
        // Compliance validation
        info!("Running compliance validation suite");
        let compliance_result = ComplianceSuite::new().run().await
            .map_err(|e| ValidationError::ComplianceFailed(e.to_string()))?;
        results.push(compliance_result);
        
        // Regression validation
        info!("Running regression validation suite");
        let regression_result = RegressionSuite::new().run().await
            .map_err(|e| ValidationError::RegressionFailed(e.to_string()))?;
        results.push(regression_result);
        
        let duration = start_time.elapsed();
        info!("Validation completed in {:?}", duration);
        
        // Generate release decision
        let decision = self.generate_decision(&results)?;
        
        // Generate reports
        self.generate_reports(&results, &decision).await?;
        
        Ok(decision)
    }
    
    fn generate_decision(&self, results: &[ValidationResult]) -> Result<ReleaseDecision, ValidationError> {
        let mut critical_count = 0;
        let mut high_count = 0;
        let mut failed_suites = Vec::new();
        
        for result in results {
            if !result.passed {
                failed_suites.push(result.suite_name.clone());
            }
            
            for finding in &result.findings {
                match finding.severity {
                    Severity::Critical => critical_count += 1,
                    Severity::High => high_count += 1,
                    _ => {}
                }
            }
        }
        
        let decision = if !failed_suites.is_empty() {
            Decision::Block
        } else if critical_count > 0 || high_count > 0 {
            Decision::Hold
        } else {
            Decision::Allow
        };
        
        let justification = if decision == Decision::Block {
            format!("Release BLOCKED: {} suite(s) failed: {}", 
                failed_suites.len(), 
                failed_suites.join(", "))
        } else if decision == Decision::Hold {
            format!("Release HELD: {} critical, {} high severity findings", 
                critical_count, high_count)
        } else {
            "All validation suites passed. No critical or high severity findings.".to_string()
        };
        
        Ok(ReleaseDecision {
            decision,
            timestamp: Utc::now(),
            validation_results: results.to_vec(),
            justification,
            signature: None, // Will be signed by auditor
        })
    }
    
    async fn generate_reports(
        &self,
        results: &[ValidationResult],
        decision: &ReleaseDecision,
    ) -> Result<(), ValidationError> {
        use std::fs;
        use std::io::Write;
        
        // Generate security report
        let security_report = self.generate_security_report(results)?;
        let security_path = self.reports_dir.join("security_report.md");
        fs::File::create(&security_path)
            .and_then(|mut f| f.write_all(security_report.as_bytes()))
            .map_err(|e| ValidationError::ReportFailed(format!("Failed to write security report: {}", e)))?;
        
        // Generate performance report
        let perf_report = self.generate_performance_report(results)?;
        let perf_path = self.reports_dir.join("performance_report.md");
        fs::File::create(&perf_path)
            .and_then(|mut f| f.write_all(perf_report.as_bytes()))
            .map_err(|e| ValidationError::ReportFailed(format!("Failed to write performance report: {}", e)))?;
        
        // Generate stress report
        let stress_report = self.generate_stress_report(results)?;
        let stress_path = self.reports_dir.join("stress_report.md");
        fs::File::create(&stress_path)
            .and_then(|mut f| f.write_all(stress_report.as_bytes()))
            .map_err(|e| ValidationError::ReportFailed(format!("Failed to write stress report: {}", e)))?;
        
        // Generate compliance report
        let compliance_report = self.generate_compliance_report(results)?;
        let compliance_path = self.reports_dir.join("compliance_report.md");
        fs::File::create(&compliance_path)
            .and_then(|mut f| f.write_all(compliance_report.as_bytes()))
            .map_err(|e| ValidationError::ReportFailed(format!("Failed to write compliance report: {}", e)))?;
        
        // Generate release decision report
        let decision_report = self.generate_decision_report(decision)?;
        let decision_path = self.reports_dir.join("release_decision.md");
        fs::File::create(&decision_path)
            .and_then(|mut f| f.write_all(decision_report.as_bytes()))
            .map_err(|e| ValidationError::ReportFailed(format!("Failed to write release decision: {}", e)))?;
        
        // Generate JSON summary
        let json_summary = serde_json::to_string_pretty(decision)
            .map_err(|e| ValidationError::ReportFailed(format!("Failed to serialize decision: {}", e)))?;
        let json_path = self.reports_dir.join("release_decision.json");
        fs::File::create(&json_path)
            .and_then(|mut f| f.write_all(json_summary.as_bytes()))
            .map_err(|e| ValidationError::ReportFailed(format!("Failed to write JSON summary: {}", e)))?;
        
        info!("All reports generated in {:?}", self.reports_dir);
        Ok(())
    }
    
    fn generate_security_report(&self, results: &[ValidationResult]) -> Result<String, ValidationError> {
        let mut report = String::new();
        report.push_str("# Security Validation Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", Utc::now().to_rfc3339()));
        
        if let Some(security_result) = results.iter().find(|r| r.suite_name == "security") {
            report.push_str(&format!("## Status: {}\n\n", 
                if security_result.passed { "PASS" } else { "FAIL" }));
            report.push_str(&format!("Duration: {}ms\n\n", security_result.duration_ms));
            
            if !security_result.findings.is_empty() {
                report.push_str("## Findings\n\n");
                for finding in &security_result.findings {
                    report.push_str(&format!("### {} - {}\n", 
                        format!("{:?}", finding.severity), finding.category));
                    report.push_str(&format!("**Description:** {}\n\n", finding.description));
                    report.push_str(&format!("**Evidence:** {}\n\n", finding.evidence));
                }
            }
        }
        
        Ok(report)
    }
    
    fn generate_performance_report(&self, results: &[ValidationResult]) -> Result<String, ValidationError> {
        let mut report = String::new();
        report.push_str("# Performance Validation Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", Utc::now().to_rfc3339()));
        
        if let Some(perf_result) = results.iter().find(|r| r.suite_name == "performance") {
            report.push_str(&format!("## Status: {}\n\n", 
                if perf_result.passed { "PASS" } else { "FAIL" }));
            report.push_str(&format!("Duration: {}ms\n\n", perf_result.duration_ms));
            
            if !perf_result.findings.is_empty() {
                report.push_str("## Metrics\n\n");
                for finding in &perf_result.findings {
                    report.push_str(&format!("- **{}:** {}\n", finding.category, finding.description));
                }
            }
        }
        
        Ok(report)
    }
    
    fn generate_stress_report(&self, results: &[ValidationResult]) -> Result<String, ValidationError> {
        let mut report = String::new();
        report.push_str("# Stress Validation Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", Utc::now().to_rfc3339()));
        
        if let Some(stress_result) = results.iter().find(|r| r.suite_name == "stress") {
            report.push_str(&format!("## Status: {}\n\n", 
                if stress_result.passed { "PASS" } else { "FAIL" }));
            report.push_str(&format!("Duration: {}ms\n\n", stress_result.duration_ms));
            
            if !stress_result.findings.is_empty() {
                report.push_str("## Stress Test Results\n\n");
                for finding in &stress_result.findings {
                    report.push_str(&format!("- **{}:** {}\n", finding.category, finding.description));
                }
            }
        }
        
        Ok(report)
    }
    
    fn generate_compliance_report(&self, results: &[ValidationResult]) -> Result<String, ValidationError> {
        let mut report = String::new();
        report.push_str("# Compliance Validation Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", Utc::now().to_rfc3339()));
        
        if let Some(compliance_result) = results.iter().find(|r| r.suite_name == "compliance") {
            report.push_str(&format!("## Status: {}\n\n", 
                if compliance_result.passed { "PASS" } else { "FAIL" }));
            report.push_str(&format!("Duration: {}ms\n\n", compliance_result.duration_ms));
            
            if !compliance_result.findings.is_empty() {
                report.push_str("## Compliance Checks\n\n");
                for finding in &compliance_result.findings {
                    report.push_str(&format!("- **{}:** {}\n", finding.category, finding.description));
                }
            }
        }
        
        Ok(report)
    }
    
    fn generate_decision_report(&self, decision: &ReleaseDecision) -> Result<String, ValidationError> {
        let mut report = String::new();
        report.push_str("# Release Decision Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", decision.timestamp.to_rfc3339()));
        report.push_str(&format!("## Decision: {:?}\n\n", decision.decision));
        report.push_str(&format!("## Justification\n\n{}\n\n", decision.justification));
        
        report.push_str("## Validation Summary\n\n");
        for result in &decision.validation_results {
            report.push_str(&format!("- **{}:** {}\n", 
                result.suite_name, 
                if result.passed { "PASS" } else { "FAIL" }));
        }
        
        report.push_str("\n---\n");
        report.push_str("© RansomEye.Tech | Support: Gagan@RansomEye.Tech\n");
        
        Ok(report)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    let reports_dir = PathBuf::from("/home/ransomeye/rebuild/ransomeye_validation/reports");
    std::fs::create_dir_all(&reports_dir)?;
    
    let orchestrator = ValidationOrchestrator::new(reports_dir);
    let decision = orchestrator.run_full_validation().await?;
    
    println!("Release Decision: {:?}", decision.decision);
    println!("Justification: {}", decision.justification);
    
    if matches!(decision.decision, Decision::Block) {
        std::process::exit(1);
    }
    
    Ok(())
}

