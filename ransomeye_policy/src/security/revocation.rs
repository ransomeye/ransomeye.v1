// Path and File Name : /home/ransomeye/rebuild/ransomeye_policy/src/security/revocation.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Policy revocation - checks if policies are revoked

/*
 * Policy Revocation
 * 
 * Checks if policies are revoked.
 * Revoked policy → not used in evaluation
 */

use std::collections::HashSet;
use tracing::{debug, warn};

use crate::errors::PolicyError;

pub struct PolicyRevocationChecker {
    revoked_policies: HashSet<String>,
}

impl PolicyRevocationChecker {
    pub fn new() -> Result<Self, PolicyError> {
        Ok(Self {
            revoked_policies: HashSet::new(),
        })
    }
    
    /// Check if policy is revoked
    pub fn is_revoked(&self, policy_id: &str) -> bool {
        let revoked = self.revoked_policies.contains(policy_id);
        if revoked {
            warn!("Policy is revoked: {}", policy_id);
        }
        revoked
    }
    
    /// Add revoked policy (for testing/manual revocation)
    pub fn revoke_policy(&mut self, policy_id: &str) {
        self.revoked_policies.insert(policy_id.to_string());
        debug!("Policy revoked: {}", policy_id);
    }
}

