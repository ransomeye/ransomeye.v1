# Phase 1 — Core Engine & Installer

**Path and File Name:** `/home/ransomeye/rebuild/docs/readme/01_Core_Engine_Installer_readme.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Forensic-grade technical validation and truth audit for Phase 1 - Core Engine & Installer

---

## 1️⃣ Phase Purpose & Security Objective

### Purpose
Phase 1 provides the **unified installer and uninstaller** for all RansomEye master-core modules. It is the **ONLY supported installation mechanism** for RansomEye. No component may run outside this installer.

### Security Objective
- **Fail-closed installation** with rollback capability
- **EULA enforcement** (mandatory, no bypass)
- **Cryptographic identity generation** (RSA-4096 per installation)
- **Systemd service generation** (unified `/home/ransomeye/rebuild/systemd/`)
- **Phantom module detection** (prevents installation of non-existent modules)
- **Installation state management** (signed `install_state.json`)

### Role in Architecture
Phase 1 is the **foundational installation layer** that all subsequent phases depend on. It:
- Validates prerequisites (OS, disk, swap, clock)
- Enforces EULA acceptance
- Configures retention policies
- Generates cryptographic identity
- Creates systemd service units
- Manages installation state

---

## 2️⃣ Implementation Status (What Exists vs What Doesn't)

| Component | Status | Location | Notes |
|-----------|--------|----------|-------|
| Python Installer | ✅ **FULLY IMPLEMENTED** | `ransomeye_installer/installer.py` | Main orchestrator |
| Rust Operations Tool | ✅ **FULLY IMPLEMENTED** | `ops/tuner/` (ransomeye_operations) | Service lifecycle management |
| Root Install Script | ✅ **FULLY IMPLEMENTED** | `/home/ransomeye/rebuild/install.sh` | Entry point wrapper |
| Root Uninstall Script | ✅ **FULLY IMPLEMENTED** | `/home/ransomeye/rebuild/uninstall.sh` | Uninstallation entry point |
| State Manager | ✅ **FULLY IMPLEMENTED** | `ransomeye_installer/state_manager.py` | Install state management |
| Module Resolver | ✅ **FULLY IMPLEMENTED** | `ransomeye_installer/module_resolver.py` | Phantom module detection |
| Systemd Writer | ✅ **FULLY IMPLEMENTED** | `ransomeye_installer/services/systemd_writer.py` | Service unit generation |
| EULA Enforcement | ✅ **FULLY IMPLEMENTED** | `ransomeye_installer/eula/EULA.txt` | Mandatory EULA acceptance |
| Prerequisites Check | ✅ **FULLY IMPLEMENTED** | `ransomeye_installer/system/` | OS, disk, swap, clock validation |
| Retention Configuration | ✅ **FULLY IMPLEMENTED** | `ransomeye_installer/retention/` | Retention policy setup |
| Identity Generator | ✅ **FULLY IMPLEMENTED** | `ransomeye_installer/crypto/identity_generator.py` | RSA-4096 key generation |
| Manifest Generator | ✅ **FULLY IMPLEMENTED** | `ransomeye_installer/manifest_generator.py` | Installation manifest |

**CRITICAL FINDING: Phase 1 is FULLY IMPLEMENTED and operational. All core components exist and are functional.**

---

## 3️⃣ File & Folder Structure (Absolute Paths)

### Root Installer Directory
`/home/ransomeye/rebuild/ransomeye_installer/`

### Core Files
```
ransomeye_installer/
├── __init__.py
├── installer.py                    # Main orchestrator
├── state_manager.py                 # Install state management
├── module_resolver.py               # Phantom module detection
├── manifest_generator.py             # Installation manifest
├── eula/
│   └── EULA.txt                     # End User License Agreement
├── system/
│   ├── os_check.py                  # OS validation
│   ├── disk_check.py                # Disk space validation
│   ├── swap_check.py                # Swap space validation
│   └── clock_check.py               # Clock synchronization check
├── retention/
│   ├── retention_writer.py          # Writes retention.txt
│   └── retention_validator.py      # Validates retention config
├── crypto/
│   ├── identity_generator.py        # Generates RSA-4096 identity
│   └── keystore.py                  # Secure key storage
├── services/
│   └── systemd_writer.py            # Generates systemd units
└── docs/
    ├── installer_flow.md
    ├── failure_modes.md
    └── README.md
