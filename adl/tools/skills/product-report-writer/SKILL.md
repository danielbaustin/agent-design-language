---
name: product-report-writer
description: Turn CodeBuddy review packet artifacts into customer-grade product reports with executive summary, scope, top findings, architecture summary, diagrams, test recommendations, remediation sequence, caveats, and residual risks while preserving severity, disagreement, and publication boundaries.
---

# Product Report Writer

Write a customer-grade CodeBuddy review report from already-produced review
artifacts. This skill is a report-writing and packaging skill, not a reviewer,
not a remediator, not a publisher, and not an approval authority.

Use this skill after review packet construction, specialist reviews, synthesis,
diagram planning/review, test planning, redaction, and issue planning have
produced the evidence needed for a report.

## Quick Start

1. Confirm the report source:
   - CodeBuddy packet root
   - synthesis artifact
   - specialist artifact bundle
   - existing draft report refresh
2. Confirm audience, privacy mode, and publication intent.
3. Run the deterministic helper when local filesystem access is available:
   - `scripts/write_product_report.py <packet-root> --out <artifact-root>`
4. Review the report for evidence boundaries, severity preservation, specialist
   disagreement, and residual risk.
5. Stop before publication, approval, customer delivery, issue creation, PR
   creation, remediation, or repository mutation.

## Required Inputs

At minimum, gather:

- `artifact_root` or `packet_root`
- `mode`
- `audience`
- `policy`

Supported modes:

- `write_from_packet`
- `write_from_synthesis`
- `write_from_specialist_artifacts`
- `refresh_report`

Useful policy fields:

- `privacy_mode`
- `publication_intent`
- `write_report_artifact`
- `require_redaction_status`
- `preserve_specialist_disagreement`
- `stop_before_publication`
- `stop_before_mutation`

If there is no concrete artifact root or report source, stop and report
`not_run`.

## Workflow

### 1. Establish Report Boundary

Record:

- artifact root or source report
- repo name and ref, if present
- audience
- privacy mode
- publication intent
- redaction status
- specialist artifacts included
- non-reviewed surfaces

Do not upgrade publication readiness. If redaction or quality-gate evidence is
missing, make that caveat prominent.

### 2. Preserve Review Truth

Carry forward:

- highest severity per finding
- source roles
- confidence
- evidence summary
- impact
- recommended action
- validation gap
- specialist disagreement
- residual risk
- non-reviewed surfaces

Do not rewrite findings into softer product language that hides severity,
uncertainty, or disagreement.

### 3. Write The Report

The report should include:

- executive summary
- review scope
- top findings
- architecture summary
- security and privacy notes
- diagram links
- test recommendations
- documentation and onboarding notes
- remediation sequence
- caveats and residual risks
- appendix of source artifacts

Keep the report useful to a customer or leadership reader without losing enough
evidence for an engineer to act.

### 4. Stop Before Publication

This skill must not:

- publish or send a report
- claim approval, compliance, merge-readiness, or remediation completion
- create issues, pull requests, tests, diagrams, ADRs, or fixes
- edit customer repositories
- run specialist review lanes
- hide missing redaction, quality-gate, or specialist coverage

Handoff candidates to:

- `redaction-and-evidence-auditor` before customer-facing use
- `review-quality-evaluator` before publication candidate status
- `finding-to-issue-planner` for approved issue follow-through
- `review-to-test-planner` for approved test planning
- `diagram-author` or `repo-diagram-planner` when diagram evidence is missing

## Output

Write Markdown and JSON report artifacts when an output root is available.

Default artifact root:

```text
.adl/reviews/codebuddy/<run_id>/product-report/
```

Required artifacts:

- `codebuddy_product_report.md`
- `codebuddy_product_report.json`

Use the detailed contract in `references/output-contract.md`.

## Blocked States

Return `not_run` when the source is missing or unreadable.

Return `blocked` when:

- publication was requested without redaction status
- the source has findings without evidence
- the requested report would hide severity or disagreement
- the report is asked to claim compliance, approval, merge-readiness, or
  remediation completion without proof

Return `partial` when a report can be drafted but important specialist,
redaction, diagram, test, or quality-gate evidence is missing.
