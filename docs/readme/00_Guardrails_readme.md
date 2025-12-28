# Phase 0 — Global Guardrails Enforcement

**Path and File Name:** `/home/ransomeye/rebuild/docs/readme/00_Guardrails_readme.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Forensic-grade technical validation and truth audit for Phase 0 - Global Guardrails

---

## 1️⃣ Phase Purpose & Security Objective

### Purpose
Phase 0 establishes **irreversible, fail-closed guardrails** that enforce security and compliance rules across all RansomEye phases. These guardrails act as build-time and runtime enforcement mechanisms that ensure no future phase can violate core security principles without causing build failure.

### Security Objective
- **Fail-closed enforcement** of security invariants
- **Prevention of hardcoded secrets**, IPs, URLs, and paths
- **Mandatory file headers** across all source files
- **ML model governance** (training requirements, SHAP explainability)
- **Cryptographic signature enforcement** for all artifacts
- **Data retention policy** validation
- **Phantom module detection** and prevention

### Role in Architecture
Phase 0 serves as the **foundational security layer** that all subsequent phases must conform to. It runs during:
- **Build time**: Static code analysis and validation
- **CI/CD pipelines**: Automated enforcement before merge
- **Pre-commit hooks**: Developer workflow integration
- **Service startup**: Runtime validation via `ExecStartPre` in systemd units

---

## 2️⃣ Implementation Status (What Exists vs What Doesn't)

| Component | Status | Location | Notes |
|-----------|--------|----------|-------|
| Static Scanner | ✅ **FULLY IMPLEMENTED** | `ransomeye_guardrails/scanner.py` | Regex and AST-based pattern detection |
| Header Enforcer | ✅ **FULLY IMPLEMENTED** | `ransomeye_guardrails/header_enforcer.py` | Validates mandatory file headers |
| ENV Enforcer | ✅ **FULLY IMPLEMENTED** | `ransomeye_guardrails/env_enforcer.py` | Enforces ENV-only configuration |
| ML Enforcer | ✅ **FULLY IMPLEMENTED** | `ransomeye_guardrails/ml_enforcer.py` | Validates ML training and SHAP requirements |
| Crypto Enforcer | ✅ **FULLY IMPLEMENTED** | `ransomeye_guardrails/crypto_enforcer.py` | Validates cryptographic signatures |
| Retention Enforcer | ✅ **FULLY IMPLEMENTED** | `ransomeye_guardrails/retention_enforcer.py` | Validates retention policy configuration |
| Fail-Closed Mechanism | ✅ **FULLY IMPLEMENTED** | `ransomeye_guardrails/fail_closed.py` | Central abort mechanism for violations |
| Runtime Check | ✅ **FULLY IMPLEMENTED** | `ransomeye_guardrails/runtime_check.py` | Python 3.10+ validation |
| Guardrails Specification | ✅ **FULLY IMPLEMENTED** | `ransomeye_guardrails/rules.yaml` | Signed YAML specification (Ed25519) |
| Main Entry Point | ✅ **FULLY IMPLEMENTED** | `ransomeye_guardrails/main.py` | Orchestrates all enforcers |

**CRITICAL FINDING: Phase 0 is FULLY IMPLEMENTED and operational. All core components exist and are functional.**

---

## 3️⃣ File & Folder Structure (Absolute Paths)

### Root Directory
`/home/ransomeye/rebuild/ransomeye_guardrails/`

### Core Files
```
ransomeye_guardrails/
├── __init__.py                    # Package initialization with runtime check
├── __main__.py                    # CLI entry point
├── main.py                        # Main orchestrator (runs all enforcers)
├── scanner.py                     # Static code scanner (regex + AST)
├── header_enforcer.py             # File header validation
├── env_enforcer.py                 # ENV-only configuration enforcement
├── ml_enforcer.py                  # ML model governance
├── crypto_enforcer.py              # Cryptographic signature validation
├── retention_enforcer.py           # Retention policy validation
├── fail_closed.py                  # Fail-closed abort mechanism
├── runtime_check.py                # Python 3.10+ runtime validation
├── rules_schema.py                 # YAML schema validation
├── rules.yaml                      # Guardrails specification (signed)
├── README.md                       # Module documentation
├── PHASE0_FIXES.md                 # Historical fixes documentation
├── RULES_YAML_FIX.md               # Specification correction history
└── tests/                          # Unit tests
    ├── test_scanner.py
    ├── test_header_enforcer.py
    ├── test_env_enforcer.py
    ├── test_ml_enforcer.py
    ├── test_crypto_enforcer.py
    └── test_retention_enforcer.py
