# Model Governance

**Path and File Name:** `/home/ransomeye/rebuild/ransomeye_ai_advisory/docs/model_governance.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Model governance and integrity requirements

## Model Requirements

### Signature Verification
- All models MUST be signed with RSA-4096
- Signatures verified before model load
- Unsigned models REJECTED

### Integrity Checks
- SHA-256 hash verification
- Manifest validation
- Size verification

### Resource Constraints
- Core AI storage ≤ 3 GB
- DPI Probe AI storage ≤ 3 GB
- Bounded memory usage at runtime

## Model Loading Process

1. Load manifest
2. Verify manifest signature (RSA-4096)
3. Load model file
4. Verify model hash
5. Verify model size
6. Check memory limits
7. Load model (read-only)

## Failure Handling

- Missing model → AI DISABLED
- Invalid signature → AI DISABLED
- Hash mismatch → AI DISABLED
- Size exceeded → AI DISABLED

All failures are fail-closed: AI disabled, core continues.

