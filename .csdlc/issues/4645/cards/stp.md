# Structured Task Prompt

Template: 1.0.0

Issue: 4645

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare and execute only the WP-18 internal review packet; do not perform WP-19 external review, WP-20 remediation, WP-22 next-milestone review, or WP-23 release ceremony.

## Deliverables

- Retained v0.91.7 internal milestone review packet
- Findings register with severity, evidence, disposition, and owner route
- Specialist lane coverage matrix
- Validation adequacy and release-readiness boundary summary
- Explicit non-claims and follow-up routing

## Acceptance

1. AC-1: The review covers WP-01 through WP-23 and named closed sprint packets without silently skipping open or blocked rows
2. AC-2: Findings are evidence-bound, severity-ranked, and routed to existing or proposed owners
3. AC-3: The packet distinguishes release-ready, blocked-with-evidence, open, superseded, and not-reviewed surfaces
4. AC-4: Validation evidence states what was freshly checked, what is retained evidence, and what remains unproven
5. AC-5: The review does not claim v0.91.7 release readiness or v0.92 activation readiness

## Dependencies

- Current root main checkout
- Issue #4645 live GitHub truth
- docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md
- docs/reviews/v0.91.7/remaining-sprints-5403/
- Open remediation issue state for #5404 through #5413 where applicable

## Inputs

- docs/milestones/v0.91.7/README.md
- docs/milestones/v0.91.7/WBS_v0.91.7.md
- docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml
- docs/milestones/v0.91.7/REVIEW_AND_VALIDATION_CHECKLIST_v0.91.7.md
- docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md
- docs/reviews/v0.91.7/remaining-sprints-5403/
- docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md

## Non Goals

- Do not execute WP-19 external review
- Do not fix findings as part of the review issue unless separately assigned
- Do not claim release ceremony readiness
- Do not use AWS or remote paid validation lanes for review preparation
- Do not rerun broad Rust or runtime suites unless the review execution changes code or tooling
