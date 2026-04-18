---
name: gap-analysis
description: Compare an explicit expected baseline against observed implementation, docs, tests, review, PR, milestone, or closeout evidence and produce findings-first gap reports with severity, evidence, uncertainty, and bounded follow-up recommendations without fixing, approving, or mutating repositories.
---

# Gap Analysis

Run a bounded gap review between what was expected and what evidence shows. This
skill is a comparison and closeout-safety skill, not a code reviewer, not a
remediator, not a publisher, and not an approval authority.

Use this skill before issue closeout, milestone closeout, third-party review,
release readiness, or customer-facing report publication when a specific
baseline must be reconciled against observed truth.

## Quick Start

1. Confirm the expected baseline:
   - issue acceptance criteria
   - milestone plan
   - feature spec
   - PR scope
   - review packet
   - closeout record
2. Confirm the observed evidence:
   - changed files
   - test results
   - docs
   - review artifacts
   - PR body/checks
   - output card or closeout record
3. Run the deterministic helper when local filesystem access is available:
   - `scripts/analyze_gaps.py <gap-root> --out <artifact-root>`
4. Review the gap report for source-grounded findings, uncertainty, and follow-up
   recommendations.
5. Stop before fixing gaps, creating issues, creating PRs, approving closeout, or
   mutating repositories.

## Required Inputs

At minimum, gather:

- `mode`
- `expected_baseline`
- `observed_evidence`
- `policy`

Supported modes:

- `compare_issue_to_implementation`
- `compare_milestone_to_evidence`
- `compare_spec_to_docs`
- `compare_review_to_closeout`
- `compare_packet_to_report`

Useful policy fields:

- `severity_floor`
- `required_gap_types`
- `uncertainty_policy`
- `issue_creation_allowed`
- `write_gap_artifact`
- `stop_before_fix`
- `stop_before_mutation`

If there is no explicit expected baseline, stop and report `not_run`. Do not
infer intended outcomes from vibes or from the absence of evidence.

## Gap Types

Classify gaps as:

- `missing_evidence`: expected proof is absent or unreadable.
- `implementation_gap`: expected behavior or artifact is not present in observed
  implementation evidence.
- `docs_drift`: docs claim something different from the expected or observed
  truth.
- `test_gap`: expected validation is missing, weak, skipped, or not linked to
  behavior.
- `closeout_drift`: output card, PR body, issue state, or closeout note overstates
  integration, validation, scope, or merge truth.
- `scope_ambiguity`: evidence is insufficient to decide whether a gap exists.

## Severity Policy

- `P0`: critical release or customer-facing truth break, security/privacy
  exposure, or irreversible closeout failure.
- `P1`: release-blocking mismatch, serious overclaim, missing high-risk proof, or
  issue closeout that would mislead operators.
- `P2`: meaningful implementation, docs, tests, review, or closeout gap that
  should be fixed but does not block all work.
- `P3`: lower-risk clarity, traceability, or caveat gap with concrete reviewer or
  operator impact.

Do not escalate speculation into a finding. If the evidence is insufficient,
record `scope_ambiguity` or `missing_evidence` with the missing source named.

## Output

Write Markdown and JSON artifacts when an output root is available.

Default artifact root:

```text
.adl/reviews/gap-analysis/<run_id>/
```

Required artifacts:

- `gap_analysis_report.md`
- `gap_analysis_report.json`

Use the detailed contract in `references/output-contract.md`.

## Stop Boundary

This skill must not:

- fix implementation, docs, tests, cards, or reports
- create issues or PRs without explicit operator approval
- close issues or approve releases
- claim merge-readiness, release-readiness, remediation completion, compliance,
  or publication approval
- replace repo-code-review, security-threat-model, review-quality-evaluator, or
  product-report-writer
- treat absent evidence as proof of failure when uncertainty is the truthful
  outcome

Handoff candidates:

- `finding-to-issue-planner` when human-approved issue candidates are needed.
- `review-to-test-planner` when gaps are validation/test related.
- `product-report-writer` or `review-quality-evaluator` when report truth is the
  gap surface.
- `pr-closeout` or `sor-editor` when closeout truth needs normalization.

## Blocked States

Return `not_run` when the expected baseline is missing or unreadable.

Return `blocked` when the requested output would require fixing, approving,
publishing, creating issues/PRs, or mutating a repository.

Return `partial` when a report can be produced but important evidence is missing
or ambiguous.
