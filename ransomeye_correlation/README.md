# RansomEye Correlation Engine

**Path and File Name:** `/home/ransomeye/rebuild/ransomeye_correlation/README.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Phase 5 - Deterministic correlation engine and kill-chain inference

## Overview

The RansomEye Correlation Engine is the **ONLY deterministic detection authority** in RansomEye. It processes validated events from Phase 4 and generates alerts based on rule-based correlation.

## Core Principles

### Determinism
- Same input → same output (always)
- No AI, no ML, no heuristics
- No probabilistic decisions
- All outputs are reproducible

### Fail-Closed
- Ambiguous correlation → NO ALERT
- Missing rule → NO ALERT
- Ordering violation → DROP EVENT
- State corruption → ENGINE HALT

### Rule-Based
- Declarative YAML rules
- Rules must be signed
- Rules must be versioned
- Unsigned rule → engine refuses to start

## Architecture

### Components

1. **CorrelationEngine** - Main engine coordinator
2. **EventPipeline** - Event processing pipeline
3. **RuleEngine** - Rule loading and validation
4. **Correlator** - Rule-based correlation
5. **StateMachine** - Kill-chain state tracking
6. **KillChainInferencer** - Stage inference
7. **EvidenceBuilder** - Cryptographic evidence bundling
8. **OrderingValidator** - Event ordering validation
9. **SlidingWindow** - Window management

## Usage

### Configuration

Environment variables:
- `RANSOMEYE_CORRELATION_RULES_PATH` - Path to rules directory (default: `/etc/ransomeye/correlation/rules`)
- `RANSOMEYE_CORRELATION_WINDOW_SECONDS` - Window size in seconds (default: `300`)
- `RANSOMEYE_CORRELATION_MAX_EVENTS` - Max events per window (default: `1000`)

### Processing Events

```rust
use ransomeye_correlation::CorrelationEngine;

let engine = CorrelationEngine::new(
    "/etc/ransomeye/correlation/rules",
    300,  // window size
    1000, // max events
    "1.0.0"
)?;

let alert = engine.process_event(
    "event_id",
    "producer_id",
    1, // sequence number
    Utc::now(),
    "event_type",
    event_data
)?;
```

## Rules

Rules are defined in YAML format and must be signed. See `rules/` directory for examples.

## Kill-Chain

The engine tracks attack progression through MITRE ATT&CK kill-chain stages:
1. Reconnaissance
2. Weaponization
3. Delivery
4. Exploitation
5. Installation
6. Command & Control
7. Actions on Objectives

## Evidence

Every alert includes a complete evidence bundle:
- Input event hashes
- Rule IDs
- Kill-chain stage
- State transitions
- Cryptographic signature

## Testing

Run tests with:
```bash
cargo test
```

Test categories:
- Deterministic replay tests
- Rule consistency tests
- Ordering dependency tests
- Ambiguity rejection tests
- Evidence integrity tests

## Documentation

See `docs/` directory for detailed documentation:
- `correlation_model.md` - Correlation model
- `determinism_guarantees.md` - Determinism guarantees
- `false_positive_policy.md` - False positive policy
- `failure_modes.md` - Failure modes

## Compliance

This phase fully complies with:
- Phase 1: Core Engine & Installer
- Phase 4: Event Ingestion (consumes validated events)
- Phase 5: Correlation Engine (this phase)

## License

© RansomEye.Tech | Support: Gagan@RansomEye.Tech

