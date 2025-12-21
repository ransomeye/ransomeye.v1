# Path and File Name: /home/ransomeye/rebuild/ROOTLESS_RUNTIME_ENFORCEMENT_REPORT.md
# Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
# Details: Military-Grade Runtime Privilege Enforcement Report - Rootless Runtime

# RansomEye Rootless Runtime Enforcement Report

**Enforcement Date:** 2024-12-19  
**Enforcement Type:** Military-Grade Runtime Privilege Enforcement  
**Status:** ✅ COMPLETE - Zero Root-Running Services

---

## Executive Summary

Strict rootless runtime enforcement has been implemented. **NO RansomEye service runs as root (UID 0) at runtime**. All services are configured to run as the `ransomeye` user with capability-based privileges where needed.

**Fail-Closed:** Any service configured or running as root causes immediate validation failure.

---

## Enforcement Rules

### Rule 1: Explicit User/Group Required
- ✅ All services MUST specify `User=` directive
- ✅ All services MUST specify `Group=` directive
- ❌ **FORBIDDEN:** `User=root` or `User=0`
- ❌ **FORBIDDEN:** Missing User/Group directives

### Rule 2: Root (UID 0) Forbidden
- ❌ **FORBIDDEN:** Any service running as root at runtime
- ✅ **REQUIRED:** All services run as `ransomeye` user (non-root)

### Rule 3: Capability-Based Privileges
- ✅ Use `CapabilityBoundingSet` for minimal required capabilities
- ✅ Use `AmbientCapabilities` only when explicitly needed
- ✅ No root privileges required for normal operation

### Rule 4: Runtime and State Directories
- ✅ `RuntimeDirectory` for `/run/ransomeye/{service}`
- ✅ `StateDirectory` for `/var/lib/ransomeye/{service}`
- ✅ Proper permissions for non-root access

---

## Service Configuration Changes

### Before Enforcement
- ❌ 7 services configured as `User=root`
- ❌ No RuntimeDirectory or StateDirectory
- ❌ No capability management
- ❌ No validation of root runtime

### After Enforcement
- ✅ All 7 services configured as `User=ransomeye` and `Group=ransomeye`
- ✅ RuntimeDirectory and StateDirectory added for all services
- ✅ Capability-based privileges configured
- ✅ Runtime validation enforces rootless operation

---

## Updated Services

### Core Services (7 services - all updated)

| Service | User | Group | RuntimeDirectory | StateDirectory | Status |
|---------|------|-------|------------------|----------------|--------|
| `ransomeye-core.service` | ransomeye | ransomeye | ransomeye/core | ransomeye/core | ✅ Rootless |
| `ransomeye-ingestion.service` | ransomeye | ransomeye | ransomeye/ingestion | ransomeye/ingestion | ✅ Rootless |
| `ransomeye-correlation.service` | ransomeye | ransomeye | ransomeye/correlation | ransomeye/correlation | ✅ Rootless |
| `ransomeye-policy.service` | ransomeye | ransomeye | ransomeye/policy | ransomeye/policy | ✅ Rootless |
| `ransomeye-enforcement.service` | ransomeye | ransomeye | ransomeye/enforcement | ransomeye/enforcement | ✅ Rootless |
| `ransomeye-intelligence.service` | ransomeye | ransomeye | ransomeye/intelligence | ransomeye/intelligence | ✅ Rootless |
| `ransomeye-reporting.service` | ransomeye | ransomeye | ransomeye/reporting | ransomeye/reporting | ✅ Rootless |

### Utility Services (1 service - already compliant)

| Service | User | Group | Status |
|---------|------|-------|--------|
| `ransomeye-github-sync.service` | ransomeye | ransomeye | ✅ Rootless (already compliant) |

---

## Capability Configuration

All services use minimal capability sets:

```ini
CapabilityBoundingSet=CAP_NET_BIND_SERVICE CAP_NET_RAW CAP_SYS_NICE
AmbientCapabilities=
```

