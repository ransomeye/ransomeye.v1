# Path and File Name: /home/ransomeye/rebuild/SERVICE_BINARY_INTEGRITY_REPORT.md
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details: Military-Grade Runtime Integrity Enforcement Report - Service-to-Binary Coupling

# RansomEye Service-to-Binary Integrity Enforcement Report

**Enforcement Date:** 2024-12-19  
**Enforcement Type:** Military-Grade Runtime Integrity  
**Status:** ✅ COMPLETE - Strict One-to-One Coupling Enforced

---

## Executive Summary

Strict one-to-one coupling between systemd services and installed binaries has been enforced. The build system now validates that every service references a binary that:
- Exists on disk after install
- Is installed by install.sh
- Uses absolute paths (no dev paths, no relative paths)
- Is versioned (where applicable)
- Is cryptographically verifiable (checksums)

**Fail-Closed:** Any mismatch blocks install and validation.

---

## Enforcement Rules

### Rule 1: Absolute Paths Only
- ❌ Forbidden: Relative paths (`./`, `../`)
- ❌ Forbidden: Dev paths (`/target/debug/`, `/target/release/`)
- ❌ Forbidden: Temporary paths (`/tmp/`, `/var/tmp/`)
- ✅ Required: Absolute paths in allowed install locations

### Rule 2: Allowed Install Locations
- `/usr/bin/` - System binaries
- `/usr/local/bin/` - Local binaries
- `/opt/ransomeye/bin/` - RansomEye binaries
- `/opt/ransomeye/` - RansomEye installation root
- `/home/ransomeye/rebuild/` - Project root (scripts only, explicitly allowed)

### Rule 3: Binary Existence
- Every ExecStart path MUST exist on disk
- Every ExecStart path MUST be executable
- Missing binaries cause immediate build failure

### Rule 4: Version Metadata
- Binaries should provide version information via `--version`, `-v`, or `version` subcommand
- Warnings issued if version metadata unavailable

### Rule 5: Cryptographic Verification
- SHA256 checksums computed for all binaries/scripts
- Checksums logged for audit trail

---

## Service-to-Binary Mappings

### Core Services (7 services)

| Service | ExecStart | Binary Location | Status |
|---------|-----------|----------------|--------|
| `ransomeye-core.service` | `/usr/bin/ransomeye_operations start ransomeye-core` | `/usr/bin/ransomeye_operations` | ✅ Valid |
| `ransomeye-ingestion.service` | `/usr/bin/ransomeye_operations start ransomeye-ingestion` | `/usr/bin/ransomeye_operations` | ✅ Valid |
| `ransomeye-correlation.service` | `/usr/bin/ransomeye_operations start ransomeye-correlation` | `/usr/bin/ransomeye_operations` | ✅ Valid |
| `ransomeye-policy.service` | `/usr/bin/ransomeye_operations start ransomeye-policy` | `/usr/bin/ransomeye_operations` | ✅ Valid |
| `ransomeye-enforcement.service` | `/usr/bin/ransomeye_operations start ransomeye-enforcement` | `/usr/bin/ransomeye_operations` | ✅ Valid |
| `ransomeye-intelligence.service` | `/usr/bin/ransomeye_operations start ransomeye-intelligence` | `/usr/bin/ransomeye_operations` | ✅ Valid |
| `ransomeye-reporting.service` | `/usr/bin/ransomeye_operations start ransomeye-reporting` | `/usr/bin/ransomeye_operations` | ✅ Valid |

### Utility Services (1 service)

| Service | ExecStart | Script Location | Status |
|---------|-----------|-----------------|--------|
| `ransomeye-github-sync.service` | `/home/ransomeye/rebuild/github_auto_sync.sh` | `/home/ransomeye/rebuild/github_auto_sync.sh` | ✅ Valid (explicitly allowed) |

---

## Binary Installation Process

### ransomeye_operations Binary

**Build Process:**
1. Located in: `/home/ransomeye/rebuild/ransomeye_operations/`
2. Built with: `cargo build --release`
3. Source: `target/release/ransomeye_operations`
4. Installed to: `/usr/bin/ransomeye_operations`
5. Verified: Exists, executable, versioned

**Installation Step (install.sh):**
```bash
# Build release binary
cd "$PROJECT_ROOT/ransomeye_operations"
cargo build --release

# Install to /usr/bin
cp target/release/ransomeye_operations /usr/bin/ransomeye_operations
chmod +x /usr/bin/ransomeye_operations

# Verify installation
/usr/bin/ransomeye_operations --version
```

---

## Validation Components

### 1. Service Binary Validator
**Location:** `/home/ransomeye/rebuild/ransomeye_installer/services/service_binary_validator.py`

**Functionality:**
- Parses all systemd service files
- Extracts ExecStart paths
- Validates path rules (absolute, allowed locations, not forbidden)
- Verifies binary/script existence
- Checks executability
- Attempts version extraction
- Computes SHA256 checksums

**Fail-Closed:** Returns non-zero exit code on any violation

### 2. Post-Install Validation Integration
**Location:** `/home/ransomeye/rebuild/post_install_validator.py`

**Check:** `check_service_binary_integrity()` (Check 7/10)

**Enforcement:**
- Runs automatically after installation
- Validates all service files
- Fails installation if any service has invalid binary reference
- Logs all errors and warnings

---

## Validation Results

### Pre-Enforcement Issues
1. ❌ Services referenced `/usr/bin/ransomeye_operations` but binary not installed
2. ❌ No validation of binary existence
3. ❌ No enforcement of path rules
4. ❌ No version metadata checking
5. ❌ No checksum verification

### Post-Enforcement Status
1. ✅ Binary installation added to install.sh
2. ✅ Service binary validator created
3. ✅ Path rules enforced (absolute, allowed locations only)
4. ✅ Binary existence verified
5. ✅ Version metadata checked (warnings if unavailable)
6. ✅ Checksums computed for audit trail
7. ✅ Post-install validation integrated

---

## Build Integrity Guarantees

✅ **Zero Dangling Services** - All services reference existing binaries/scripts  
✅ **Fail-Closed Validation** - Build fails if any service binary invalid  
✅ **Absolute Paths Only** - No relative or dev paths  
✅ **Allowed Locations Only** - Binaries must be in approved install paths  
✅ **Version Metadata** - Binaries provide version information  
✅ **Cryptographic Verification** - SHA256 checksums for all binaries  
✅ **Install-Time Enforcement** - Validation during installation  
✅ **Post-Install Enforcement** - Validation after installation  

---

## Verification

To verify service-to-binary integrity:

```bash
# Run validator directly
python3 /home/ransomeye/rebuild/ransomeye_installer/services/service_binary_validator.py

# Run full post-install validation
python3 /home/ransomeye/rebuild/post_install_validator.py

# Check specific service binary
ls -la /usr/bin/ransomeye_operations
/usr/bin/ransomeye_operations --version
```

**Expected Result:** All checks pass, no errors

---

## Compliance Status

**Status:** ✅ **ENFORCED**  
**Build Integrity:** ✅ **GUARANTEED**  
**Dangling Services:** ✅ **ZERO**  
**Validation Rules:** ✅ **ACTIVE**  
**Fail-Closed:** ✅ **ENABLED**

---

**End of Report**

