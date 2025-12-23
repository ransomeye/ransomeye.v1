# Phase 5 — Correlation & Decision Engine

**Path and File Name:** `/home/ransomeye/rebuild/docs/readme/05_Correlation_Decision_Engine_readme.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Comprehensive technical audit and documentation for Phase 5 - Correlation & Decision Engine

---

## 1️⃣ Phase Overview

### Purpose
Phase 5 implements the **Correlation & Decision Engine** that processes validated events from the Ingestion Pipeline (Phase 4) and performs deterministic ransomware kill-chain correlation to produce authoritative detection results. It uses a ransomware-specific kill-chain model (NOT generic MITRE) with 10 defined stages and enforces strict invariants to ensure deterministic, fail-closed behavior.

### Security Objective
- **Deterministic correlation** - Same events → same detections (always)
- **Ransomware-specific kill-chain** - 10-stage model optimized for ransomware detection
- **Invariant enforcement** - Fail-closed on invariant violations
- **Bounded memory** - Entity state eviction prevents memory exhaustion
- **Explainability** - Human and machine-readable detection rationales
- **No AI/ML dependencies** - Pure rule-based correlation

### Role in Architecture
Phase 5 is the **Core Control Plane component** that:
- Receives validated events from Ingestion Pipeline (Phase 4)
- Correlates events into ransomware kill-chain stages
- Generates authoritative detection results
- Outputs detection results to Policy Engine (Phase 3) for enforcement decisions
- Maintains bounded entity state with TTL and LRU eviction

---

## 2️⃣ Implementation Status

| Component | Status | Notes |
|-----------|--------|-------|
| Correlation Engine | ✅ Fully Implemented | Main orchestrator |
| Kill-Chain Inferencer | ✅ Fully Implemented | Rule-based stage inference |
| Entity State Manager | ✅ Fully Implemented | Bounded state with eviction |
| Confidence Scorer | ✅ Fully Implemented | Deterministic scoring |
| Invariant Enforcer | ✅ Fully Implemented | Fail-closed enforcement |
| Temporal Correlator | ✅ Fully Implemented | Time-based correlation |
| Entity Graph | ✅ Fully Implemented | Relationship tracking |
| Scheduler | ✅ Fully Implemented | Priority-based scheduling |
| Explainability Generator | ✅ Fully Implemented | Human/machine-readable explanations |
| Event Normalizer | ✅ Fully Implemented | Event to signal conversion |

---

## 3️⃣ File & Folder Structure

### Root Directory
`/home/ransomeye/rebuild/core/engine/`

### Key Files
**Correlation Engine (`correlation/src/`):**
- **`engine.rs`**: Main correlation engine orchestrator
- **`entity_state.rs`**: Bounded entity state management (TTL + LRU eviction)
- **`scoring.rs`**: Deterministic confidence scoring
- **`explainability.rs`**: Explainability artifact generation
- **`invariants.rs`**: Invariant enforcement (fail-closed)
- **`temporal.rs`**: Temporal correlation (event time vs processing time)
- **`graph.rs`**: Entity relationship graph
- **`scheduler.rs`**: Priority-based entity scheduling
- **`errors.rs`**: Error types and handling

**Kill-Chain (`correlation/kill_chain/`):**
- **`stages.rs`**: 10 ransomware-specific kill-chain stages
- **`rules.rs`**: Kill-chain inference rules
- **`transitions.rs`**: Stage transition rules
- **`inference.rs`**: Kill-chain stage inference logic

**Input (`correlation/input/`):**
- **`validated_events.rs`**: Validated event structure from Phase 4
- **`normalization.rs`**: Event to signal normalization

**Output (`correlation/output/`):**
- **`detection_result.rs`**: Detection result structure
- **`confidence.rs`**: Confidence score representation
- **`rationale.rs`**: Detection rationale

**Tests (`correlation/tests/`):**
- **`synthetic_attack_tests.rs`**: Ransomware campaign replay tests
- **`false_positive_tests.rs`**: Legitimate activity handling
- **`ordering_tests.rs`**: Event ordering validation
- **`scale_tests.rs`**: 50k+ entity scale testing
- **`invariant_violation_tests.rs`**: Invariant enforcement tests
- **`determinism_tests.rs`**: Identical input → identical output tests

**Documentation (`docs/`):**
- **`kill_chain_model.md`**: Ransomware kill-chain model documentation
- **`invariants.md`**: Invariant enforcement documentation
- **`performance_model.md`**: Performance and scale model
- **`failure_modes.md`**: Failure mode definitions

---

## 4️⃣ Modules & Services

### Modules

1. **CorrelationEngine** (`engine.rs`)
   - **Responsibility**: Main orchestrator for correlation processing
   - **Runtime Behavior**: Processes validated events, correlates into kill-chain stages, generates detection results
   - **systemd Integration**: ✅ YES (part of `ransomeye-correlation.service`)
   - **Installer Integration**: ✅ YES

2. **KillChainInferencer** (`kill_chain/inference.rs`)
   - **Responsibility**: Rule-based kill-chain stage inference
   - **Runtime Behavior**: Matches signals against kill-chain rules, infers current stage
   - **systemd Integration**: ✅ YES (part of correlation service)
   - **Installer Integration**: ✅ YES

3. **EntityStateManager** (`entity_state.rs`)
   - **Responsibility**: Bounded entity state management with eviction
   - **Runtime Behavior**: Maintains entity state, enforces memory bounds via TTL and LRU eviction
   - **systemd Integration**: ✅ YES (part of correlation service)
   - **Installer Integration**: ✅ YES

4. **ConfidenceScorer** (`scoring.rs`)
   - **Responsibility**: Deterministic confidence scoring
   - **Runtime Behavior**: Calculates confidence from signals using weighted scoring with temporal decay
   - **systemd Integration**: ✅ YES (part of correlation service)
   - **Installer Integration**: ✅ YES

5. **InvariantEnforcer** (`invariants.rs`)
   - **Responsibility**: Enforces correlation invariants (fail-closed)
   - **Runtime Behavior**: Validates invariants, aborts correlation on violation
   - **systemd Integration**: ✅ YES (part of correlation service)
   - **Installer Integration**: ✅ YES

6. **TemporalCorrelator** (`temporal.rs`)
   - **Responsibility**: Temporal correlation (event time vs processing time)
   - **Runtime Behavior**: Correlates events within temporal windows
   - **systemd Integration**: ✅ YES (part of correlation service)
   - **Installer Integration**: ✅ YES

7. **EntityGraph** (`graph.rs`)
   - **Responsibility**: Entity relationship graph
   - **Runtime Behavior**: Tracks relationships between entities
   - **systemd Integration**: ✅ YES (part of correlation service)
   - **Installer Integration**: ✅ YES

8. **EntityScheduler** (`scheduler.rs`)
   - **Responsibility**: Priority-based entity scheduling
   - **Runtime Behavior**: Schedules entities for processing based on priority (Critical, High, Normal, Low)
   - **systemd Integration**: ✅ YES (part of correlation service)
   - **Installer Integration**: ✅ YES

9. **ExplainabilityGenerator** (`explainability.rs`)
   - **Responsibility**: Generates human and machine-readable explanations
   - **Runtime Behavior**: Creates explainability artifacts for detections
   - **systemd Integration**: ✅ YES (part of correlation service)
   - **Installer Integration**: ✅ YES

10. **EventNormalizer** (`input/normalization.rs`)
    - **Responsibility**: Converts validated events to kill-chain signals
    - **Runtime Behavior**: Normalizes event data into signal format for correlation
    - **systemd Integration**: ✅ YES (part of correlation service)
    - **Installer Integration**: ✅ YES

### Services

1. **ransomeye-correlation.service**
   - **Location**: `/home/ransomeye/rebuild/systemd/ransomeye-correlation.service`
   - **Type**: systemd service (simple)
   - **User**: ransomeye (rootless)
   - **Dependencies**: `network.target`, `ransomeye-ingestion.service`
   - **Restart**: always
   - **Status**: ✅ Enabled and running

---

## 5️⃣ AI / ML / LLM DETAILS

**NOT APPLICABLE** - Phase 5 is **purely deterministic rule-based correlation**. No AI/ML/LLM components.

**Hard Rule**: "NO correlation. NO policy. NO AI. NO inference." (applies to ingestion, but correlation is deterministic rule-based only)

**Note**: Phase 5 uses rule-based kill-chain inference, not ML models. All logic is deterministic and replayable.

---

## 6️⃣ SOC Copilot / AI Copilot

**NOT PRESENT** - Phase 5 does not include SOC Copilot functionality.

**Related Functionality:**
- **Explainability**: Detection results include explainability artifacts that may be consumed by SOC Copilot (Phase 8)
- **Detection Results**: Detection results are inputs to Policy Engine (Phase 3) and may be displayed in UI (Phase 11)

---

## 7️⃣ Database Design

**NOT APPLICABLE** - Phase 5 does not use a database directly.

**Storage Mechanisms:**
- **In-Memory Entity State**: Bounded in-memory entity state (`EntityStateManager`)
- **Detection Results**: Output as structured data to Policy Engine (Phase 3)
- **No Persistent Storage**: Entity state is in-memory only with TTL/LRU eviction

**Note**: Detection results may be persisted by downstream modules (e.g., Reporting module, Policy Engine), but Phase 5 does not create or manage database tables. Entity state is intentionally in-memory to ensure bounded memory usage.

---

## 8️⃣ Ports & Interconnectivity

### Network Ports

**NO EXPLICIT NETWORK PORTS** - Phase 5 receives events via internal IPC/API from Ingestion Pipeline.

**Interconnectivity:**
- **Input**: Receives validated events from Ingestion Pipeline (Phase 4)
  - **Protocol**: Internal IPC or API (implementation-specific)
  - **Trust Boundaries**: ✅ YES - Events are pre-validated by Phase 4

- **Output**: Emits detection results to Policy Engine (Phase 3)
  - **Protocol**: Internal IPC or API (implementation-specific)
  - **Trust Boundaries**: ✅ YES - Detection results are authoritative for Policy Engine

### Trust Boundaries Enforced

✅ **FULLY ENFORCED**
- **Event Validation**: Receives only validated events from Phase 4
- **Deterministic Processing**: All processing is deterministic and replayable
- **Invariant Enforcement**: Fail-closed on invariant violations
- **No External Dependencies**: No network calls or external dependencies

### Configuration (Environment Variables)

**Configuration** (via ENV or code defaults):
- Maximum entities: Default 50,000
- Maximum signals per entity: Default 1,000
- Maximum transitions per entity: Default 50
- Entity TTL: Default 3600 seconds (1 hour)
- Temporal window size: Default 300 seconds (5 minutes)
- Maximum events per window: Default 1,000
- Minimum confidence threshold: Default 0.6

**Service Dependencies:**
- Depends on: `ransomeye-ingestion.service` (Phase 4)
- Required by: `ransomeye-policy.service` (Phase 3), `ransomeye-intelligence.service` (Phase 3)

---

## 9️⃣ UI / Dashboards / Frontend

**NO UI IN THIS PHASE** - Phase 5 is a backend correlation service with no user interface.

**Management Interfaces:**
- **Detection Results**: Structured JSON output (consumed by Policy Engine and potentially UI)
- **Logs**: Systemd journal logs

**Note**: UI for viewing correlation results, kill-chain progression, and entity graphs is expected in Phase 11 (UI & Dashboards).

---

## 🔟 Logging, Metrics & Observability

### Logs Generated

**Operational Logs:**
- **Event Processing**: Event correlation processing
- **Detection Generation**: Detection result generation
- **Stage Transitions**: Kill-chain stage transitions
- **Entity State**: Entity state creation, updates, eviction
- **Invariant Violations**: Invariant violation aborts (fail-closed)

**Error Logs:**
- **Correlation Errors**: Correlation processing errors
- **State Management Errors**: Entity state management errors
- **Invariant Violations**: Invariant enforcement failures

### Log Formats

**Structured Logging:**
- **Framework**: Rust tracing framework
- **Format**: Structured logs with fields (timestamp, level, module, message, context)
- **Output**: Systemd journal (via `StandardOutput=journal`, `StandardError=journal`)

### Metrics Exposed

**NO METRICS ENDPOINTS** - Phase 5 does not expose metrics endpoints (Prometheus, etc.).

**Internal Metrics (Not Exposed):**
- Entities processed per second
- Detections generated per second
- Average entity confidence scores
- Stage transition rates
- Entity eviction rates
- Invariant violation counts

**Recommendation**: Add Prometheus metrics endpoints for operational observability.

### Prometheus/Grafana Integration

**NOT IMPLEMENTED** - No Prometheus metrics endpoints exposed.

### Audit Logs

✅ **YES** - Critical events logged:
- **Detection Generation**: All detection results logged (if audit enabled)
- **Invariant Violations**: All invariant violations logged (fail-closed behavior)
- **Stage Transitions**: Stage transitions logged (if enabled)

**Format**: Structured logs via Rust tracing framework
**Location**: Systemd journal

### Tamper-Proofing

✅ **ENFORCED**
- **Deterministic Processing**: All processing is deterministic and replayable
- **Invariant Enforcement**: Fail-closed on invariant violations
- **Bounded State**: Memory bounds enforced via TTL and LRU eviction
- **No External Dependencies**: No network calls or external dependencies that could be tampered with

---

## 1️⃣1️⃣ Security & Compliance

### Fail-Closed Enforcement

✅ **FULLY ENFORCED**
- **Invariant Violations** → ABORT_CORRELATION + audit log
- **Memory Bounds Exceeded** → Entity eviction (fail-safe, not fail-closed)
- **Invalid Events** → Event rejection (events pre-validated by Phase 4)
- **State Corruption** → Correlation error + audit log

### Cryptographic Controls

**NOT APPLICABLE** - Phase 5 does not perform cryptographic operations. Events are pre-validated and signed by Phase 4.

### Signature Verification

**NOT APPLICABLE** - Signature verification is performed by Phase 4 (Ingestion Pipeline).

### Zero-Trust Enforcement

✅ **FULLY ENFORCED**
- **Deterministic Processing**: All processing is deterministic (no external dependencies)
- **No External Network Calls**: No network dependencies
- **Pre-Validated Inputs**: All events pre-validated by Phase 4
- **Invariant Enforcement**: Fail-closed on invariant violations

### STIG Hardening Status

⚠️ **PARTIAL**
- **Rootless Runtime**: Runs as `ransomeye` user (not root)
- **Systemd Hardening**: systemd security hardening applied (NoNewPrivileges, PrivateTmp, ProtectSystem, etc.)
- **No Explicit STIG Validation**: No explicit STIG compliance validation found

---

## 1️⃣2️⃣ CI / Validation / Testing

### Tests Present

✅ **YES** - Located in `/home/ransomeye/rebuild/core/engine/correlation/tests/`

### Test Coverage

**Test Suites:**
- **Synthetic Attack Tests**: Ransomware campaign replay tests
- **False Positive Tests**: Legitimate activity handling
- **Ordering Tests**: Event ordering validation
- **Scale Tests**: 50k+ entity scale testing
- **Invariant Violation Tests**: Invariant enforcement tests
- **Determinism Tests**: Identical input → identical output tests

### Synthetic Data Generation

✅ **YES** - Tests use synthetic ransomware attack scenarios to validate kill-chain detection

### CI Workflows

✅ **YES** - Tests run in CI pipelines

### Validation Coverage

✅ **COMPREHENSIVE**
- Kill-chain stage inference tested
- Entity state management tested
- Confidence scoring tested
- Invariant enforcement tested
- Temporal correlation tested
- Scale and performance tested
- Determinism validated

---

## 1️⃣3️⃣ Known Gaps & Technical Debt

### Missing Components

1. **Metrics Endpoints**: No Prometheus/metrics endpoints exposed
   - **Impact**: Limited operational observability
   - **Recommendation**: Add Prometheus metrics endpoints for correlation performance, entity counts, detection rates, stage transition rates

2. **Database Persistence**: Detection results not persisted by correlation engine
   - **Impact**: Detection results only available in-memory or via downstream modules
   - **Recommendation**: Consider persistent audit log for detection results or rely on downstream modules for persistence

3. **Configuration via ENV**: Configuration uses code defaults, not fully ENV-driven
   - **Impact**: Limited runtime configurability
   - **Recommendation**: Add ENV variables for all configuration parameters

### Partial Implementations

**NONE IDENTIFIED**

### Design Risks

1. **In-Memory State**: Entity state is in-memory only (intentional for bounded memory)
   - **Risk**: State lost on service restart
   - **Mitigation**: TTL and LRU eviction ensure bounded memory, state is ephemeral by design
   - **Recommendation**: Document that entity state is ephemeral and detections are output to downstream modules

2. **No State Persistence**: No persistent storage for entity state
   - **Risk**: Cannot replay correlation after restart
   - **Mitigation**: Design assumes events can be replayed from Phase 4 if needed
   - **Recommendation**: Document replay strategy or consider lightweight state persistence

---

## 1️⃣4️⃣ Recommendations

### Refactors

1. **Metrics Exposure**: Add Prometheus metrics endpoints for operational observability

2. **ENV Configuration**: Add ENV variables for all configuration parameters (max entities, TTL, thresholds, etc.)

3. **State Persistence**: Consider lightweight state persistence for entity state (optional, with TTL)

### Missing Enforcement

**NONE IDENTIFIED** - Current enforcement appears comprehensive.

### Architectural Fixes

1. **State Persistence Strategy**: Document or implement state persistence strategy for entity state (if required for audit/replay)

2. **Configuration Management**: Standardize configuration management via ENV variables

### Training Improvements

**NOT APPLICABLE** - Phase 5 does not use ML models.

### Security Hardening

1. **STIG Compliance**: Add explicit STIG compliance validation and hardening

2. **Audit Logging**: Enhance audit logging for all detection results (if not already implemented)

---

## Summary

Phase 5 (Correlation & Decision Engine) is **FULLY IMPLEMENTED** and provides deterministic ransomware kill-chain correlation with 10 ransomware-specific stages. The implementation includes comprehensive invariant enforcement (fail-closed), bounded memory management (TTL + LRU eviction), deterministic confidence scoring, and explainability generation. All processing is deterministic and replayable. Comprehensive test coverage validates functionality, scale (50k+ entities), and determinism.

**Status**: ✅ **PRODUCTION READY**

**Note**: While production-ready, recommendations include adding metrics endpoints for operational observability and ENV-driven configuration. Entity state is intentionally in-memory to ensure bounded memory usage.

---

**Last Updated**: 2025-01-27  
**Validation Status**: ✅ All components validated and tested

