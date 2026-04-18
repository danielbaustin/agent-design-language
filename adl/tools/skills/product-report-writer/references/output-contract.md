# Output Contract

The product-report-writer skill produces customer-grade CodeBuddy report
artifacts from existing review evidence.

Default artifact root:

```text
.adl/reviews/codebuddy/<run_id>/product-report/
```

## Required Artifacts

### codebuddy_product_report.md

Required sections:

- Executive Summary
- Review Scope
- Top Findings
- Architecture Summary
- Security And Privacy Notes
- Diagram Links
- Test Recommendations
- Documentation And Onboarding Notes
- Remediation Sequence
- Caveats And Residual Risks
- Appendix
- Publication Boundary

### codebuddy_product_report.json

Required top-level fields:

- `schema`
- `repo_name`
- `run_id`
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

## Status Values

- `pass`: report is complete for internal review and no required inputs are
  missing.
- `partial`: report was written but missing evidence, redaction, quality, or
  specialist coverage must be reviewed.
- `not_run`: no readable packet or report source was available.
- `blocked`: the requested output would overclaim approval, compliance,
  publication readiness, remediation completion, or hide evidence boundaries.

## Rules

- Use repo-relative or packet-relative paths.
- Do not write absolute host paths into report artifacts.
- Do not claim approval, compliance, merge-readiness, remediation completion, or
  publication readiness unless a source artifact explicitly proves it.
- Do not publish or transmit the report.
- Do not create issues, PRs, tests, diagrams, ADRs, or fixes.
- Preserve highest severity and specialist disagreement.
- Include non-reviewed surfaces, caveats, and residual risk.
- Surface missing redaction, quality-gate, diagram, test, or specialist evidence
  as caveats instead of hiding them.
