# Explainability Contract

**Path and File Name:** `/home/ransomeye/rebuild/ransomeye_ai_advisory/docs/explainability_contract.md`  
**Author:** nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU  
**Details:** SHAP explainability requirements and contract

## Mandatory SHAP

All numeric AI outputs MUST include SHAP explanations:
- Feature contributions
- Signal references
- Timestamp references
- Baseline comparisons

## SHAP Requirements

### Feature Contributions
- Each feature contribution must reference:
  - Feature name
  - Contribution value
  - Signal reference
  - Timestamp reference

### Validation
- SHAP values must sum to output - baseline
- All features must have contributions
- Contributions must be validated before output

## Rationale Alignment

Human-readable rationale MUST:
- Align with deterministic Phase 5 outputs
- Never contradict core detection
- Reference specific features and signals
- Include timestamp references

## Output Blocking

Outputs without SHAP are BLOCKED:
- Missing SHAP → Output rejected
- Invalid SHAP → Output rejected
- Unvalidated SHAP → Output rejected

