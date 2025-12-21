# False Positive Policy

**Path and File Name:** `/home/ransomeye/rebuild/ransomeye_correlation/docs/false_positive_policy.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** False positive policy - strict policy to minimize false positives

## Overview

The Correlation Engine follows a **strict false positive policy**: ambiguous correlation → NO ALERT.

## Policy

### 1. Ambiguity Rejection
- If correlation is ambiguous → NO ALERT
- Better to miss than to false alarm
- All conditions must be met deterministically

### 2. Rule Requirements
- Rules must be explicit
- All required conditions must match
- Optional conditions don't block alerts

### 3. Confidence Levels
- High confidence → Alert generated
- Medium confidence → Alert generated (with note)
- Low confidence → Alert generated (with warning)

### 4. Evidence Requirements
- Complete evidence chain required
- All events must be verifiable
- Missing evidence → NO ALERT

## Implementation

### Condition Evaluation
- All required conditions must match
- Optional conditions are informational
- Ambiguous conditions → NO ALERT

### Rule Matching
- Multiple rules can match
- Highest confidence rule wins
- Conflicting rules → NO ALERT

### State Validation
- State transitions must be valid
- Invalid transitions → NO ALERT
- State corruption → ENGINE HALT

## Trade-offs

### False Positives vs. False Negatives
- Prefer false negatives over false positives
- Ambiguity → NO ALERT
- Better to miss than to alarm incorrectly

### Alert Quality
- Every alert has complete evidence
- Every alert is verifiable
- Every alert is reproducible

## Mitigation

### Rule Tuning
- Rules can be tuned to reduce false positives
- Rules can be disabled if too noisy
- Rules can be versioned and updated

### Evidence Review
- All alerts include evidence
- Evidence can be reviewed offline
- Evidence is cryptographically verifiable

