# Confidence Rules

**Path and File Name:** `/home/ransomeye/rebuild/ransomeye_correlation/kill_chain/confidence_rules.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** Confidence rules for kill-chain stage inference

## Overview

Confidence levels for kill-chain stage inference are deterministic and based on explicit indicators.

## Confidence Levels

### High Confidence
- Multiple indicators present
- Explicit event type mapping
- Clear attack pattern

### Medium Confidence
- Single indicator present
- Partial pattern match
- Inferred from context

### Low Confidence
- Weak indicators
- Ambiguous pattern
- Requires additional context

## Rules

### High Confidence Indicators
- Event type directly maps to stage
- Multiple indicators present
- Clear attack progression

### Medium Confidence Indicators
- Event type partially matches
- Single indicator present
- Contextual inference

### Low Confidence Indicators
- Pattern-based inference
- Weak signal
- Requires correlation

## Determinism

- Confidence is deterministic
- Same indicators → same confidence
- No probabilistic confidence
- Explicit rules only

