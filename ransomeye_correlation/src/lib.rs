// Path and File Name : /home/ransomeye/rebuild/ransomeye_correlation/src/lib.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Core correlation engine library exports

pub mod engine;
pub mod pipeline;
pub mod correlator;
pub mod rules;
pub mod state;
pub mod window;
pub mod kill_chain;
pub mod evidence;
pub mod ordering;
pub mod output;
pub mod errors;
pub mod security;

pub use engine::CorrelationEngine;
pub use errors::CorrelationError;
pub use output::Alert;
pub use pipeline::ProcessedEvent;