```

### Supporting Infrastructure
- **Trust Library**: `/home/ransomeye/rebuild/ransomeye_trust/` (cryptographic signing)
- **Retention Library**: `/home/ransomeye/rebuild/ransomeye_retention/` (data retention)

---

## 4️⃣ Runtime Components & Services

### Command-Line Interface

**Canonical Invocation:**
```bash
cd /home/ransomeye/rebuild
python3 -m ransomeye_guardrails
```

**Individual Enforcers:**
```bash
# Static scanner
python3 -m ransomeye_guardrails.scanner --rules ransomeye_guardrails/rules.yaml

# Header enforcer
python3 -m ransomeye_guardrails.header_enforcer

# ENV enforcer
python3 -m ransomeye_guardrails.env_enforcer

# ML enforcer
python3 -m ransomeye_guardrails.ml_enforcer

# Crypto enforcer
python3 -m ransomeye_guardrails.crypto_enforcer

# Retention enforcer
python3 -m ransomeye_guardrails.retention_enforcer --dry-run
```

### Systemd Integration

**NO dedicated systemd service** - Guardrails run as:
1. **Pre-commit hooks** (developer workflow)
2. **CI/CD pipeline** (`ci/global_guardrails.yml`)
3. **Service startup validation** (via `ExecStartPre` in systemd units)

**Example systemd integration:**
```ini
[Service]
ExecStartPre=/usr/bin/python3 -m ransomeye_guardrails enforce --context service --data ransomeye-core
```

### Runtime Behavior

1. **Runtime Check**: Validates Python 3.10+ before any imports
2. **Fail-Closed**: Any violation triggers immediate exit (non-zero)
3. **Audit Logging**: All violations logged before exit
4. **Phantom Module Detection**: Detects and rejects non-existent module references

---

## 5️⃣ AI / ML / LLM Reality

**Phase 0 does NOT contain AI/ML/LLM models.** However, it **enforces** AI/ML/LLM governance:

### ML Model Requirements (Enforced)
- ✅ **Training Scripts Required**: All models must have training scripts
- ✅ **SHAP Explainability Mandatory**: All models must have SHAP files
- ✅ **Model Metadata Required**: All models must have `metadata.json`
- ✅ **Model Signing Required**: All models must be cryptographically signed
- ✅ **Reject Missing SHAP**: Models without SHAP are rejected
- ✅ **Reject Missing Metadata**: Models without metadata are rejected

### Model Format Support
- `.pkl` (Pickle format)
- `.gguf` (GGUF format)
- `.onnx` (ONNX format)
- `.pt` / `.pth` (PyTorch format)

### Training Data Governance
- ✅ **No Customer Data**: Training data must be synthetic or red-team only
- ✅ **Reproducibility**: Fixed random seed (42) required
- ✅ **Signed Training Artifacts**: All training artifacts must be signed

---

## 6️⃣ Database Design

**Phase 0 does NOT use a database.** It operates on:
- **File system**: Scans source code files
- **YAML configuration**: Reads `rules.yaml` specification
- **In-memory state**: Tracks violations during scan

### No Database Tables
N/A - Phase 0 is a static analysis tool, not a database-backed service.

---

## 7️⃣ Inter-Phase Connectivity & Trust Boundaries

### Inputs
- **Source Code**: Scans all `.py`, `.rs`, `.yaml`, `.sh`, `.service` files
- **Guardrails Specification**: Reads `rules.yaml` (signed Ed25519)
- **Module Resolver**: Validates module existence (prevents phantom modules)

### Outputs
- **Violation Reports**: Emitted to stderr before exit
- **Audit Logs**: Written to logs (if configured)
- **Exit Codes**: Non-zero on violation (fail-closed)

### Trust Boundaries
- ✅ **Fail-Closed**: Any violation prevents build/service start
- ✅ **No Bypass**: No environment variable or flag can bypass enforcement
- ✅ **Cryptographic Verification**: `rules.yaml` must be signed and valid
- ✅ **Phantom Module Rejection**: References to non-existent modules cause immediate failure

### Dependencies
- **ransomeye_trust**: Cryptographic signing/verification
- **ransomeye_retention**: Retention policy validation
- **Python 3.10+**: Runtime requirement

---

## 8️⃣ UI / Dashboards / SOC Visibility

**Phase 0 does NOT provide a UI or dashboard.** It is a command-line tool.

### Visibility Mechanisms
- **Console Output**: Violations printed to stderr
- **CI/CD Integration**: Results visible in CI pipeline logs
- **Audit Logs**: Violations logged (if logging configured)

---

## 9️⃣ Copilot / AI Assistant

**Phase 0 does NOT provide a copilot or AI assistant.** It is an enforcement tool.

---

## 🔟 Security Controls & Fail-Closed Behavior

### Fail-Closed Mechanisms

1. **Hardcoded Secret Detection**
   - Pattern: `(?:api[_-]?key|apikey|token|secret|password|passwd|pwd)\s*[=:]\s*["']?[A-Za-z0-9_\-]{16,}["']?`
   - Action: **IMMEDIATE EXIT** (exit code 1)

2. **Hardcoded IP Detection**
   - Pattern: IPv4 addresses (except `127.0.0.1`, `localhost`, `0.0.0.0`)
   - Action: **IMMEDIATE EXIT** (exit code 1)

3. **Hardcoded URL Detection**
   - Pattern: `(?:https?|ftp|ws|wss)://(?!localhost|127\.0\.0\.1)[^\s'"`]+`
   - Action: **IMMEDIATE EXIT** (exit code 1)

4. **Missing File Headers**
   - Required for: `.py`, `.yaml`, `.yml`, `.json`, `.sh`, `.service`, `.tsx`, `.ts`, `.rs`
   - Action: **IMMEDIATE EXIT** (exit code 1)

5. **Phantom Module References**
   - Detection: Module referenced in code but doesn't exist on disk
   - Action: **IMMEDIATE EXIT** (exit code 1)

6. **Missing ML SHAP Files**
   - Detection: Model exists but SHAP file missing
   - Action: **IMMEDIATE EXIT** (exit code 1)

7. **Invalid Guardrails Specification**
   - Detection: `rules.yaml` signature invalid or tampered
   - Action: **IMMEDIATE EXIT** (exit code 1)

### Security Properties

- ✅ **No Bypass**: No environment variable or flag can bypass enforcement
- ✅ **Cryptographic Integrity**: `rules.yaml` signed with Ed25519
- ✅ **Audit Trail**: All violations logged before exit
- ✅ **Deterministic**: Same input always produces same result

---

## 1️⃣1️⃣ Operational Reality (Restart, Rollback, Crash Safety)

### Restart Behavior
- **No persistent state**: Guardrails run stateless
- **No service to restart**: Command-line tool only
- **No crash recovery needed**: Tool completes and exits

### Rollback Capability
- **N/A**: Phase 0 is a validation tool, not a service
- **Specification rollback**: `rules.yaml` can be reverted to previous version (if signed)

### Crash Safety
- **No data loss risk**: No persistent state
- **Fail-fast**: Exits immediately on violation
- **No partial state**: Either passes or fails completely

---

## 1️⃣2️⃣ Known Gaps & Residual Risks

### Known Gaps

1. **No Real-Time Monitoring**
   - **Gap**: Guardrails only run on-demand (CLI) or in CI/CD
   - **Risk**: Code changes between CI runs may violate rules
   - **Mitigation**: Pre-commit hooks recommended

2. **Limited Pattern Coverage**
   - **Gap**: Regex patterns may miss sophisticated obfuscation
   - **Risk**: Advanced hardcoding may evade detection
   - **Mitigation**: AST parsing helps but not foolproof

3. **No Historical Tracking**
   - **Gap**: No database of past violations
   - **Risk**: Cannot track violation trends over time
   - **Mitigation**: CI/CD logs provide historical record

### Residual Risks

1. **False Negatives**
   - **Risk**: Some violations may not be detected
   - **Impact**: Security gaps may persist
   - **Mitigation**: Regular manual audits recommended

2. **Specification Drift**
   - **Risk**: `rules.yaml` may become outdated
   - **Impact**: New violation types may not be caught
   - **Mitigation**: Regular specification review

---

## 1️⃣3️⃣ Recommendations

### Immediate Actions
1. ✅ **Deploy pre-commit hooks** to catch violations before commit
2. ✅ **Integrate with CI/CD** (already done via `ci/global_guardrails.yml`)
3. ✅ **Regular specification review** to keep `rules.yaml` current

### Future Enhancements
1. **Real-time monitoring**: Continuous file system watching
2. **Historical database**: Track violations over time
3. **Enhanced pattern detection**: Machine learning for obfuscation detection

---

## 1️⃣4️⃣ Final Verdict

**PRODUCTION-VIABLE**

Phase 0 is **fully implemented**, **operational**, and **production-ready**. All core components exist, are functional, and enforce security invariants correctly. The fail-closed mechanism works as designed, and the guardrails specification is cryptographically signed and validated.

**Deployment Readiness**: ✅ **READY FOR PRODUCTION**

**Security Posture**: ✅ **STRONG** - Fail-closed enforcement prevents security violations

**Operational Maturity**: ✅ **MATURE** - Well-tested, documented, and integrated

---

**Generated:** 2025-01-27  
**Format:** Forensic-grade technical validation  
**Purpose:** Authoritative documentation for security audit, regulator review, and architect handover
