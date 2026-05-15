# CodeFriend Product Report Template

## Purpose

Provide the bounded `CodeFriend` product-report template for `WP-06`.

This template is intentionally aligned to the existing
`product-report-writer` output contract so the product surface stays grounded in
current skill behavior.

## Required Sections

### Executive Summary

- what was reviewed
- top outcome in plain language
- highest-severity risks
- what the report does not claim

### Review Scope

- repo or path scope
- included and excluded surfaces
- review mode
- known limits

### Top Findings

For each finding:

- severity
- short summary
- evidence path(s)
- impact
- recommended action

### Architecture Summary

- major boundaries
- notable coupling or layering observations
- architectural strengths and risks

### Security And Privacy Notes

- trust-boundary observations
- privacy caveats
- secret-handling caveats
- publication-boundary caveats

### Diagram Links

- architecture diagrams
- workflow diagrams
- supporting packets when available

### Test Recommendations

- missing tests
- weak assertions
- high-value regression targets

### Documentation And Onboarding Notes

- stale docs
- missing docs
- onboarding friction

### Remediation Sequence

- ordered next steps
- dependencies between steps
- what should happen before publication or release

### Caveats And Residual Risks

- non-reviewed surfaces
- unclear evidence
- unresolved disagreement
- risks not yet remediated

### Appendix

- packet root
- synthesis artifact
- specialist artifact list
- validation notes

### Publication Boundary

Required statement:

- whether the report is internal-only, customer-review candidate, or blocked
- what additional redaction/evidence or quality checks are still required

## Required Writing Rules

- preserve the highest severity present in source findings
- do not hide specialist disagreement
- do not rewrite uncertainty into marketing certainty
- use repo-relative or packet-relative paths
- do not include absolute host paths
- do not claim approval, compliance, merge-readiness, remediation completion,
  or publication readiness unless another artifact explicitly proves it

## Minimal JSON Alignment

The paired JSON report should preserve the same major surfaces:

- `status`
- `audience`
- `publication_boundary`
- `scope`
- `top_findings`
- `architecture_summary`
- `security_privacy_notes`
- `diagram_links`
- `test_recommendations`
- `remediation_sequence`
- `residual_risks`
- `appendix`

## Non-Claims

- This template does not replace the existing product-report skill contract.
- This template does not authorize external delivery by itself.

