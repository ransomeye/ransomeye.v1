// Path and File Name : /home/ransomeye/rebuild/ransomeye_ai_advisory/src/lib.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: AI Advisory library exports - advisory-only AI assistance

// Include subdirectory modules
#[path = "../inference/mod.rs"]
pub mod inference;

#[path = "../explainability/mod.rs"]
pub mod explainability;

#[path = "../rag/mod.rs"]
pub mod rag;

#[path = "../security/mod.rs"]
pub mod security;

#[path = "../config/mod.rs"]
pub mod config;
pub mod advisory_boundary;

// Legacy modules (for compatibility)
pub mod engine;
pub mod scorer;
pub mod explainer;
pub mod context;
pub mod outputs;
pub mod controller;
pub mod errors;
pub mod registry;
pub mod shap;
pub mod llm;

pub use engine::AdvisoryEngine;
pub use errors::AdvisoryError;
pub use outputs::AdvisoryOutput;
pub use advisory_boundary::{AdvisoryBoundaryGuard, AdvisoryOutput as BoundaryAdvisoryOutput};