```

### Root-Level Scripts
- `/home/ransomeye/rebuild/install.sh` - Installation entry point
- `/home/ransomeye/rebuild/uninstall.sh` - Uninstallation entry point

### Rust Operations Tool
`/home/ransomeye/rebuild/ops/tuner/` (ransomeye_operations)
- Service lifecycle management (start/stop/restart/status)
- Service dependency resolution
- Systemd integration

---

## 4️⃣ Runtime Components & Services

### Installation Flow

**Canonical Invocation:**
```bash
cd /home/ransomeye/rebuild
sudo ./install.sh
```

**Alternative (Python module):**
```bash
cd /home/ransomeye/rebuild
sudo python3 -m ransomeye_installer.installer
```

### Installation Steps

1. **Prerequisites Validation**
   - OS check (Ubuntu >=22.04 OR RHEL >=8)
   - Disk space check (minimum 10GB)
   - Swap space check (>= 16GB OR equal to RAM)
   - Clock synchronization check (NTP)

2. **EULA Acceptance**
   - Displays EULA from `ransomeye_installer/eula/EULA.txt`
   - Requires explicit "yes" acceptance
   - **NO BYPASS** - fails-closed if not accepted

3. **Retention Configuration**
   - Prompts for telemetry retention (months) [default: 6]
   - Prompts for forensic retention (days) [default: 10]
   - Prompts for disk max usage percent [default: 80]
   - Writes to `/home/ransomeye/rebuild/config/retention.txt`

4. **Cryptographic Identity Generation**
   - Generates RSA-4096 key pair
   - Creates unique installation identity hash
   - Stores keys securely

5. **Systemd Unit Generation**
   - Generates service units for all core modules
   - Writes to `/home/ransomeye/rebuild/systemd/`
   - All services **DISABLED by default**

6. **Installation State Creation**
   - Creates signed `install_state.json`
   - Includes EULA acceptance, retention config, identity
   - Required for services to start

7. **Manifest Generation**
   - Generates installation manifest
   - Lists all installed modules
   - Includes version information

### Uninstallation Flow

**Canonical Invocation:**
```bash
cd /home/ransomeye/rebuild
sudo ./uninstall.sh
```

**Steps:**
1. Verifies install state is valid
2. Stops all services
3. Removes services, configs, and optionally evidence
4. Preserves evidence by default (unless `--remove-evidence` specified)

### Service Lifecycle Management

**Rust Operations Tool:**
```bash
# Start all services
ransomeye_operations start

# Start specific service
ransomeye_operations start ransomeye-core

# Stop all services
ransomeye_operations stop

# Restart all services
ransomeye_operations restart

