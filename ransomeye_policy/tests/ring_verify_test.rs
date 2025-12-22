// Path and File Name : /home/ransomeye/rebuild/ransomeye_policy/tests/ring_verify_test.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Test ring signature verification directly

use std::fs;
use ring::signature::{self, UnparsedPublicKey};
use base64::{Engine as _, engine::general_purpose};
use serde_yaml;

#[test]
fn test_ring_verify_persistence_policy() {
    // Read policy
    let content = fs::read_to_string("policies/persistence.yaml")
        .expect("Failed to read policy");
    
    let policy_data: serde_yaml::Value = serde_yaml::from_str(&content)
        .expect("Failed to parse policy");
    
    let signature_b64 = policy_data.get("signature")
        .and_then(|v| v.as_str())
        .expect("No signature found");
    
    // Remove signature fields
    let mut policy_for_verify = policy_data.clone();
    if let Some(obj) = policy_for_verify.as_mapping_mut() {
        obj.remove("signature");
        obj.remove("signature_hash");
        obj.remove("signature_alg");
        obj.remove("key_id");
    }
    
    // Serialize (matching signing process)
    let content_to_verify = serde_yaml::to_string(&policy_for_verify)
        .expect("Failed to serialize");
    
    // Load public key (extracted by ring from private key)
    let public_key_der = fs::read("security/trust_store/policy_root_public_ring.der")
        .expect("Failed to read public key");
    
    // Decode signature
    let signature_bytes = general_purpose::STANDARD.decode(signature_b64)
        .expect("Failed to decode signature");
    
    println!("Content length: {} bytes", content_to_verify.len());
    println!("Signature length: {} bytes", signature_bytes.len());
    println!("Public key length: {} bytes", public_key_der.len());
    
    // Verify with ring
    let public_key = UnparsedPublicKey::new(
        &signature::RSA_PKCS1_2048_8192_SHA256,
        &public_key_der,
    );
    
    let result = public_key.verify(content_to_verify.as_bytes(), &signature_bytes);
    
    match result {
        Ok(_) => {
            println!("✓ Ring verification SUCCESS");
            assert!(true);
        }
        Err(e) => {
            println!("✗ Ring verification FAILED: {:?}", e);
            panic!("Ring verification failed: {:?}", e);
        }
    }
}

