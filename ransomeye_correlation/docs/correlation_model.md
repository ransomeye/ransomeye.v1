# Correlation Model

**Path and File Name:** `/home/ransomeye/rebuild/ransomeye_correlation/docs/correlation_model.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Correlation model documentation - deterministic event-driven correlation

## Overview

The RansomEye Correlation Engine is a **deterministic, rule-based correlation system** that processes validated events and generates alerts based on explicit rule matching.

## Core Principles

### 1. Determinism
- Same input → same output (always)
- No probabilistic decisions
- No time-based heuristics
- No floating thresholds

### 2. Rule-Based
- Declarative rules in YAML format
- Rules must be signed
- Rules must be versioned
- Unsigned rule → engine refuses to start

### 3. Event-Driven
- Processes events from Phase 4 (validated, signed, ordered)
- Sliding window with fixed bounds
- Finite state machines for kill-chain tracking

### 4. Fail-Closed
- Ambiguous correlation → NO ALERT
- Missing rule → NO ALERT
- Ordering violation → DROP EVENT
- State corruption → ENGINE HALT

## Architecture

### Components

1. **Event Pipeline**
   - Validates event ordering
   - Manages sliding windows
   - Routes events to correlator

2. **Rule Engine**
   - Loads and validates signed rules
   - Manages rule lifecycle
   - Verifies rule signatures

3. **Correlator**
   - Matches events against rules
   - Evaluates conditions deterministically
   - Returns correlation results

4. **State Machine**
   - Tracks kill-chain progression
   - Enforces state transitions
   - Detects state corruption

5. **Kill-Chain Inferencer**
   - Maps events to kill-chain stages
   - Determines attack progression
   - Never skips or regresses stages

6. **Evidence Builder**
   - Creates cryptographic evidence bundles
   - Includes all input events
   - Verifiable offline

7. **Alert Builder**
   - Generates deterministic alerts
   - Includes complete evidence
   - Cryptographically signed

## Correlation Process

1. **Event Reception**
   - Event validated by Phase 4
   - Event ordering checked
   - Event added to sliding window

2. **Rule Matching**
   - Events matched against all enabled rules
   - Conditions evaluated deterministically
   - Ambiguous correlation → NO ALERT

3. **State Update**
   - Kill-chain stage inferred
   - State machine updated
   - State corruption → ENGINE HALT

4. **Evidence Creation**
   - Evidence bundle created
   - All events hashed
   - Cryptographic signature applied

5. **Alert Generation**
   - Alert created with evidence
   - Severity determined
   - Alert output

## Determinism Guarantees

- **Input Determinism**: Same events → same correlation results
- **State Determinism**: Same state + same event → same transition
- **Output Determinism**: Same correlation → same alert
- **Reproducibility**: Can replay events and get identical results

## Failure Modes

- **Ambiguous Correlation**: Multiple rules match with conflicting results → NO ALERT
- **Ordering Violation**: Event out of sequence → DROP EVENT
- **State Corruption**: Invalid state transition → ENGINE HALT
- **Rule Missing**: Required rule not found → NO ALERT
- **Window Overflow**: Too many events in window → DROP EVENT

