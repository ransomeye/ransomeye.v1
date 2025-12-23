# Phase 0 — Global Guardrails

**Path and File Name:** `/home/ransomeye/rebuild/docs/readme/00_Guardrails_readme.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Comprehensive technical audit and documentation for Phase 0 - Global Guardrails

---

## 1️⃣ Phase Overview

### Purpose
Phase 0 establishes **irreversible, fail-closed guardrails** that enforce security and compliance rules across all RansomEye phases. These guardrails act as build-time and runtime enforcement mechanisms that ensure no future phase can violate core security principles without causing build failure.

### Security Objective
- **Fail-closed enforcement** of security invariants
- **Prevention of hardcoded secrets**, IPs, URLs, and paths
- **Mandatory file headers** across all source files
- **ML model governance** (training requirements, SHAP explainability)
- **Cryptographic signature enforcement** for all artifacts
- **Data retention policy** validation

### Role in Architecture
Phase 0 serves as the **foundational security layer** that all subsequent phases must conform to. It runs during:
- **Build time**: Static code analysis and validation
- **CI/CD pipelines**: Automated enforcement before merge
- **Pre-commit hooks**: Developer workflow integration

---

## 2️⃣ Implementation Status

| Component | Status | Notes |
|-----------|--------|-------|
| Static Scanner | ✅ Fully Implemented | Regex and AST-based pattern detection |
| Header Enforcer | ✅ Fully Implemented | Validates mandatory file headers |
| ENV Enforcer | ✅ Fully Implemented | Enforces ENV-only configuration |
| ML Enforcer | ✅ Fully Implemented | Validates ML training and SHAP requirements |
| Crypto Enforcer | ✅ Fully Implemented | Validates cryptographic signatures |
| Retention Enforcer | ✅ Fully Implemented | Validates retention policy configuration |
| Fail-Closed Mechanism | ✅ Fully Implemented | Central abort mechanism for violations |
| Runtime Check | ✅ Fully Implemented | Python 3.10+ validation |

---

## 3️⃣ File & Folder Structure

### Root Directory
`/home/ransomeye/rebuild/ransomeye_guardrails/`

### Key Files
- **`main.py`**: Main entry point for all guardrail checks
- **`scanner.py`**: Static code scanner using regex and AST parsing
- **`header_enforcer.py`**: Validates mandatory file headers
- **`env_enforcer.py`**: Enforces ENV-only configuration
- **`ml_enforcer.py`**: Enforces ML training and SHAP requirements
- **`crypto_enforcer.py`**: Validates cryptographic signatures
- **`retention_enforcer.py`**: Validates retention policy configuration
- **`fail_closed.py`**: Central abort mechanism for violations
- **`runtime_check.py`**: Python runtime version validation
- **`rules.yaml`**: Forbidden patterns and rules configuration
- **`rules_schema.py`**: Schema validation for rules.yaml

### Configuration Files
- **`rules.yaml`**: Located at `/home/ransomeye/rebuild/ransomeye_guardrails/rules.yaml`
  - Contains forbidden patterns (hardcoded IPs, URLs, secrets, paths)
  - Defines severity levels and exceptions

### Related Infrastructure
- **Trust Infrastructure**: `/home/ransomeye/rebuild/ransomeye_trust/`
  - Root CA generator
  - Sign and verify tools
- **Retention Infrastructure**: `/home/ransomeye/rebuild/ransomeye_retention/`
  - Retention policy parser
  - Disk monitor
  - Telemetry and forensic retention modules

---

## 4️⃣ Modules & Services

### Modules
1. **GuardrailScanner** (`scanner.py`)
   - **Responsibility**: Static code analysis using regex and AST parsing
   - **Runtime Behavior**: Scans source files for forbidden patterns
   - **systemd Integration**: ❌ NO (build-time tool)
   - **Installer Integration**: ❌ NO (pre-install validation)

2. **HeaderEnforcer** (`header_enforcer.py`)
   - **Responsibility**: Validates mandatory file headers
   - **Runtime Behavior**: Checks all source files have required header format
   - **systemd Integration**: ❌ NO
   - **Installer Integration**: ❌ NO

3. **EnvEnforcer** (`env_enforcer.py`)
   - **Responsibility**: Enforces ENV-only configuration
   - **Runtime Behavior**: Detects hardcoded values (IPs, ports, paths, secrets)
   - **systemd Integration**: ❌ NO
   - **Installer Integration**: ❌ NO

4. **MLEnforcer** (`ml_enforcer.py`)
   - **Responsibility**: Enforces ML training and SHAP requirements
   - **Runtime Behavior**: Validates models have training scripts and SHAP files
   - **systemd Integration**: ❌ NO
   - **Installer Integration**: ❌ NO

5. **CryptoEnforcer** (`crypto_enforcer.py`)
   - **Responsibility**: Validates cryptographic signatures
   - **Runtime Behavior**: Verifies manifest.json and manifest.sig for artifacts
   - **systemd Integration**: ❌ NO
   - **Installer Integration**: ❌ NO

6. **RetentionEnforcer** (`retention_enforcer.py`)
   - **Responsibility**: Validates retention policy configuration
   - **Runtime Behavior**: Ensures retention enforcement modules exist
   - **systemd Integration**: ❌ NO
   - **Installer Integration**: ❌ NO

### Services
**NO systemd services** - Phase 0 is a build-time validation tool, not a runtime service.

---

## 5️⃣ AI / ML / LLM DETAILS

**NOT APPLICABLE** - Phase 0 is a validation and enforcement tool, not an AI/ML module.

---

## 6️⃣ SOC Copilot / AI Copilot

**NOT PRESENT** - Phase 0 does not include AI Copilot functionality.

---

## 7️⃣ Database Design

**NOT APPLICABLE** - Phase 0 does not use a database. It performs static code analysis and validation only.

---

## 8️⃣ Ports & Interconnectivity

**NO NETWORK PORTS** - Phase 0 is a local build-time tool with no network connectivity requirements.

**Interconnectivity:**
- Reads source files from `/home/ransomeye/rebuild/`
- Reads configuration from `ransomeye_guardrails/rules.yaml`
- Reads retention policy from `config/retention.txt`
- Validates trust infrastructure in `ransomeye_trust/`
- Validates retention infrastructure in `ransomeye_retention/`

---

## 9️⃣ UI / Dashboards / Frontend

**NO UI IN THIS PHASE** - Phase 0 is a command-line validation tool.

**Output:**
- Console output with violation reports
- Exit code 1 on violations (fail-closed)
- Exit code 0 on success

---

## 🔟 Logging, Metrics & Observability

### Logs Generated
- **Console Output**: Violation reports printed to stdout/stderr
- **Exit Codes**: 
  - `0`: All checks passed
  - `1`: Violation detected (fail-closed)

### Log Formats
- **Violation Reports**: Human-readable text with file paths, line numbers, and violation descriptions
- **Success Messages**: Simple confirmation messages

### Metrics Exposed
**NO METRICS** - Phase 0 is a build-time validation tool.

### Prometheus/Grafana Integration
**NOT APPLICABLE**

### Audit Logs
**NO PERSISTENT AUDIT LOGS** - Violations are reported immediately and cause build failure.

### Tamper-Proofing
- **Fail-closed mechanism**: Any violation immediately terminates the build
- **No bypass mechanisms**: Violations cannot be ignored or suppressed
- **Runtime validation**: Python 3.10+ requirement enforced before execution

---

## 1️⃣1️⃣ Security & Compliance

### Fail-Closed Enforcement
✅ **FULLY ENFORCED**
- All violations cause immediate exit with code 1
- No warnings-only mode
- No bypass mechanisms
- Runtime check validates Python version before execution

### Cryptographic Controls
✅ **VALIDATED**
- Crypto Enforcer validates manifest.json and manifest.sig for all artifacts
- Verifies cryptographic signatures
- Enforces trust domain hierarchy

### Signature Verification
✅ **ENFORCED**
- All artifacts must have manifest.json and manifest.sig
- Signature verification required before build proceeds

### Zero-Trust Enforcement
**PARTIALLY APPLICABLE**
- Validates that code does not hardcode credentials
- Enforces ENV-only configuration
- Validates cryptographic signatures

### STIG Hardening Status
**NOT APPLICABLE** - Phase 0 is a validation tool, not a runtime service.

---

## 1️⃣2️⃣ CI / Validation / Testing

### Tests Present
✅ **YES** - Located in `/home/ransomeye/rebuild/ransomeye_guardrails/tests/`

### Test Coverage
- Unit tests for each enforcer module
- Integration tests for rule validation
- Test cases for violation detection

### Synthetic Data Generation
**NOT APPLICABLE** - Phase 0 tests use real source code patterns.

### CI Workflows
✅ **YES** - CI pipeline located at `ci/global_guardrails.yml`
- Runs on push to `main` or `develop` branches
- Runs on pull requests
- Manual workflow dispatch supported

### Validation Coverage
✅ **COMPREHENSIVE**
- All 6 enforcer modules tested
- Rules validation tested
- Fail-closed mechanism tested
- Runtime check tested

---

## 1️⃣3️⃣ Known Gaps & Technical Debt

### Missing Components
**NONE IDENTIFIED** - Phase 0 implementation appears complete.

### Partial Implementations
**NONE IDENTIFIED**

### Design Risks
1. **Regex-based pattern matching**: May produce false positives/negatives for complex patterns
   - **Mitigation**: AST parsing used where applicable
   - **Recommendation**: Consider expanding AST-based analysis

2. **No persistent audit log**: Violations only reported during build
   - **Recommendation**: Consider adding optional audit log for compliance tracking

---

## 1️⃣4️⃣ Recommendations

### Refactors
1. **Expand AST-based analysis**: Current scanner uses regex for many patterns; AST analysis would be more robust for complex code patterns.

2. **Add persistent audit logging**: For compliance purposes, consider adding optional audit log that records all violations (even if build fails).

### Missing Enforcement
**NONE IDENTIFIED** - Current enforcement appears comprehensive.

### Architectural Fixes
**NONE IDENTIFIED** - Architecture is sound for a build-time validation tool.

### Training Improvements
**NOT APPLICABLE** - Phase 0 does not use ML models.

### Security Hardening
1. **Code signing for guardrails themselves**: Consider signing the guardrails tool and validating its signature before execution.

2. **Immutable rules.yaml**: Consider cryptographic signing of rules.yaml to prevent tampering.

---

## Summary

Phase 0 (Global Guardrails) is **FULLY IMPLEMENTED** and serves as a robust foundation for enforcing security and compliance rules across all RansomEye phases. The implementation includes comprehensive static analysis, mandatory file header validation, ENV-only configuration enforcement, ML model governance, cryptographic signature validation, and retention policy validation. All components use fail-closed mechanisms with no bypass options, ensuring strict enforcement of security invariants.

**Status**: ✅ **PRODUCTION READY**

---

**Last Updated**: 2025-01-27  
**Validation Status**: ✅ All components validated and tested

