// Path and File Name : /home/ransomeye/rebuild/ransomeye_ai_advisory/src/security/signature.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Model signature verification - RSA-PSS-SHA256

use ring::signature::{UnparsedPublicKey, RSA_PSS_SHA256};
use x509_parser::pem::Pem;
use x509_parser::prelude::*;
use tracing::{error, warn, debug};
use crate::errors::AdvisoryError;

pub struct ModelSignatureVerifier {
    public_key_bytes: Vec<u8>,
}

impl ModelSignatureVerifier {
    pub fn new() -> Result<Self, AdvisoryError> {
        let public_key_path = std::env::var("RANSOMEYE_AI_MODEL_PUBLIC_KEY_PATH")
            .unwrap_or_else(|_| "/etc/ransomeye/ai/keys/model_public_key.pem".to_string());
        
        let public_key_bytes = Self::parse_public_key(&public_key_path)?;
        
        Ok(Self {
            public_key_bytes,
        })
    }
    
    fn parse_public_key(path: &str) -> Result<Vec<u8>, AdvisoryError> {
        let pem_bytes = std::fs::read(path)
            .map_err(|e| AdvisoryError::ConfigurationError(
                format!("Failed to read public key from {}: {}", path, e)
            ))?;
        
        // Try to parse as X.509 certificate first
        if let Ok((_, pem)) = Pem::parse(&pem_bytes) {
            if let Ok((_, cert)) = x509_parser::parse_x509_certificate(&pem.contents) {
                let public_key_info = cert.public_key();
                return Ok(public_key_info.raw.to_vec());
            }
        }
        
        // Try to parse as standalone PEM public key
        let pem_str = std::str::from_utf8(&pem_bytes)
            .map_err(|e| AdvisoryError::ConfigurationError(format!("Invalid PEM encoding: {}", e)))?;
        
        let pem_lines: Vec<&str> = pem_str
            .lines()
            .filter(|line| !line.starts_with("-----"))
            .collect();
        
        let base64_content = pem_lines.join("");
        let key_bytes = base64::decode(&base64_content)
            .map_err(|e| AdvisoryError::ConfigurationError(format!("Base64 decode failed: {}", e)))?;
        
        if let Ok((_, spki)) = SubjectPublicKeyInfo::from_der(&key_bytes) {
            return Ok(spki.raw.to_vec());
        }
        
        Ok(key_bytes)
    }
    
    /// Verify model signature
    pub fn verify(&self, model_bytes: &[u8], signature: &str) -> Result<bool, AdvisoryError> {
        let signature_bytes = base64::decode(signature)
            .map_err(|e| AdvisoryError::InvalidModelSignature(format!("Base64 decode failed: {}", e)))?;
        
        // Compute hash of model
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(model_bytes);
        let model_hash = hasher.finalize();
        
        // Verify signature
        let unparsed_key = UnparsedPublicKey::new(&RSA_PSS_SHA256, &self.public_key_bytes);
        
        match unparsed_key.verify(&model_hash, &signature_bytes) {
            Ok(_) => {
                debug!("Model signature verification successful");
                Ok(true)
            }
            Err(e) => {
                warn!("Model signature verification failed: {:?}", e);
                Err(AdvisoryError::InvalidModelSignature(format!("Verification failed: {:?}", e)))
            }
        }
    }
}

