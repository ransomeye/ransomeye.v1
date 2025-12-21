# Failure Modes

**Path and File Name:** `/home/ransomeye/rebuild/ransomeye_correlation/docs/failure_modes.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Failure modes documentation - how the engine handles failures

## Overview

The Correlation Engine is **fail-closed**: on any ambiguity or error, it rejects rather than proceeding.

## Failure Modes

### 1. Ambiguous Correlation
**Symptom**: Multiple rules match with conflicting results  
**Action**: NO ALERT  
**Reason**: Cannot determine correct correlation  
**Recovery**: Review rules, adjust conditions

### 2. Missing Rule
**Symptom**: Required rule not found  
**Action**: NO ALERT  
**Reason**: Cannot correlate without rule  
**Recovery**: Load missing rule, restart engine

### 3. Ordering Violation
**Symptom**: Event out of sequence  
**Action**: DROP EVENT  
**Reason**: Cannot process out-of-order events  
**Recovery**: Event is dropped, processing continues

### 4. State Corruption
**Symptom**: Invalid state transition detected  
**Action**: ENGINE HALT  
**Reason**: State machine integrity compromised  
**Recovery**: Manual intervention required, engine reset

### 5. Window Overflow
**Symptom**: Too many events in window  
**Action**: DROP EVENT  
**Reason**: Window capacity exceeded  
**Recovery**: Event is dropped, processing continues

### 6. Rule Signature Invalid
**Symptom**: Rule signature verification fails  
**Action**: ENGINE REFUSES TO START  
**Reason**: Unsigned or tampered rule  
**Recovery**: Fix rule signature, restart engine

### 7. Evidence Bundle Failure
**Symptom**: Cannot create evidence bundle  
**Action**: NO ALERT  
**Reason**: Cannot generate alert without evidence  
**Recovery**: Fix evidence creation, retry

## Error Handling

### Fail-Closed Principle
- On any ambiguity → NO ALERT
- On any error → NO ALERT (unless recoverable)
- On state corruption → ENGINE HALT

### Logging
- All failures are logged
- Failure reasons are recorded
- Recovery actions are documented

### Recovery
- Most failures are recoverable
- State corruption requires manual intervention
- Engine can be restarted after rule fixes

## Prevention

### Rule Validation
- Rules validated on load
- Signatures verified
- Versions checked

### State Monitoring
- State transitions monitored
- Corruption detected immediately
- Halt on detection

### Window Management
- Window size limits enforced
- Overflow prevented
- Cleanup performed

