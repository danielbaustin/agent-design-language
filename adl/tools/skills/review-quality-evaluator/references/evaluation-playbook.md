# Evaluation Playbook

Use this playbook when evaluating a CodeBuddy review packet or product report.

## Quality Bar

- Findings are first-class and ordered by severity where possible.
- Each finding has evidence, impact, recommended action, and validation gap.
- Severity labels match concrete user, customer, security, or operational impact.
- Scope, exclusions, assumptions, and non-reviewed surfaces are visible.
- Specialist disagreement is preserved or explicitly absent.
- Diagrams and generated tests are tied to source evidence and findings.
- Redaction and evidence-boundary status is visible before customer-facing use.
- Residual risk is stated plainly.

## Blocker Examples

- A finding has no evidence.
- A report claims approval, compliance, remediation completion, publication
  approval, or merge-readiness without a source artifact proving that status.
- Customer-facing publication is requested with no redaction report.
- The reviewed scope is unclear.
- Required report sections are missing.
- Residual risk and non-reviewed surfaces are absent.

## Warning Examples

- A specialist role is missing but the report caveats the gap.
- Findings are actionable but validation gaps are incomplete.
- Duplicate findings are present and need manual dedupe.
- Diagram or test artifacts are missing for a packet that can still be reviewed
  internally.

## Handoff Guidance

- Send missing redaction or unsafe evidence to `redaction-and-evidence-auditor`.
- Send weak final report structure to `product-report-writer`.
- Send unresolved disagreement or duplicate findings to `repo-review-synthesis`.
- Send approved follow-through candidates to `finding-to-issue-planner`.
