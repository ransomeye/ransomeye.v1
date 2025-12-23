# Phase 1 — Core Engine & Installer

**Path and File Name:** `/home/ransomeye/rebuild/docs/readme/01_Core_Engine_Installer_readme.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Comprehensive technical audit and documentation for Phase 1 - Core Engine & Installer

---

## 1️⃣ Phase Overview

### Purpose
Phase 1 provides the **unified installation and lifecycle management** infrastructure for RansomEye. It includes both a Python-based installer (`ransomeye_installer`) and a Rust-based operations module (`ransomeye_operations` in `ops/tuner`) that handle installation, configuration, service lifecycle, and uninstallation.

### Security Objective
- **Fail-closed installation** with mandatory EULA acceptance
- **Cryptographic identity generation** for installation instances
- **Signed install state** validation before service startup
- **Retention policy configuration** during installation
- **Systemd service unit generation** for all core modules
- **Clean uninstallation** with evidence preservation options

### Role in Architecture
Phase 1 is the **entry point** for all RansomEye deployments. It:
- Validates system prerequisites
- Enforces EULA acceptance (mandatory, no bypass)
- Configures data retention policies
- Generates cryptographic identities
- Creates systemd service units (disabled by default)
- Manages service lifecycle (start/stop/restart/status)
- Provides uninstallation with clean state removal

---

## 2️⃣ Implementation Status

| Component | Status | Notes |
|-----------|--------|-------|
| Python Installer (`ransomeye_installer`) | ✅ Fully Implemented | Main orchestrator for installation flow |
| Rust Operations (`ops/tuner`) | ✅ Fully Implemented | Service lifecycle management |
| Prerequisites Validation | ✅ Fully Implemented | OS, disk, swap, clock checks |
| EULA Enforcement | ✅ Fully Implemented | Mandatory acceptance, fail-closed |
| Retention Configuration | ✅ Fully Implemented | Configurable retention policies |
| Identity Generation | ✅ Fully Implemented | RSA-4096 key pair generation |
| Systemd Unit Generation | ✅ Fully Implemented | Unified systemd/ directory |
| Install State Management | ✅ Fully Implemented | Signed state with validation |
| Service Lifecycle | ✅ Fully Implemented | Start/stop/restart/status |
| Uninstaller | ✅ Fully Implemented | Clean removal with options |
| Root-level install.sh | ✅ Fully Implemented | Wrapper script for Python installer |
| Root-level uninstall.sh | ✅ Fully Implemented | Wrapper script for uninstallation |

---

## 3️⃣ File & Folder Structure

### Python Installer (`ransomeye_installer/`)
`/home/ransomeye/rebuild/ransomeye_installer/`

**Key Files:**
- **`installer.py`**: Main orchestrator for installation flow
- **`state_manager.py`**: Install state management and signing
- **`system/os_check.py`**: OS compatibility validation
- **`system/disk_check.py`**: Disk space validation
- **`system/swap_check.py`**: Swap space validation
- **`system/clock_check.py`**: Clock synchronization check
- **`retention/retention_writer.py`**: Writes retention.txt configuration
- **`retention/retention_validator.py`**: Validates retention configuration
- **`crypto/identity_generator.py`**: Generates installation identity
- **`crypto/keystore.py`**: Secure key storage
- **`services/systemd_writer.py`**: Generates systemd unit files
- **`services/lifecycle.py`**: Service lifecycle management (Python)

### Rust Operations (`ops/tuner/`)
`/home/ransomeye/rebuild/ops/tuner/`

**Key Files:**
- **`src/main.rs`**: CLI entry point for operations commands
- **`src/lib.rs`**: Library root with public exports
- **`src/installer/install.rs`**: Rust installer implementation
- **`src/installer/preflight.rs`**: Preflight checks
- **`src/installer/retention.rs`**: Retention configuration
- **`src/installer/crypto.rs`**: Cryptographic identity generation
- **`src/installer/state.rs`**: Install state management
- **`src/uninstaller/uninstall.rs`**: Uninstallation logic
- **`src/uninstaller/cleanup.rs`**: Cleanup operations
- **`src/uninstaller/verification.rs`**: Uninstall verification
- **`src/lifecycle/start.rs`**: Service start logic
- **`src/lifecycle/stop.rs`**: Service stop logic
- **`src/lifecycle/restart.rs`**: Service restart logic
- **`src/lifecycle/status.rs`**: Service status checking

### Root-Level Scripts
- **`install.sh`**: Root-level installation wrapper (ONLY supported method)
- **`uninstall.sh`**: Root-level uninstallation wrapper

### Configuration Files
- **`ransomeye_installer/eula/EULA.txt`**: End User License Agreement
- **`config/retention.txt`**: Data retention policy (generated during installation)
- **`ransomeye_installer/config/install_state.json`**: Signed installation state

### Documentation
- **`ransomeye_installer/docs/installer_flow.md`**: Complete installation flow
- **`ransomeye_installer/docs/failure_modes.md`**: Failure modes and recovery
- **`ransomeye_installer/docs/upgrade_policy.md`**: Upgrade procedures
- **`ops/tuner/docs/operations_guide.md`**: Operations guide
- **`ops/tuner/docs/uninstall_procedure.md`**: Uninstall procedure
- **`ops/tuner/docs/upgrade_procedure.md`**: Upgrade procedure

---

## 4️⃣ Modules & Services

### Modules

1. **RansomEyeInstaller** (`installer.py`)
   - **Responsibility**: Main orchestrator for installation flow
   - **Runtime Behavior**: Coordinates all installation steps
   - **systemd Integration**: ❌ NO (generates systemd units, but doesn't run as service)
   - **Installer Integration**: ✅ YES (this IS the installer)

2. **StateManager** (`state_manager.py`)
   - **Responsibility**: Install state management and signing
   - **Runtime Behavior**: Creates and signs install_state.json
   - **systemd Integration**: ❌ NO
   - **Installer Integration**: ✅ YES

3. **SystemdWriter** (`services/systemd_writer.py`)
   - **Responsibility**: Generates systemd unit files
   - **Runtime Behavior**: Writes service units to unified systemd/ directory
   - **systemd Integration**: ❌ NO (generates units but doesn't run as service)
   - **Installer Integration**: ✅ YES

4. **Operations CLI** (`ops/tuner/src/main.rs`)
   - **Responsibility**: CLI interface for service lifecycle management
   - **Runtime Behavior**: Handles start/stop/restart/status commands
   - **systemd Integration**: ✅ YES (manages systemd services)
   - **Installer Integration**: ❌ NO (post-install tool)

### Services

**NO systemd service for Phase 1 itself** - Phase 1 generates systemd units for other modules but does not run as a service.

**Generated Services:**
Phase 1 generates systemd units in `/home/ransomeye/rebuild/systemd/` for:
- `ransomeye-core.service`
- `ransomeye-ingestion.service`
- `ransomeye-correlation.service`
- `ransomeye-policy.service`
- `ransomeye-enforcement.service`
- `ransomeye-intelligence.service`
- `ransomeye-reporting.service`
- `ransomeye-posture-engine.service`
- `ransomeye-linux-agent.service` (standalone)
- `ransomeye-dpi-probe.service` (standalone)
- `ransomeye-sentinel.service` (standalone)
- `ransomeye-feed-fetcher.service` (timer)
- `ransomeye-feed-retraining.service` (timer)
- `ransomeye-github-sync.service` (timer)

**Note**: All generated services are **DISABLED by default** and require:
1. Valid `install_state.json`
2. EULA acceptance logged in state
3. Retention configuration
4. Identity generation

---

## 5️⃣ AI / ML / LLM DETAILS

**NOT APPLICABLE** - Phase 1 is infrastructure for installation and lifecycle management, not an AI/ML module.

---

## 6️⃣ SOC Copilot / AI Copilot

**NOT PRESENT** - Phase 1 does not include AI Copilot functionality.

---

## 7️⃣ Database Design

**NOT APPLICABLE** - Phase 1 does not use a database. Install state is stored in signed JSON files.

**State Storage:**
- **`install_state.json`**: Located at `/home/ransomeye/rebuild/ransomeye_installer/config/install_state.json`
  - Contains: version, EULA acceptance, retention config, identity info, state
  - **Signed**: Cryptographic signature validation required
  - **Validation**: Services check state validity before startup

---

## 8️⃣ Ports & Interconnectivity

**NO NETWORK PORTS** - Phase 1 installer and operations tools do not expose network ports.

**Interconnectivity:**
- **System Checks**: Reads system information (OS, disk, swap, clock)
- **State Management**: Reads/writes `install_state.json`
- **Configuration**: Reads/writes `config/retention.txt`
- **Systemd Integration**: Generates service units, manages service lifecycle via systemctl
- **EULA**: Reads from `ransomeye_installer/eula/EULA.txt`

---

## 9️⃣ UI / Dashboards / Frontend

**NO UI IN THIS PHASE** - Phase 1 uses command-line interfaces only.

**CLI Interfaces:**
1. **Python Installer**: 
   - Interactive prompts for EULA and retention configuration
   - Console output with installation progress

2. **Rust Operations**:
   - CLI commands: `install`, `uninstall`, `start`, `stop`, `restart`, `status`
   - Console output for all operations

3. **Root Scripts**:
   - `install.sh`: Interactive installation with EULA display
   - `uninstall.sh`: Command-line uninstallation with options

---

## 🔟 Logging, Metrics & Observability

### Logs Generated
- **Installation Log**: `/var/log/ransomeye/install.log` (via install.sh)
- **Uninstallation Log**: `/var/log/ransomeye/uninstall.log` (via uninstall.sh)
- **Console Output**: Installation progress and error messages
- **State Logging**: EULA acceptance and configuration logged in install_state.json

### Log Formats
- **Installation Log**: Timestamped entries with status messages
- **Console Output**: Human-readable progress indicators and error messages
- **State JSON**: Structured JSON with cryptographic signature

### Metrics Exposed
**NO METRICS** - Phase 1 does not expose metrics endpoints.

### Prometheus/Grafana Integration
**NOT APPLICABLE**

### Audit Logs
- **Installation Audit**: EULA acceptance, retention configuration, and identity generation logged in install_state.json
- **State Signing**: Cryptographic signature provides tamper-proofing

### Tamper-Proofing
- **Signed State**: install_state.json is cryptographically signed
- **State Validation**: Services validate state signature before startup
- **Fail-Closed**: Missing or invalid state prevents service startup

---

## 1️⃣1️⃣ Security & Compliance

### Fail-Closed Enforcement
✅ **FULLY ENFORCED**
- EULA acceptance mandatory (no bypass)
- Prerequisites validation fails-closed
- Invalid state prevents service startup
- Unsupported OS causes installation abort

### Cryptographic Controls
✅ **ENFORCED**
- RSA-4096 identity key pair generation
- Signed install_state.json
- State signature validation before service startup
- Trust chain validation

### Signature Verification
✅ **ENFORCED**
- install_state.json must be signed and valid
- Services check signature before startup
- State tampering detected and causes failure

### Zero-Trust Enforcement
**PARTIALLY APPLICABLE**
- State validation before service startup
- Cryptographic identity generation per installation
- No hardcoded credentials (ENV-only configuration)

### STIG Hardening Status
**NOT APPLICABLE** - Phase 1 is an installation tool, not a runtime service.

---

## 1️⃣2️⃣ CI / Validation / Testing

### Tests Present
✅ **YES** - Located in:
- `/home/ransomeye/rebuild/ransomeye_installer/tests/`
- `/home/ransomeye/rebuild/ops/tuner/tests/`

### Test Coverage
- **EULA Enforcement Tests**: Validates mandatory EULA acceptance
- **Retention Default Tests**: Validates retention configuration defaults
- **Fail-Closed Tests**: Validates fail-closed behavior
- **Install State Tamper Tests**: Validates state signature validation
- **Lifecycle Control Tests**: Validates service start/stop/restart
- **Clean Uninstall Tests**: Validates clean uninstallation

### Synthetic Data Generation
**NOT APPLICABLE** - Tests use real installation scenarios.

### CI Workflows
✅ **YES** - Installation and lifecycle tests run in CI pipelines.

### Validation Coverage
✅ **COMPREHENSIVE**
- All installation steps tested
- EULA enforcement tested
- State management tested
- Service lifecycle tested
- Uninstallation tested

---

## 1️⃣3️⃣ Known Gaps & Technical Debt

### Missing Components
**NONE IDENTIFIED** - Phase 1 implementation appears complete.

### Partial Implementations
**NONE IDENTIFIED**

### Design Risks
1. **Dual Implementation**: Both Python installer and Rust operations exist
   - **Status**: Both implementations exist and are functional
   - **Recommendation**: Consider consolidating to single implementation (likely Rust) for consistency

2. **State File Location**: install_state.json location might need clarification
   - **Current**: `/home/ransomeye/rebuild/ransomeye_installer/config/install_state.json`
   - **Recommendation**: Consider standardizing state file location

---

## 1️⃣4️⃣ Recommendations

### Refactors
1. **Consolidate Implementations**: Consider consolidating Python installer and Rust operations into single Rust-based implementation for consistency and performance.

2. **Standardize State Location**: Ensure install_state.json location is standardized and documented consistently.

### Missing Enforcement
**NONE IDENTIFIED** - Current enforcement appears comprehensive.

### Architectural Fixes
**NONE IDENTIFIED** - Architecture is sound for installation and lifecycle management.

### Training Improvements
**NOT APPLICABLE** - Phase 1 does not use ML models.

### Security Hardening
1. **State File Permissions**: Ensure install_state.json has appropriate file permissions (600) to prevent unauthorized access.

2. **Key Storage**: Ensure cryptographic keys are stored securely with appropriate permissions.

---

## Summary

Phase 1 (Core Engine & Installer) is **FULLY IMPLEMENTED** and provides comprehensive installation and lifecycle management for RansomEye. The implementation includes both Python-based installer (`ransomeye_installer`) and Rust-based operations (`ops/tuner`), with mandatory EULA acceptance, cryptographic identity generation, signed install state management, systemd service unit generation, and clean uninstallation capabilities. All components use fail-closed mechanisms to ensure secure installation and service management.

**Status**: ✅ **PRODUCTION READY**

---

**Last Updated**: 2025-01-27  
**Validation Status**: ✅ All components validated and tested

