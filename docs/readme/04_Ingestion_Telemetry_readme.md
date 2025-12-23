# Phase 4 — Ingestion & Telemetry Pipeline

**Path and File Name:** `/home/ransomeye/rebuild/docs/readme/04_Ingestion_Telemetry_readme.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Comprehensive technical audit and documentation for Phase 4 - Ingestion & Telemetry Pipeline

---

## 1️⃣ Phase Overview

### Purpose
Phase 4 builds the **ONLY ingress point** into the RansomEye Control Plane. It receives, validates, and processes all telemetry events from Data Plane components (DPI Probe, Linux Agent, Windows Agent). All incoming data is untrusted, potentially malicious, and must be strictly validated before forwarding to the Control Plane.

**NO correlation. NO policy. NO AI. NO inference.** Pure ingestion and validation only.

### Security Objective
- **Zero trust** - All events are untrusted and must be authenticated
- **Cryptographic validation** - All events must be signed (RSA-4096-PSS-SHA256)
- **Strict schema validation** - No permissive parsing
- **Deterministic backpressure** - Explicit rejection signals (no silent drops)
- **Replay protection** - Nonce-based replay detection
- **Ordering guarantees** - Per-producer event ordering

### Role in Architecture
Phase 4 is the **boundary between Data Plane and Control Plane**:
- Receives events from Data Plane components (DPI Probe, Linux Agent, Windows Agent)
- Validates all events through strict pipeline (authentication, signature, schema, rate limiting, backpressure, ordering)
- Dispatches validated events to Control Plane (Correlation Engine - Phase 5)
- Enforces deterministic backpressure to prevent system overload

---

## 2️⃣ Implementation Status

| Component | Status | Notes |
|-----------|--------|-------|
| Event Listener | ✅ Fully Implemented | TCP listener for incoming events |
| Authentication | ✅ Fully Implemented | Producer identity verification |
| Signature Verification | ✅ Fully Implemented | RSA-4096-PSS-SHA256 |
| Schema Validation | ✅ Fully Implemented | Strict JSON schema validation |
| Rate Limiting | ✅ Fully Implemented | Per-producer, per-component, global limits |
| Backpressure Controller | ✅ Fully Implemented | Explicit backpressure signals |
| Event Buffer | ✅ Fully Implemented | Bounded in-memory buffer |
| Ordering Manager | ✅ Fully Implemented | Per-producer ordering guarantees |
| Content Deduplicator | ✅ Fully Implemented | Content hash deduplication |
| Event Dispatcher | ✅ Fully Implemented | Dispatch to Control Plane |
| Trust Store | ✅ Fully Implemented | Identity certificate management |
| Revocation Checking | ✅ Fully Implemented | CRL and JSON revocation lists |
| Replay Protection | ✅ Fully Implemented | Nonce-based replay detection |

---

## 3️⃣ File & Folder Structure

### Root Directory
`/home/ransomeye/rebuild/core/ingest/`

### Key Files
- **`src/main.rs`**: Main entry point for ingestion server
- **`src/server.rs`**: Ingestion server coordinator
- **`src/listener.rs`**: TCP event listener
- **`src/auth.rs`**: Producer authentication
- **`src/signature.rs`**: Cryptographic signature verification
- **`src/schema.rs`**: Event schema validation
- **`src/rate_limit.rs`**: Rate limiting (per-producer, per-component, global)
- **`src/backpressure.rs`**: Backpressure controller
- **`src/buffer.rs`**: Bounded event buffer
- **`src/ordering.rs`**: Event ordering manager
- **`src/dedupe.rs`**: Content deduplication
- **`src/dispatcher.rs`**: Event dispatcher to Control Plane
- **`src/normalization.rs`**: Event normalization
- **`src/versioning.rs`**: Schema version management
- **`src/config.rs`**: Configuration loader
- **`src/security/identity.rs`**: Identity verification
- **`src/security/trust_chain.rs`**: Trust chain validation
- **`src/security/revocation.rs`**: Revocation list checking
- **`src/security/replay.rs`**: Replay protection
- **`protocol/event_envelope.rs`**: Event envelope structure
- **`protocol/event_schema_v1.json`**: Event schema v1 definition

### Configuration Files
- **`config/env_schema.md`**: Environment variable schema documentation
- **`config/validation.rs`**: Configuration validation

### Documentation
- **`docs/ingestion_flow.md`**: Complete ingestion flow documentation
- **`docs/backpressure_model.md`**: Backpressure model documentation
- **`docs/rate_limit_policy.md`**: Rate limiting policy
- **`docs/ORDERING_GUARANTEES.md`**: Event ordering guarantees
- **`docs/failure_modes.md`**: Failure modes and recovery
- **`SECURITY_FIXES.md`**: Security fixes and improvements

---

## 4️⃣ Modules & Services

### Modules

1. **EventListener** (`src/listener.rs`)
   - **Responsibility**: TCP listener for incoming events from producers
   - **Runtime Behavior**: Accepts TCP connections, reads event data, processes through pipeline
   - **systemd Integration**: ✅ YES (part of `ransomeye-ingestion.service`)
   - **Installer Integration**: ✅ YES

2. **Authenticator** (`src/auth.rs`)
   - **Responsibility**: Verifies producer identity, expiration, and revocation
   - **Runtime Behavior**: Validates producer ID, identity certificates, checks revocation lists
   - **systemd Integration**: ✅ YES (part of ingestion service)
   - **Installer Integration**: ✅ YES

3. **SignatureVerifier** (`src/signature.rs`)
   - **Responsibility**: Verifies cryptographic signatures on all events
   - **Runtime Behavior**: Validates RSA-4096-PSS-SHA256 signatures, matches producer identity
   - **systemd Integration**: ✅ YES (part of ingestion service)
   - **Installer Integration**: ✅ YES

4. **SchemaValidator** (`src/schema.rs`)
   - **Responsibility**: Strictly validates event schemas
   - **Runtime Behavior**: Validates schema version compatibility, required fields, field types
   - **systemd Integration**: ✅ YES (part of ingestion service)
   - **Installer Integration**: ✅ YES

5. **RateLimiter** (`src/rate_limit.rs`)
   - **Responsibility**: Enforces deterministic rate limits
   - **Runtime Behavior**: Tracks per-producer, per-component, and global rate limits using fixed windows
   - **systemd Integration**: ✅ YES (part of ingestion service)
   - **Installer Integration**: ✅ YES

6. **BackpressureController** (`src/backpressure.rs`)
   - **Responsibility**: Manages explicit backpressure signals
   - **Runtime Behavior**: Signals backpressure when buffer full, rate limit exceeded, or system overloaded
   - **systemd Integration**: ✅ YES (part of ingestion service)
   - **Installer Integration**: ✅ YES

7. **EventBuffer** (`src/buffer.rs`)
   - **Responsibility**: Manages bounded event buffering
   - **Runtime Behavior**: Bounded in-memory buffer with explicit capacity checks
   - **systemd Integration**: ✅ YES (part of ingestion service)
   - **Installer Integration**: ✅ YES

8. **OrderingManager** (`src/ordering.rs`)
   - **Responsibility**: Ensures per-producer event ordering
   - **Runtime Behavior**: Tracks sequence numbers per producer, detects out-of-order and replay
   - **systemd Integration**: ✅ YES (part of ingestion service)
   - **Installer Integration**: ✅ YES

9. **ContentDeduplicator** (`src/dedupe.rs`)
   - **Responsibility**: Content hash deduplication
   - **Runtime Behavior**: Detects duplicate content based on integrity hash
   - **systemd Integration**: ✅ YES (part of ingestion service)
   - **Installer Integration**: ✅ YES

10. **EventDispatcher** (`src/dispatcher.rs`)
    - **Responsibility**: Dispatches validated events to Control Plane
    - **Runtime Behavior**: Forwards events to Correlation Engine (Phase 5)
    - **systemd Integration**: ✅ YES (part of ingestion service)
    - **Installer Integration**: ✅ YES

### Services

1. **ransomeye-ingestion.service**
   - **Location**: `/home/ransomeye/rebuild/systemd/ransomeye-ingestion.service`
   - **Type**: systemd service (simple)
   - **User**: ransomeye (rootless)
   - **Dependencies**: `network.target`, `ransomeye-core.service`
   - **Restart**: always
   - **Status**: ✅ Enabled and running

---

## 5️⃣ AI / ML / LLM DETAILS

**NOT APPLICABLE** - Phase 4 is pure ingestion and validation only. No AI/ML/LLM components.

**Hard Rule**: "NO correlation. NO policy. NO AI. NO inference."

---

## 6️⃣ SOC Copilot / AI Copilot

**NOT PRESENT** - Phase 4 does not include AI Copilot functionality.

---

## 7️⃣ Database Design

**NOT APPLICABLE** - Phase 4 does not store events in a database.

**Storage Mechanisms:**
- **In-Memory Buffer**: Bounded event buffer (`EventBuffer`) for transient storage
- **Event Dispatch**: Events are immediately dispatched to Control Plane (Correlation Engine) after validation
- **No Persistent Storage**: Events are not persisted by ingestion module itself

**Note**: Events may be persisted by downstream modules (e.g., Correlation Engine, Reporting module), but Phase 4 does not create or manage database tables.

---

## 8️⃣ Ports & Interconnectivity

### Network Ports

**Inbound Port:**
- **Listen Address**: Configurable via `RANSOMEYE_INGESTION_LISTEN_ADDR` (default: `0.0.0.0:8080`)
- **Protocol**: TCP
- **Purpose**: Receives events from Data Plane components (DPI Probe, Linux Agent, Windows Agent)

**Outbound Connectivity:**
- **Control Plane Address**: Configurable via `RANSOMEYE_CONTROL_PLANE_ADDR` (default: `127.0.0.1:9090`)
- **Protocol**: Internal IPC or TCP (implementation-specific)
- **Purpose**: Dispatches validated events to Correlation Engine (Phase 5)

### Interconnectivity

**Data Flow:**
1. **Data Plane → Ingestion** (Phase 4)
   - **Source**: DPI Probe (Phase 23), Linux Agent (Phase 21), Windows Agent (Phase 22)
   - **Protocol**: TCP
   - **Port**: `RANSOMEYE_INGESTION_LISTEN_ADDR` (default: 8080)
   - **Authentication**: Producer identity verification required
   - **Trust Boundaries**: ✅ YES - Zero trust, all events authenticated and signed

2. **Ingestion → Control Plane** (Phase 4 → Phase 5)
   - **Destination**: Correlation Engine (Phase 5)
   - **Protocol**: Internal IPC or TCP
   - **Address**: `RANSOMEYE_CONTROL_PLANE_ADDR` (default: 127.0.0.1:9090)
   - **Trust Boundaries**: ✅ YES - Validated events only

### Trust Boundaries Enforced

✅ **FULLY ENFORCED**
- **Producer Authentication**: All producers must have valid identity certificates
- **Signature Verification**: All events must be cryptographically signed
- **Revocation Checking**: Revoked identities rejected
- **Replay Protection**: Replay attacks detected and rejected
- **Schema Validation**: Only valid schema versions accepted
- **Rate Limiting**: Prevents flooding attacks

### Configuration (Environment Variables)

**Network Configuration:**
- `RANSOMEYE_INGESTION_LISTEN_ADDR`: Listen address (default: `0.0.0.0:8080`)
- `RANSOMEYE_CONTROL_PLANE_ADDR`: Control Plane address (default: `127.0.0.1:9090`)

**Rate Limiting:**
- `RANSOMEYE_PRODUCER_RATE_LIMIT`: Per-producer rate limit (default: 1000 events)
- `RANSOMEYE_GLOBAL_RATE_LIMIT`: Global rate limit (default: 10000 events)
- `RANSOMEYE_RATE_LIMIT_WINDOW_SECONDS`: Rate limit window (default: 60 seconds)

**Buffer Configuration:**
- `RANSOMEYE_BUFFER_CAPACITY`: Buffer capacity (default: 10000 events)

**Backpressure Configuration:**
- `RANSOMEYE_BACKPRESSURE_CLEAR_SECONDS`: Backpressure clear timeout (default: 10 seconds)

**Security Configuration:**
- `RANSOMEYE_TRUST_STORE_PATH`: Trust store path (default: `/etc/ransomeye/trust_store`)
- `RANSOMEYE_CRL_PATH`: Certificate revocation list path (default: `/etc/ransomeye/crl/revocation.json`)

---

## 9️⃣ UI / Dashboards / Frontend

**NO UI IN THIS PHASE** - Phase 4 is a backend service with no user interface.

**Monitoring Interfaces:**
- **Systemd Journal**: Logs available via `journalctl -u ransomeye-ingestion`
- **Console Output**: Structured logging via Rust tracing framework

**Note**: UI for monitoring ingestion metrics (rate limits, backpressure, buffer usage) is expected in Phase 11 (UI & Dashboards).

---

## 🔟 Logging, Metrics & Observability

### Logs Generated

**Operational Logs:**
- **Event Acceptance**: Successful event processing
- **Event Rejection**: Authentication failures, signature failures, schema violations, rate limit exceeded, backpressure
- **Buffer Status**: Buffer capacity warnings
- **Rate Limit Status**: Rate limit warnings
- **Backpressure Events**: Backpressure activation/clearing

**Security Logs:**
- **Authentication Failures**: Invalid producer IDs, revoked identities, expired certificates
- **Signature Failures**: Invalid signatures, signature mismatches
- **Replay Attacks**: Replay detection events
- **Ordering Violations**: Out-of-order events

**Error Logs:**
- **System Errors**: Server errors, connection errors, dispatch failures

### Log Formats

**Structured Logging:**
- **Framework**: Rust tracing framework
- **Format**: Structured logs with fields (timestamp, level, module, message, context)
- **Output**: Systemd journal (via `StandardOutput=journal`, `StandardError=journal`)

### Metrics Exposed

**NO METRICS ENDPOINTS** - Phase 4 does not expose metrics endpoints (Prometheus, etc.).

**Internal Metrics (Not Exposed):**
- Event acceptance rate
- Event rejection rate (by reason)
- Buffer utilization
- Rate limit utilization
- Backpressure status
- Producer connection count

**Recommendation**: Add Prometheus metrics endpoints for operational observability.

### Prometheus/Grafana Integration

**NOT IMPLEMENTED** - No Prometheus metrics endpoints exposed.

### Audit Logs

✅ **YES** - Security events logged:
- **Authentication Failures**: Logged with producer ID, reason, timestamp
- **Signature Failures**: Logged with producer ID, event details, timestamp
- **Replay Attacks**: Logged with producer ID, nonce, timestamp
- **Ordering Violations**: Logged with producer ID, sequence numbers, timestamp

**Format**: Structured logs via Rust tracing framework
**Location**: Systemd journal

### Tamper-Proofing

✅ **ENFORCED**
- **Event Signatures**: All events cryptographically signed (RSA-4096-PSS-SHA256)
- **Signature Verification**: Signatures verified before processing
- **Replay Protection**: Nonce-based replay detection prevents replay attacks
- **Integrity Hashes**: SHA-256 integrity hashes verified
- **Ordering Protection**: Sequence number validation prevents ordering attacks

---

## 1️⃣1️⃣ Security & Compliance

### Fail-Closed Enforcement

✅ **FULLY ENFORCED**
- **Authentication Failure** → Event rejected + audit log
- **Signature Failure** → Event rejected + audit log
- **Schema Violation** → Event rejected + audit log
- **Rate Limit Exceeded** → Event rejected + backpressure signal
- **Buffer Full** → Event rejected + backpressure signal
- **Ordering Violation** → Event rejected + audit log
- **Replay Attack** → Event rejected + audit log

### Cryptographic Controls

✅ **ENFORCED**
- **RSA-4096-PSS-SHA256**: All events cryptographically signed
- **SHA-256 Integrity Hashes**: Event data integrity verification
- **Identity Certificates**: Producer identity verification via X.509 certificates
- **Trust Store**: Centralized trust store for identity certificates
- **Revocation Lists**: CRL and JSON revocation list support

### Signature Verification

✅ **ENFORCED**
- **Mandatory Signatures**: All events must be signed
- **Signature Validation**: Signatures verified against producer identity
- **Data Integrity**: Signatures verified against event data
- **Unsigned Events**: Rejected immediately

### Zero-Trust Enforcement

✅ **FULLY ENFORCED**
- **Zero Trust**: All events are untrusted by default
- **Mandatory Authentication**: All producers must authenticate
- **No Implicit Trust**: No trust granted without explicit authentication
- **Continuous Verification**: Identity and signature verified for every event

### STIG Hardening Status

⚠️ **PARTIAL**
- **Rootless Runtime**: Runs as `ransomeye` user (not root)
- **Systemd Hardening**: systemd security hardening applied (NoNewPrivileges, PrivateTmp, ProtectSystem, etc.)
- **Network Isolation**: Minimal network exposure (single listen port)
- **No Explicit STIG Validation**: No explicit STIG compliance validation found

---

## 1️⃣2️⃣ CI / Validation / Testing

### Tests Present

✅ **YES** - Located in `/home/ransomeye/rebuild/core/ingest/tests/`

### Test Coverage

**Test Suites:**
- **Authentication Tests**: Producer identity verification, revocation checking, certificate validation
- **Signature Tests**: Signature verification, signature failure handling
- **Schema Tests**: Schema validation, schema version compatibility, field validation
- **Rate Limiting Tests**: Per-producer limits, per-component quotas, global caps, priority handling
- **Backpressure Tests**: Backpressure signaling, backpressure clearing, buffer capacity
- **Ordering Tests**: Sequence number validation, replay detection, ordering violations
- **Deduplication Tests**: Content hash deduplication
- **Overload Tests**: System overload handling, buffer exhaustion

### Synthetic Data Generation

**NOT APPLICABLE** - Tests use real event structures, not synthetic data generation.

### CI Workflows

✅ **YES** - Tests run in CI pipelines

### Validation Coverage

✅ **COMPREHENSIVE**
- All validation pipeline steps tested
- All failure modes tested
- All security controls tested
- Rate limiting and backpressure tested
- Ordering and replay protection tested

---

## 1️⃣3️⃣ Known Gaps & Technical Debt

### Missing Components

1. **Metrics Endpoints**: No Prometheus/metrics endpoints exposed
   - **Impact**: Limited operational observability
   - **Recommendation**: Add Prometheus metrics endpoints for event rates, buffer utilization, rate limit utilization, backpressure status

2. **Database Persistence**: Events not persisted by ingestion module
   - **Impact**: Events lost if downstream unavailable (mitigated by buffer, but buffer is in-memory)
   - **Recommendation**: Consider persistent buffer or acknowledgment mechanism for critical events

3. **Event Acknowledgment**: No explicit acknowledgment mechanism to producers
   - **Impact**: Producers cannot confirm event delivery
   - **Recommendation**: Add acknowledgment mechanism for reliable event delivery

### Partial Implementations

**NONE IDENTIFIED**

### Design Risks

1. **In-Memory Buffer**: Events stored in in-memory buffer only
   - **Risk**: Events lost on service restart or crash
   - **Mitigation**: Buffer size bounded, events dispatched immediately
   - **Recommendation**: Consider persistent buffer for critical events or acknowledgment mechanism

2. **No Event Retention**: Events not retained after dispatch
   - **Risk**: Cannot replay events if downstream processing fails
   - **Mitigation**: Downstream modules (Correlation Engine, Reporting) may persist events
   - **Recommendation**: Document event persistence strategy across pipeline

---

## 1️⃣4️⃣ Recommendations

### Refactors

1. **Metrics Exposure**: Add Prometheus metrics endpoints for operational observability
   - Event acceptance/rejection rates
   - Buffer utilization
   - Rate limit utilization
   - Backpressure status
   - Producer connection metrics

2. **Persistent Buffer**: Consider persistent buffer for critical events to prevent data loss on service restart

### Missing Enforcement

**NONE IDENTIFIED** - Current enforcement appears comprehensive.

### Architectural Fixes

1. **Event Acknowledgment**: Add explicit acknowledgment mechanism to producers for reliable event delivery

2. **Event Retention**: Document and potentially implement event retention strategy for audit/replay purposes

### Training Improvements

**NOT APPLICABLE** - Phase 4 does not use ML models.

### Security Hardening

1. **STIG Compliance**: Add explicit STIG compliance validation and hardening

2. **TLS/MTLS**: Consider TLS/MTLS for network communication in addition to event-level signatures

3. **Rate Limit Hardening**: Consider adaptive rate limiting based on system load

---

## Summary

Phase 4 (Ingestion & Telemetry Pipeline) is **FULLY IMPLEMENTED** and provides the **ONLY ingress point** into the RansomEye Control Plane. The implementation includes comprehensive validation pipeline (authentication, signature verification, schema validation, rate limiting, backpressure, ordering, deduplication) with fail-closed enforcement. All events are cryptographically signed and authenticated. Rate limiting and backpressure prevent system overload. Ordering and replay protection prevent attacks.

**Status**: ✅ **PRODUCTION READY**

**Note**: While production-ready, recommendations include adding metrics endpoints for operational observability and considering persistent buffer for critical events.

---

**Last Updated**: 2025-01-27  
**Validation Status**: ✅ All components validated and tested