# Check status
ransomeye_operations status
```

---

## 5️⃣ AI / ML / LLM Reality

**Phase 1 does NOT contain AI/ML/LLM models.** It is an installation tool.

### No AI/ML/LLM Components
N/A - Phase 1 is infrastructure, not AI/ML/LLM.

---

## 6️⃣ Database Design

**Phase 1 does NOT use a database.** It operates on:
- **File system**: Writes configuration files, systemd units
- **JSON state file**: `install_state.json` (signed)
- **In-memory state**: Tracks installation progress

### No Database Tables
N/A - Phase 1 is an installation tool, not a database-backed service.

---

## 7️⃣ Inter-Phase Connectivity & Trust Boundaries

### Inputs
- **Source Code**: Scans module directories for existence
- **Guardrails Specification**: Validates against `ransomeye_guardrails/rules.yaml`
- **EULA File**: Reads from `ransomeye_installer/eula/EULA.txt`

### Outputs
- **Systemd Units**: Writes to `/home/ransomeye/rebuild/systemd/`
- **Retention Config**: Writes to `/home/ransomeye/rebuild/config/retention.txt`
- **Install State**: Writes signed `install_state.json`
- **Manifest**: Writes installation manifest

### Trust Boundaries
- ✅ **Fail-Closed**: Any validation failure prevents installation
- ✅ **No Bypass**: EULA acceptance cannot be bypassed
- ✅ **Phantom Module Rejection**: References to non-existent modules cause immediate failure
- ✅ **Cryptographic Identity**: Each installation has unique RSA-4096 identity

### Dependencies
- **ransomeye_guardrails**: Validates modules exist (prevents phantom modules)
- **ransomeye_trust**: Cryptographic signing/verification
- **ransomeye_retention**: Retention policy validation
- **Python 3.10+**: Runtime requirement

---

## 8️⃣ UI / Dashboards / SOC Visibility

**Phase 1 does NOT provide a UI or dashboard.** It is a command-line tool.

### Visibility Mechanisms
- **Console Output**: Installation progress printed to stdout
- **Log Files**: Installation logs written to `/var/log/ransomeye/install.log`
- **Install State**: Signed JSON file tracks installation state

---

## 9️⃣ Copilot / AI Assistant

**Phase 1 does NOT provide a copilot or AI assistant.** It is an installation tool.

---

## 🔟 Security Controls & Fail-Closed Behavior

### Fail-Closed Mechanisms

1. **Prerequisites Validation Failure**
   - Action: **IMMEDIATE EXIT** (exit code 1)
   - Prevents installation on unsupported systems

2. **EULA Not Accepted**
   - Action: **IMMEDIATE EXIT** (exit code 1)
   - **NO BYPASS** - no environment variable or flag can bypass

3. **Phantom Module Detection**
   - Detection: Module referenced but doesn't exist on disk
   - Action: **IMMEDIATE EXIT** (exit code 1)

4. **Invalid Retention Configuration**
   - Action: **IMMEDIATE EXIT** (exit code 1)

5. **Identity Generation Failure**
   - Action: **IMMEDIATE EXIT** (exit code 1)

6. **Systemd Unit Generation Failure**
   - Action: **IMMEDIATE EXIT** (exit code 1)

### Security Properties

- ✅ **No Bypass**: EULA acceptance cannot be bypassed
- ✅ **Cryptographic Identity**: Each installation has unique RSA-4096 identity
- ✅ **Signed State**: `install_state.json` is cryptographically signed
- ✅ **Phantom Module Prevention**: Detects and rejects non-existent modules

---

## 1️⃣1️⃣ Operational Reality (Restart, Rollback, Crash Safety)

### Restart Behavior
- **No persistent service**: Installer runs once and exits
- **Install state persists**: `install_state.json` tracks installation state
- **Services disabled by default**: Must be manually enabled after installation

### Rollback Capability
- **Uninstaller available**: `uninstall.sh` removes installation
- **Evidence preservation**: Evidence preserved by default (unless `--remove-evidence`)
- **Secure deletion**: Optional 3-pass overwrite for sensitive data

### Crash Safety
- **Atomic operations**: Installation steps are atomic where possible
- **State validation**: Install state validated before uninstallation
- **Partial install detection**: Uninstaller detects and handles partial installations

---

## 1️⃣2️⃣ Known Gaps & Residual Risks

### Known Gaps

1. **No Upgrade Path**
   - **Gap**: Installer does not support upgrades (only fresh installs)
   - **Risk**: Upgrades require uninstall + reinstall
   - **Mitigation**: Documented in failure modes

2. **No Network Validation**
   - **Gap**: Installer does not validate network connectivity
   - **Risk**: Services may fail if network unavailable
   - **Mitigation**: Services handle network failures gracefully

3. **Limited Rollback Granularity**
   - **Gap**: Rollback is all-or-nothing (no partial rollback)
   - **Risk**: Cannot rollback individual modules
   - **Mitigation**: Documented limitation

### Residual Risks

1. **Installation State Corruption**
   - **Risk**: `install_state.json` may become corrupted
   - **Impact**: Uninstallation may fail
   - **Mitigation**: State validation before uninstallation

2. **Service Dependency Deadlock**
   - **Risk**: Circular dependencies may prevent service startup
   - **Impact**: Services may not start correctly
   - **Mitigation**: Dependency resolution in operations tool

---

## 1️⃣3️⃣ Recommendations

### Immediate Actions
1. ✅ **Deploy installer** as only supported installation mechanism
2. ✅ **Document EULA acceptance** requirement (no bypass)
3. ✅ **Enable services manually** after installation (disabled by default)

### Future Enhancements
1. **Upgrade support**: Add upgrade path for existing installations
2. **Network validation**: Validate network connectivity during installation
3. **Partial rollback**: Support rolling back individual modules

---

## 1️⃣4️⃣ Final Verdict

**PRODUCTION-VIABLE**

Phase 1 is **fully implemented**, **operational**, and **production-ready**. All core components exist, are functional, and enforce security invariants correctly. The installer provides fail-closed installation with EULA enforcement, phantom module detection, and cryptographic identity generation.

**Deployment Readiness**: ✅ **READY FOR PRODUCTION**

**Security Posture**: ✅ **STRONG** - Fail-closed enforcement prevents security violations

**Operational Maturity**: ✅ **MATURE** - Well-tested, documented, and integrated

---

**Generated:** 2025-01-27  
**Format:** Forensic-grade technical validation  
**Purpose:** Authoritative documentation for security audit, regulator review, and architect handover
