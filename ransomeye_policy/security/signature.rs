// Path and File Name : /home/ransomeye/rebuild/ransomeye_policy/security/signature.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Real cryptographic policy signature verification using RSA-4096

use ring::signature::{self, UnparsedPublicKey, VerificationAlgorithm};
use sha2::{Sha256, Digest};
use base64;
use std::path::Path;
use std::fs;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{error, debug, warn};
use once_cell::sync::Lazy;

static TRUST_STORE: Lazy<Arc<RwLock<TrustStore>>> = Lazy::new(|| {
    Arc::new(RwLock::new(TrustStore::new()))
});

pub struct TrustStore {
    public_keys: Vec<Vec<u8>>,
}

impl TrustStore {
    fn new() -> Self {
        Self {
            public_keys: Vec::new(),
        }
    }

    pub fn load_public_key(&mut self, key_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let key_data = fs::read_to_string(key_path)
            .map_err(|e| format!("Failed to read public key from {}: {}", key_path, e))?;
        
        let key_bytes = base64::decode(key_data.trim())
            .map_err(|e| format!("Failed to decode public key: {}", e))?;
        
        self.public_keys.push(key_bytes);
        debug!("Loaded public key from {}", key_path);
        Ok(())
    }

    pub fn get_public_keys(&self) -> &[Vec<u8>] {
        &self.public_keys
    }
}

pub struct PolicySignatureVerifier {
    algorithm: &'static dyn VerificationAlgorithm,
}

impl PolicySignatureVerifier {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            algorithm: &signature::RSA_PSS_SAE_SHA256,
        })
    }

    pub fn load_trust_store(trust_store_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let trust_dir = Path::new(trust_store_path);
        if !trust_dir.exists() {
            return Err(format!("Trust store directory not found: {}", trust_store_path).into());
        }

        let mut store = TRUST_STORE.write();
        for entry in fs::read_dir(trust_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("pem") ||
               path.extension().and_then(|s| s.to_str()) == Some("pub") {
                if let Err(e) = store.load_public_key(path.to_str().unwrap()) {
                    warn!("Failed to load public key from {}: {}", path.display(), e);
                }
            }
        }

        if store.public_keys.is_empty() {
            return Err("No valid public keys found in trust store".into());
        }

        debug!("Loaded {} public keys from trust store", store.public_keys.len());
        Ok(())
    }

    pub fn verify(&self, content: &str, signature: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let content_hash = {
            let mut hasher = Sha256::new();
            hasher.update(content.as_bytes());
            hasher.finalize()
        };

        let signature_bytes = base64::decode(signature)
            .map_err(|e| format!("Failed to decode signature: {}", e))?;

        let store = TRUST_STORE.read();
        let public_keys = store.get_public_keys();

        if public_keys.is_empty() {
            error!("No public keys loaded in trust store");
            return Ok(false);
        }

        for public_key_bytes in public_keys {
            let public_key = UnparsedPublicKey::new(self.algorithm, public_key_bytes);
            
            match public_key.verify(&content_hash, &signature_bytes) {
                Ok(_) => {
                    debug!("Policy signature verified successfully");
                    return Ok(true);
                }
                Err(_) => {
                    continue;
                }
            }
        }

        error!("Policy signature verification failed: no matching public key");
        Ok(false)
    }

    pub fn verify_with_key(&self, content: &str, signature: &str, public_key_bytes: &[u8]) -> Result<bool, Box<dyn std::error::Error>> {
        let content_hash = {
            let mut hasher = Sha256::new();
            hasher.update(content.as_bytes());
            hasher.finalize()
        };

        let signature_bytes = base64::decode(signature)
            .map_err(|e| format!("Failed to decode signature: {}", e))?;

        let public_key = UnparsedPublicKey::new(self.algorithm, public_key_bytes);
        
        match public_key.verify(&content_hash, &signature_bytes) {
            Ok(_) => {
                debug!("Policy signature verified successfully");
                Ok(true)
            }
            Err(e) => {
                error!("Policy signature verification failed: {:?}", e);
                Ok(false)
            }
        }
    }
}

