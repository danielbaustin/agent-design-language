# Structured Task Prompt

Template: 1.0.0

Issue: 5842

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver quality gate that blocks internal review until every indexed v0.92 feature is landed with accepted exact-revision proof.

## Deliverables

- quality gate that blocks internal review until every indexed v0.92 feature is landed with accepted exact-revision proof
- feature-completion matrix, quality-gate record, platform proof, and blocker report

## Acceptance

1. AC-1: Every declared predecessor is merged, terminal, claim-free, ancestral, and observed from live GitHub plus canonical typed truth at the pinned gate SHA.
2. AC-2: Every canonical v0.92 feature and supporting critical path has exactly one matrix row with owner, implementation paths, reviewed head, PR/merge, validation/negative/integration/platform evidence, and typed terminal reference.
3. AC-3: Planned, open, unknown, fixture-only, receipt-only, demo-mode, synthetic, substituted-provider, stale-review, non-ancestral, or platform-unproven rows are rejected as blockers.
4. AC-4: Documentation/planning rows are source-grounded and executable; tooling/cleanup rows show measured value and regression safety; runtime/provider/consumer rows prove real production paths.
5. AC-5: The retained matrix, gate record, and blocker report are schema-valid, reproducible, findings-first, and block WP-25 while any required row is not accepted.
6. AC-6: One exact-head independent review validates the gate logic and all row dispositions with no unresolved actionable finding.

## Dependencies

- WP-04
- WP-05
- WP-06
- WP-07
- WP-13A
- WP-20
- WP-21
- WP-21A

## Inputs

- Live issue #5842 and exact predecessor issues WP-04, WP-05, WP-06, WP-07, WP-13A, WP-20, WP-21, and WP-21A
- docs/milestones/v0.92/features/README.md, FEATURE_PROOF_COVERAGE_v0.92.md, QUALITY_GATE_v0.92.md, DEMO_MATRIX_v0.92.md, and MILESTONE_CHECKLIST_v0.92.md
- Canonical .csdlc records, receipts, issue/PR state, required checks, reviewed heads, merge SHAs, and retained proof

## Non Goals

- Repairing product features or waiving incomplete rows to preserve schedule
- Crediting fixtures, receipts, demo mode, synthetic success, substituted providers, stale review, or issue closure as delivery
- WP-23 docs alignment, WP-25 review execution, or release approval
