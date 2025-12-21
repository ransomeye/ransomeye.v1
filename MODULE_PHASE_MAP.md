# RansomEye Module-Phase Mapping

**Path and File Name:** `/home/ransomeye/rebuild/MODULE_PHASE_MAP.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Canonical mapping of all RansomEye modules to phases, types, and operational requirements

---

## Overview

This document provides the canonical mapping of all modules in the RansomEye repository to their phases, types, and operational requirements. This mapping resolves all name mismatches and ambiguities identified in the operational audit.

---

## Module Directory

### Phase 0: Global Guardrails

| Module | Directory | Type | Installer | Service | Notes |
|--------|-----------|------|-----------|---------|-------|
| `ransomeye_guardrails` | `ransomeye_guardrails` | tool | ❌ No | ❌ No | Static analysis and guardrail enforcement tool |

---

### Phase 1: Core Engine & Installer

| Module | Directory | Type | Installer | Service | Notes |
|--------|-----------|------|-----------|---------|-------|
| `ransomeye_installer` | `ransomeye_installer` | tool | ❌ No | ❌ No | Python-based installer. Root-level `install.sh` wrapper required |
| `ransomeye_operations` | `ransomeye_operations` | tool | ❌ No | ❌ No | Rust-based installer/uninstaller/lifecycle. Root-level `install.sh` wrapper required |

---

### Phase 2: AI Core & Model Registry

| Module | Directory | Type | Installer | Service | Notes |
|--------|-----------|------|-----------|---------|-------|
| `ransomeye_ai_core` | ❌ **NOT FOUND** | service | ✅ Yes | ✅ Yes | **SPECIFIED IN CODE BUT DOES NOT EXIST**. May map to `ransomeye_ai_advisory` (Phase 8) or needs creation |
| `ransomeye_architecture` | `ransomeye_architecture` | library | ❌ No | ❌ No | Architecture documentation and design specifications |

---

### Phase 3: Alert Engine & Policy Manager

| Module | Directory | Type | Installer | Service | Notes |
|--------|-----------|------|-----------|---------|-------|
| `ransomeye_alert_engine` | ❌ **NOT FOUND** | service | ✅ Yes | ✅ Yes | **SPECIFIED IN CODE BUT DOES NOT EXIST**. Functionality split between `ransomeye_intelligence` and `ransomeye_policy` |
| `ransomeye_intelligence` | `ransomeye_intelligence` | service | ✅ Yes | ✅ Yes | Intelligence Plane with baseline ML models. Service in wrong location |

---

### Phase 4: KillChain & Forensic Dump / Event Ingestion

| Module | Directory | Type | Installer | Service | Notes |
|--------|-----------|------|-----------|---------|-------|
| `ransomeye_ingestion` | `ransomeye_ingestion` | service | ✅ Yes | ✅ Yes | Core Event Ingestion & Deterministic Backpressure. Service in wrong location |
| `ransomeye_forensic` | ❌ **NOT FOUND** | service | ✅ Yes | ✅ Yes | **SPECIFIED IN CODE BUT DOES NOT EXIST**. May be part of `ransomeye_reporting` |
| `ransomeye_killchain_core` | ❌ **NOT FOUND** | service | ✅ Yes | ✅ Yes | **SPECIFIED IN CODE BUT DOES NOT EXIST**. Maps to `ransomeye_correlation` (Phase 5) |

---

### Phase 5: LLM Summarizer / Correlation Engine

| Module | Directory | Type | Installer | Service | Notes |
|--------|-----------|------|-----------|---------|-------|
| `ransomeye_correlation` | `ransomeye_correlation` | service | ✅ Yes | ✅ Yes | Deterministic correlation engine and kill-chain inference. Service in wrong location |
| `ransomeye_threat_correlation` | ❌ **NOT FOUND** | service | ✅ Yes | ✅ Yes | **SPECIFIED IN CODE BUT DOES NOT EXIST**. Maps to `ransomeye_correlation` |
| `ransomeye_llm` | ❌ **NOT FOUND** | service | ✅ Yes | ✅ Yes | **SPECIFIED IN CODE BUT DOES NOT EXIST**. Needs creation |
| `ransomeye_incident_summarizer` | ❌ **NOT FOUND** | service | ✅ Yes | ✅ Yes | **SPECIFIED IN CODE BUT DOES NOT EXIST**. Needs creation or merge with `ransomeye_llm` |

---

### Phase 6: Incident Response & Playbooks / Policy Engine

| Module | Directory | Type | Installer | Service | Notes |
|--------|-----------|------|-----------|---------|-------|
| `ransomeye_policy` | `ransomeye_policy` | service | ✅ Yes | ✅ Yes | Policy Engine & Enforcement Semantics. Service in wrong location |
| `ransomeye_response` | ❌ **NOT FOUND** | service | ✅ Yes | ✅ Yes | **SPECIFIED IN CODE BUT DOES NOT EXIST**. Needs creation or handled by `ransomeye_enforcement` |

---

### Phase 7: SOC Copilot / Enforcement Dispatcher

| Module | Directory | Type | Installer | Service | Notes |
|--------|-----------|------|-----------|---------|-------|
| `ransomeye_enforcement` | `ransomeye_enforcement` | service | ✅ Yes | ✅ Yes | Enforcement Dispatcher & Safety Guards. Service in wrong location |

---

### Phase 8: Threat Correlation Engine / AI Advisory

| Module | Directory | Type | Installer | Service | Notes |
|--------|-----------|------|-----------|---------|-------|
| `ransomeye_ai_advisory` | `ransomeye_ai_advisory` | service | ✅ Yes | ✅ Yes | AIML Inference, Explainability & Analyst Assistance (SOC Copilot) |

---

### Phase 9: Network Scanner

| Module | Directory | Type | Installer | Service | Notes |
|--------|-----------|------|-----------|---------|-------|
| `ransomeye_net_scanner` | ❌ **NOT FOUND** | service | ✅ Yes | ✅ Yes | **SPECIFIED IN CODE BUT DOES NOT EXIST**. Needs creation |

---

### Phase 10: DB Core / Reporting

| Module | Directory | Type | Installer | Service | Notes |
|--------|-----------|------|-----------|---------|-------|
| `ransomeye_db_core` | ❌ **NOT FOUND** | service | ✅ Yes | ✅ Yes | **SPECIFIED IN CODE BUT DOES NOT EXIST**. PostgreSQL integration may be library-based |
| `ransomeye_reporting` | `ransomeye_reporting` | service | ✅ Yes | ✅ Yes | Reporting, Forensics & Evidence Preservation. Service in wrong location |

---

### Phase 11: UI & Dashboards

| Module | Directory | Type | Installer | Service | Notes |
|--------|-----------|------|-----------|---------|-------|
| `ransomeye_ui` | ❌ **NOT FOUND** | service | ✅ Yes | ✅ Yes | **SPECIFIED IN CODE BUT DOES NOT EXIST**. Needs creation |

---

### Phase 12: Orchestrator (Master Flow) / Validation

| Module | Directory | Type | Installer | Service | Notes |
|--------|-----------|------|-----------|---------|-------|
| `ransomeye_master_core` | ❌ **NOT FOUND** | service | ✅ Yes | ✅ Yes | **SPECIFIED IN CODE BUT DOES NOT EXIST**. Needs creation or part of `ransomeye_operations` |
| `ransomeye_validation` | `ransomeye_validation` | tool | ❌ No | ❌ No | Validation, Stress, Security & Release Gate |

---

### Phase 18: Threat Intelligence Feed Engine

| Module | Directory | Type | Installer | Service | Notes |
|--------|-----------|------|-----------|---------|-------|
| `ransomeye_threat_intel_engine` | ❌ **NOT FOUND** | service | ✅ Yes | ✅ Yes | **SPECIFIED IN CODE BUT DOES NOT EXIST**. Maps to `ransomeye_intelligence` (Phase 3) |

---

### Phase 19: HNMP Engine

| Module | Directory | Type | Installer | Service | Notes |
|--------|-----------|------|-----------|---------|-------|
| `ransomeye_hnmp_engine` | ❌ **NOT FOUND** | service | ✅ Yes | ✅ Yes | **SPECIFIED IN CODE BUT DOES NOT EXIST**. Needs creation |

---

### Phase 21: Linux Agent (Standalone)

| Module | Directory | Type | Installer | Service | Notes |
|--------|-----------|------|-----------|---------|-------|
| `ransomeye_linux_agent` | `ransomeye_linux_agent` | standalone_product | ⚠️ **MISSING** | ⚠️ **MISSING** | **CRITICAL**: Standalone agent but LACKS installer, uninstaller, and service definition |

---

### Phase 22: Windows Agent (Standalone)

| Module | Directory | Type | Installer | Service | Notes |
|--------|-----------|------|-----------|---------|-------|
| `ransomeye_windows_agent` | `ransomeye_windows_agent` | standalone_product | ⚠️ **MISSING** | ⚠️ **MISSING** | **CRITICAL**: Standalone agent but LACKS installer, uninstaller, and Windows service definition. Requires MSI installer |

---

### Phase 23: DPI Probe (Standalone)

| Module | Directory | Type | Installer | Service | Notes |
|--------|-----------|------|-----------|---------|-------|
| `ransomeye_dpi_probe` | `ransomeye_dpi_probe` | standalone_product | ⚠️ **MISSING** | ⚠️ **MISSING** | **CRITICAL**: Standalone probe but LACKS installer, uninstaller, and service definition |

---

### Supporting Infrastructure Libraries

| Module | Directory | Type | Installer | Service | Notes |
|--------|-----------|------|-----------|---------|-------|
| `ransomeye_retention` | `ransomeye_retention` | library | ❌ No | ❌ No | Data retention and disk management library |
| `ransomeye_trust` | `ransomeye_trust` | library | ❌ No | ❌ No | Cryptographic signing and verification library |

---

## Service Location Resolution

### Services That Need Relocation

The following services exist in `/home/ransomeye/rebuild/ransomeye_operations/systemd/` but **MUST** be moved to the unified `/home/ransomeye/rebuild/systemd/` directory:

1. `core.service` → `systemd/ransomeye-core.service` (module: UNKNOWN)
2. `ingestion.service` → `systemd/ransomeye-ingestion.service` (module: `ransomeye_ingestion`)
3. `correlation.service` → `systemd/ransomeye-correlation.service` (module: `ransomeye_correlation`)
4. `policy.service` → `systemd/ransomeye-policy.service` (module: `ransomeye_policy`)
5. `enforcement.service` → `systemd/ransomeye-enforcement.service` (module: `ransomeye_enforcement`)
6. `intelligence.service` → `systemd/ransomeye-intelligence.service` (module: `ransomeye_intelligence`)
7. `reporting.service` → `systemd/ransomeye-reporting.service` (module: `ransomeye_reporting`)

---

## Module Name Resolution

### Missing Modules Requiring Resolution

The following modules are specified in `ransomeye_installer/services/systemd_writer.py` but do not exist:

| Specified Name | Phase | Resolution |
|----------------|-------|------------|
| `ransomeye_ai_core` | 2 | May map to `ransomeye_ai_advisory` (Phase 8) or needs creation |
| `ransomeye_alert_engine` | 3 | Functionality split between `ransomeye_intelligence` and `ransomeye_policy` |
| `ransomeye_db_core` | 10 | PostgreSQL integration may be library-based or needs service module |
| `ransomeye_forensic` | 4 | Functionality may be part of `ransomeye_reporting` (Phase 10) |
| `ransomeye_hnmp_engine` | 19 | **Needs creation** |
| `ransomeye_incident_summarizer` | 5 | Needs creation or merge with `ransomeye_llm` |
| `ransomeye_killchain_core` | 4 | Maps to `ransomeye_correlation` (includes kill-chain functionality) |
| `ransomeye_llm` | 5 | **Needs creation** or may be part of `ransomeye_ai_advisory` |
| `ransomeye_master_core` | 12 | Needs creation or may be part of `ransomeye_operations` |
| `ransomeye_net_scanner` | 9 | **Needs creation** |
| `ransomeye_response` | 6 | Needs creation or may be handled by `ransomeye_enforcement` |
| `ransomeye_threat_correlation` | 8 | Maps to `ransomeye_correlation` (Phase 5) |
| `ransomeye_threat_intel_engine` | 18 | Maps to `ransomeye_intelligence` (includes threat intelligence) |
| `ransomeye_ui` | 11 | **Needs creation** |

---

## Audit Summary

| Metric | Count |
|--------|-------|
| **Total Directories** | 17 |
| **Existing Modules** | 17 |
| **Missing Modules** | 14 |
| **Modules with Wrong Service Location** | 7 |
| **Standalone Modules Missing Installers** | 3 |

### Critical Issues

1. **3 Standalone Modules Lack Installers**: `ransomeye_linux_agent`, `ransomeye_windows_agent`, `ransomeye_dpi_probe`
2. **14 Modules Specified But Don't Exist**: Listed in `systemd_writer.py` but no corresponding directories
3. **7 Services in Wrong Location**: Must be moved from `ransomeye_operations/systemd/` to unified `systemd/`

---

## Resolution Status

**PARTIAL** - Many modules need creation or name resolution. The canonical mapping is established, but significant work remains to align the codebase with the specification.

---

**Generated:** 2025-01-27  
**Format:** Canonical YAML + Human-readable Markdown  
**Purpose:** Resolve all ambiguities and establish single source of truth for module-phase mapping

