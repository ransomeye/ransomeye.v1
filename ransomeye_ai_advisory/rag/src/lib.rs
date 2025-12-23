// Path and File Name : /home/ransomeye/rebuild/ransomeye_ai_advisory/rag/src/lib.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: RAG module exports - read-only index access and deterministic retrieval

pub mod index;
pub mod retrieval;
pub mod errors;

pub use index::RAGIndex;
pub use retrieval::RAGRetriever;
pub use errors::RAGError;

