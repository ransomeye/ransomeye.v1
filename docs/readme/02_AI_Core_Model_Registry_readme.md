# Phase 2 — AI Core & Model Registry

**Path and File Name:** `/home/ransomeye/rebuild/docs/readme/02_AI_Core_Model_Registry_readme.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Comprehensive technical audit and documentation for Phase 2 - AI Core & Model Registry

---

## 1️⃣ Phase Overview

### Purpose
Phase 2 establishes the **architectural foundation** and **AI model registry infrastructure** for RansomEye. It includes:
- **System Architecture** (`ransomeye_architecture`): Zero-trust, plane-separated architecture definitions
- **AI Model Registry** (`core/ai`): Model registry infrastructure (partially overlaps with Phase 8)
- **Baseline Intelligence Pack** (`ransomeye_intelligence/baseline_pack`): Pre-trained baseline models (partially overlaps with Phase 3)

**Note**: Phase 2 functionality is distributed across multiple modules. The AI Core functionality is primarily implemented in `core/ai` (Phase 8 - AI Advisory), and baseline models are in `ransomeye_intelligence` (Phase 3).

### Security Objective
- **Zero-trust architecture** with explicit trust boundaries
- **Plane separation** (Data, Control, Intelligence, Management)
- **AI non-authority guarantee** (advisory-only)
- **Model registry** with signature verification
- **Baseline Intelligence Pack** with signed, pre-trained models

### Role in Architecture
Phase 2 defines the **constitutional architecture** that all other phases must conform to. It establishes:
- Trust boundaries and data flows
- Plane separation (Data, Control, Intelligence, Management)
- Identity model and authentication
- Data contracts between planes
- AI/ML governance principles

---

## 2️⃣ Implementation Status

| Component | Status | Notes |
|-----------|--------|-------|
| System Architecture (`ransomeye_architecture`) | ✅ Fully Implemented | Zero-trust architecture definitions |
| AI Model Registry (`core/ai/registry`) | ✅ Fully Implemented | Part of Phase 8 (AI Advisory) |
| Baseline Models (`ransomeye_intelligence/baseline_pack`) | ✅ Fully Implemented | Part of Phase 3 (Intelligence) |
| Architecture Documentation | ✅ Fully Implemented | Complete plane and trust boundary docs |
| Model Registry (Rust) | ✅ Fully Implemented | Located in `core/ai/src/registry/` |
| Model Signing & Verification | ✅ Fully Implemented | RSA-4096 signature verification |
| Baseline Model Training | ✅ Fully Implemented | Synthetic and red-team data only |

---

## 3️⃣ File & Folder Structure

### Architecture Documentation (`ransomeye_architecture/`)
`/home/ransomeye/rebuild/ransomeye_architecture/`

**Key Files:**
- **`README.md`**: Architecture overview
- **`planes/data_plane.md`**: Data Plane definition
- **`planes/control_plane.md`**: Control Plane definition
- **`planes/intelligence_plane.md`**: Intelligence Plane definition
- **`planes/management_plane.md`**: Management Plane definition
- **`trust/forbidden_flows.md`**: Forbidden data flows
- **`identity/component_identity.md`**: Identity model
- **`data_contracts/`**: Data contracts between planes

### AI Model Registry (`core/ai/src/registry/`)
`/home/ransomeye/rebuild/core/ai/src/registry/`

**Key Files:**
- **`registry.rs`**: Model registry implementation
- **`verification.rs`**: Model signature verification
- **`rollback.rs`**: Model rollback capabilities

**Note**: This is part of Phase 8 (AI Advisory) but provides model registry functionality.

### Baseline Intelligence Pack (`ransomeye_intelligence/baseline_pack/`)
`/home/ransomeye/rebuild/ransomeye_intelligence/baseline_pack/`

**Key Files:**
- **`train_baseline_models.py`**: Baseline model training script
- **`models/model_manifest.json`**: Model manifest with metadata
- **`models/`**: Trained model files (`.pkl` format)
- **`shap/`**: SHAP explainability files
- **`incremental_update.py`**: Incremental model update support
- **`metadata/training_manifest.json`**: Training provenance

**Note**: This is part of Phase 3 (Intelligence) but provides baseline models for Phase 2.

---

## 4️⃣ Modules & Services

### Modules

1. **System Architecture** (`ransomeye_architecture`)
   - **Responsibility**: Defines zero-trust architecture and trust boundaries
   - **Runtime Behavior**: Documentation and validation tooling (no runtime service)
   - **systemd Integration**: ❌ NO
   - **Installer Integration**: ❌ NO

2. **ModelRegistry** (`core/ai/src/registry/registry.rs`)
   - **Responsibility**: Manages signed baseline models with verification
   - **Runtime Behavior**: Loads and verifies models from configured directory
   - **systemd Integration**: ✅ YES (part of Phase 8 service)
   - **Installer Integration**: ❌ NO

3. **Baseline Model Training** (`ransomeye_intelligence/baseline_pack/train_baseline_models.py`)
   - **Responsibility**: Trains baseline models using synthetic and red-team data
   - **Runtime Behavior**: Build-time training script (not runtime service)
   - **systemd Integration**: ❌ NO
   - **Installer Integration**: ❌ NO

### Services

**NO dedicated systemd service for Phase 2** - Phase 2 is primarily architectural definitions and infrastructure. Model registry functionality is part of Phase 8 (AI Advisory) service.

---

## 5️⃣ AI / ML / LLM DETAILS

### Baseline Models

**Location**: `/home/ransomeye/rebuild/ransomeye_intelligence/baseline_pack/models/`

#### Model 1: Ransomware Behavior Classifier
- **Model Name**: `ransomware_behavior.model`
- **Type**: ML / Classifier
- **Algorithm**: Random Forest
- **Training Status**: ✅ Fully trained
- **Training Scripts Present**: ✅ YES (`train_baseline_models.py`)
- **Incremental Learning Support**: ✅ YES (`incremental_update.py`)
- **SHAP Explainability Present**: ✅ YES (`shap/ransomware_behavior_shap.json`)
- **Model Signing & Verification**: ✅ YES (RSA-4096 signatures)
- **Data Sources**: Synthetic data + Red-team exercises only
- **Features**: 256 features
- **Classes**: benign, ransomware, suspicious
- **Accuracy**: 99.405%
- **Metadata**: `model_manifest.json`

#### Model 2: Anomaly Baseline
- **Model Name**: `anomaly_baseline.model`
- **Type**: ML / Anomaly Detector
- **Algorithm**: Isolation Forest
- **Training Status**: ✅ Fully trained
- **Training Scripts Present**: ✅ YES (`train_baseline_models.py`)
- **Incremental Learning Support**: ⚠️ PARTIAL
- **SHAP Explainability Present**: ✅ YES (`shap/anomaly_baseline_shap.json`)
- **Model Signing & Verification**: ✅ YES
- **Data Sources**: Synthetic data only
- **Features**: 128 features
- **Contamination**: 0.01
- **Metadata**: `model_manifest.json`

#### Model 3: Confidence Calibration
- **Model Name**: `confidence_calibration.model`
- **Type**: ML / Calibration
- **Algorithm**: CalibratedClassifierCV
- **Training Status**: ✅ Fully trained
- **Training Scripts Present**: ✅ YES (`train_baseline_models.py`)
- **Incremental Learning Support**: ❌ NO
- **SHAP Explainability Present**: ✅ YES
- **Model Signing & Verification**: ✅ YES
- **Data Sources**: Synthetic data only
- **Metadata**: `model_manifest.json`

### Model Registry

**Implementation**: Rust-based model registry in `core/ai/src/registry/`

**Features:**
- Model loading and verification
- Signature validation (RSA-4096)
- Model hash verification
- Baseline model management
- Rollback capabilities

**Supported Model Formats:**
- `.pkl` (Pickle format for Python models)
- Model manifest validation required
- SHAP file validation required

### Training Methodology

**Training Data Sources:**
1. **Synthetic Data**: Algorithmically generated ransomware behavior patterns
2. **Red Team Exercises**: Controlled, authorized penetration testing data

**Training Principles:**
- ✅ No customer data used
- ✅ No production data used
- ✅ Fully reproducible (fixed random seed: 42)
- ✅ Signed training artifacts
- ✅ SHAP explainability mandatory

---

## 6️⃣ SOC Copilot / AI Copilot

**NOT PRESENT IN PHASE 2** - SOC Copilot functionality is in Phase 8 (AI Advisory) and Phase 7.

**Related Functionality:**
- Model registry supports AI Advisory (Phase 8) which includes SOC Copilot
- Baseline models support Intelligence Plane advisory functions

---

## 7️⃣ Database Design

**NOT APPLICABLE** - Phase 2 does not use a database. Model metadata is stored in JSON manifests and model files are stored on filesystem.

**Storage:**
- **Model Files**: Filesystem (`ransomeye_intelligence/baseline_pack/models/`)
- **Model Metadata**: JSON manifests (`model_manifest.json`)
- **SHAP Files**: Filesystem (`ransomeye_intelligence/baseline_pack/shap/`)
- **Signatures**: Filesystem (signature files alongside models)

---

## 8️⃣ Ports & Interconnectivity

**NO NETWORK PORTS** - Phase 2 architecture definitions and model registry do not expose network ports.

**Interconnectivity:**
- **Model Registry**: Reads models from configured directory (ENV: `RANSOMEYE_AI_MODELS_DIR`)
- **Architecture Validation**: Static validation tooling (no runtime connectivity)
- **Model Loading**: File-based model loading (no network access)

---

## 9️⃣ UI / Dashboards / Frontend

**NO UI IN THIS PHASE** - Phase 2 provides architecture definitions and model infrastructure only.

**Documentation:**
- Architecture documentation in `ransomeye_architecture/`
- Model metadata in JSON manifests

---

## 🔟 Logging, Metrics & Observability

### Logs Generated
**MINIMAL** - Phase 2 is primarily infrastructure:
- Model registry logs (via Phase 8 service): Model loading and verification events
- Training logs: Training script output (build-time)

### Log Formats
- **Model Registry**: Structured logging via Rust tracing
- **Training Scripts**: Console output and log files

### Metrics Exposed
**NO METRICS** - Phase 2 does not expose metrics endpoints.

### Prometheus/Grafana Integration
**NOT APPLICABLE**

### Audit Logs
- **Model Loading**: Logged via Phase 8 service (if enabled)
- **Model Verification**: Signature validation events logged

### Tamper-Proofing
- **Model Signing**: All models cryptographically signed (RSA-4096)
- **Signature Verification**: Models verified on load
- **Hash Verification**: Model hashes verified against manifest

---

## 1️⃣1️⃣ Security & Compliance

### Fail-Closed Enforcement
✅ **FULLY ENFORCED**
- Missing baseline models → AI disabled
- Invalid signatures → Models rejected
- Missing SHAP files → Models rejected
- Architecture violations → Runtime enforcement (via other phases)

### Cryptographic Controls
✅ **ENFORCED**
- RSA-4096 model signatures
- Model hash verification
- Manifest signature validation
- Signature verification on model load

### Signature Verification
✅ **ENFORCED**
- All models must be signed
- Signatures verified on load
- Unsigned models rejected

### Zero-Trust Enforcement
✅ **FULLY ENFORCED**
- Zero-trust architecture definitions
- Explicit trust boundaries
- Plane separation enforced
- AI non-authority guaranteed

### STIG Hardening Status
**NOT APPLICABLE** - Phase 2 is architecture definitions and infrastructure, not a runtime service.

---

## 1️⃣2️⃣ CI / Validation / Testing

### Tests Present
✅ **YES** - Located in:
- `/home/ransomeye/rebuild/ransomeye_architecture/tests/`
- `/home/ransomeye/rebuild/core/ai/tests/`

### Test Coverage
- **Forbidden Flow Tests**: Validates architecture boundary enforcement
- **Identity Violation Tests**: Validates identity model
- **AI Authority Violation Tests**: Validates AI non-authority
- **Plane Isolation Tests**: Validates plane separation
- **Model Registry Tests**: Validates model loading and verification
- **Signature Tests**: Validates signature verification

### Synthetic Data Generation
✅ **YES** - Baseline models trained using synthetic data generator

### CI Workflows
✅ **YES** - Architecture and model registry tests run in CI pipelines

### Validation Coverage
✅ **COMPREHENSIVE**
- Architecture definitions validated
- Model registry validated
- Signature verification validated
- Baseline model training validated

---

## 1️⃣3️⃣ Known Gaps & Technical Debt

### Missing Components
**NONE IDENTIFIED** - Phase 2 implementation appears complete.

### Partial Implementations
**NONE IDENTIFIED**

### Design Risks
1. **Phase Overlap**: Phase 2 functionality overlaps with Phase 3 (Intelligence) and Phase 8 (AI Advisory)
   - **Status**: Functional overlap exists but is acceptable
   - **Recommendation**: Consider clearer phase boundaries in documentation

2. **Model Format Limitation**: Currently supports `.pkl` format only
   - **Status**: Sufficient for current needs
   - **Recommendation**: Consider support for additional formats (`.gguf`, `.onnx`) if needed

---

## 1️⃣4️⃣ Recommendations

### Refactors
1. **Phase Boundaries**: Consider clearer documentation of phase boundaries to reduce confusion about where functionality resides.

2. **Model Format Support**: Evaluate need for additional model format support based on future requirements.

### Missing Enforcement
**NONE IDENTIFIED** - Current enforcement appears comprehensive.

### Architectural Fixes
**NONE IDENTIFIED** - Architecture is sound and well-defined.

### Training Improvements
1. **Incremental Learning**: Expand incremental learning support for anomaly baseline model.

2. **Training Data Diversity**: Consider expanding training data sources while maintaining synthetic/red-team only policy.

### Security Hardening
1. **Model Encryption**: Consider encrypting model files at rest in addition to signatures.

2. **Revocation Lists**: Ensure model revocation lists are properly integrated and validated.

---

## Summary

Phase 2 (AI Core & Model Registry) establishes the **constitutional architecture** and **model infrastructure** for RansomEye. While functionality is distributed across multiple modules (architecture definitions, model registry in Phase 8, baseline models in Phase 3), the phase successfully establishes zero-trust architecture principles, model registry infrastructure with signature verification, and baseline intelligence models with full SHAP explainability. All models are fully trained, signed, and validated.

**Status**: ✅ **PRODUCTION READY**

**Note**: Phase 2 functionality is distributed across:
- Architecture definitions: `ransomeye_architecture/`
- Model registry: `core/ai/src/registry/` (Phase 8)
- Baseline models: `ransomeye_intelligence/baseline_pack/` (Phase 3)

---

**Last Updated**: 2025-01-27  
**Validation Status**: ✅ All components validated and tested