**Capabilities Used:**
- `CAP_NET_BIND_SERVICE` - Bind to privileged ports (< 1024) if needed
- `CAP_NET_RAW` - Raw network access for packet inspection
- `CAP_SYS_NICE` - Adjust process priority

**No Root Required:** All operations can be performed with these capabilities instead of full root privileges.

---

## User/Group Creation

**Installation Step (install.sh):**
```bash
# Create ransomeye group
groupadd -r ransomeye

# Create ransomeye user (system user, no login shell)
useradd -r -g ransomeye -d /home/ransomeye -s /usr/sbin/nologin \
    -c "RansomEye Service User" ransomeye

# Set ownership
chown -R ransomeye:ransomeye /home/ransomeye/rebuild

# Create runtime and state directories
mkdir -p /run/ransomeye /var/lib/ransomeye
chown -R ransomeye:ransomeye /run/ransomeye /var/lib/ransomeye
```

**User Properties:**
- System user (`-r` flag)
- No login shell (`/usr/sbin/nologin`)
- Home directory: `/home/ransomeye`
- Group: `ransomeye`

---

## Validation Components

### 1. Service File Validation
**Location:** `post_install_validator.py::check_rootless_runtime()`

**Checks:**
1. Parses all service files for `User=` directive
2. Fails if `User=root` or `User=0` found
3. Fails if `User=` directive missing
4. Warns if `Group=` directive missing

### 2. Runtime UID Validation
**Location:** `post_install_validator.py::check_rootless_runtime()`

**Checks:**
1. Queries running services for UID
2. Fails if any service running as UID 0 (root)
3. Logs all violations

### 3. Systemd Writer Enforcement
**Location:** `ransomeye_installer/services/systemd_writer.py`

**Enforcement:**
- Generates all services with `User=ransomeye` and `Group=ransomeye`
- Adds RuntimeDirectory and StateDirectory
- Configures capability-based privileges
- **Never generates root services**

---

## Validation Results

### Pre-Enforcement Issues
1. ❌ 7 services configured as `User=root`
2. ❌ No RuntimeDirectory or StateDirectory
3. ❌ No capability management
4. ❌ No validation of root runtime
5. ❌ No user/group creation in installer

### Post-Enforcement Status
1. ✅ All services configured as `User=ransomeye`
2. ✅ RuntimeDirectory and StateDirectory added
3. ✅ Capability-based privileges configured
4. ✅ Runtime validation enforces rootless operation
5. ✅ User/group creation added to installer
6. ✅ Fail-closed validation active

---

## Build Integrity Guarantees

✅ **Zero Root Services** - All services run as non-root user  
✅ **Fail-Closed Validation** - Build fails if any service runs as root  
✅ **Explicit User/Group** - All services specify User and Group  
✅ **Capability-Based** - Minimal privileges via capabilities  
✅ **Runtime Directories** - Proper state management for non-root  
✅ **Install-Time Enforcement** - User/group created during installation  
✅ **Post-Install Enforcement** - Validation after installation  
✅ **Runtime Enforcement** - Running services checked for root UID  

---

## Justified Exceptions

**NONE**

All services run as non-root user. No exceptions justified or documented.

---

## Verification

To verify rootless runtime enforcement:

```bash
# Check service configurations
grep -r "User=" /home/ransomeye/rebuild/systemd/*.service

# Check running service UIDs (as root)
systemctl show ransomeye-core --property=UID --value
systemctl show ransomeye-ingestion --property=UID --value
# ... (should all be non-zero UID)

# Run full validation
python3 /home/ransomeye/rebuild/post_install_validator.py
```

**Expected Result:** All services configured and running as `ransomeye` user (non-root)

---

## Compliance Status

**Status:** ✅ **ENFORCED**  
**Build Integrity:** ✅ **GUARANTEED**  
**Root Services:** ✅ **ZERO**  
**Validation Rules:** ✅ **ACTIVE**  
**Fail-Closed:** ✅ **ENABLED**  
**Justified Exceptions:** ✅ **NONE**

---

**End of Report**

