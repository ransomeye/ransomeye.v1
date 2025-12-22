// Path and File Name : /home/ransomeye/rebuild/ransomeye_policy/tools/sign_policies.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Standalone tool to sign policy files using ring

use std::env;
use std::path::Path;
use std::fs;
use ring::signature::{self, RsaKeyPair};
use ring::rand::SystemRandom;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};
use serde_yaml;

fn sign_policy_content(
    policy_bytes: &[u8],
    private_key_der: &[u8],
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let key_pair = RsaKeyPair::from_pkcs8(private_key_der)
        .map_err(|e| format!("Failed to load RSA key pair: {:?}", e))?;
    
    // Verify key size (4096 bits = 512 bytes)
    let modulus_len = key_pair.public_modulus_len();
    if modulus_len != 512 {
        return Err(format!(
            "Key size mismatch: expected 512 bytes (4096 bits), got {} bytes",
            modulus_len
        ).into());
    }
    
    let mut hasher = Sha256::new();
    hasher.update(policy_bytes);
    let content_hash = hex::encode(hasher.finalize());
    
    let rng = SystemRandom::new();
    let mut signature = vec![0u8; modulus_len];
    
    // Use RSA_PKCS1_SHA256 for signing (encoding type, compatible with RSA_PKCS1_2048_8192_SHA256 for verification)
    use ring::signature::RSA_PKCS1_SHA256;
    key_pair.sign(
        &RSA_PKCS1_SHA256,
        &rng,
        policy_bytes,
        &mut signature,
    ).map_err(|e| format!("Failed to sign policy: {:?}", e))?;
    
    let signature_base64 = general_purpose::STANDARD.encode(&signature);
    
    Ok((signature_base64, content_hash))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <private_key_der> <policy_file> [policy_file2 ...]", args[0]);
        eprintln!("  private_key_der: Path to RSA-4096 private key in DER format (PKCS#8)");
        eprintln!("  policy_file: Path to policy YAML file to sign");
        std::process::exit(1);
    }
    
    let private_key_path = Path::new(&args[1]);
    let private_key_der = fs::read(private_key_path)
        .map_err(|e| format!("Failed to read private key: {}", e))?;
    
    for policy_file in args.iter().skip(2) {
        let policy_path = Path::new(policy_file);
        
        println!("Signing policy: {}", policy_path.display());
        
        // Read policy file
        let content = fs::read_to_string(policy_path)?;
        
        // Parse YAML
        let mut policy_data: serde_yaml::Value = serde_yaml::from_str(&content)?;
        
        // Extract header comments
        let header_lines: Vec<String> = content
            .lines()
            .take_while(|line| line.trim_start().starts_with('#'))
            .map(|s| s.to_string())
            .collect();
        
        // Remove signature fields
        if let Some(obj) = policy_data.as_mapping_mut() {
            obj.remove("signature");
            obj.remove("signature_hash");
            obj.remove("signature_alg");
            obj.remove("key_id");
        }
        
        // Serialize to YAML (this preserves field order from the Value structure)
        let policy_bytes = serde_yaml::to_string(&policy_data)?;
        
        // Sign the policy
        let (signature, hash) = sign_policy_content(policy_bytes.as_bytes(), &private_key_der)?;
        
        // Update policy with signature
        if let Some(obj) = policy_data.as_mapping_mut() {
            obj.insert(
                serde_yaml::Value::String("signature".to_string()),
                serde_yaml::Value::String(signature.clone()),
            );
            obj.insert(
                serde_yaml::Value::String("signature_hash".to_string()),
                serde_yaml::Value::String(hash.clone()),
            );
            obj.insert(
                serde_yaml::Value::String("signature_alg".to_string()),
                serde_yaml::Value::String("RSA-4096-SHA256".to_string()),
            );
            obj.insert(
                serde_yaml::Value::String("key_id".to_string()),
                serde_yaml::Value::String("policy_root_v1".to_string()),
            );
        }
        
        // Serialize updated policy
        let updated_content = serde_yaml::to_string(&policy_data)?;
        
        // Write back with header
        let mut final_content = header_lines.join("\n");
        if !final_content.is_empty() {
            final_content.push('\n');
        }
        final_content.push_str(&updated_content);
        
        fs::write(policy_path, final_content)?;
        
        println!("  ✓ Signed successfully");
        println!("  Signature hash: {}", &hash[..16]);
    }
    
    Ok(())
}

