# Structured Task Prompt

Template: 1.0.0

Issue: 5840

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Reconcile and validate v0.92 demo/AEE proof metadata after #5836, #5837, #5838, and #5839; do not implement child features, absorb WP-21/WP-21A reduction work, or declare milestone release readiness.

## Deliverables

- Reconciled demo matrix and feature coverage table
- Exact-revision AEE and artifact index
- Corrected WP-20 versus WP-21/WP-21A ownership
- Fail-closed coverage validator and focused tests

## Acceptance

1. AC-1: Matrix, coverage, activation-ledger, and artifact-index rows agree on owner, command, status, and exact revision.
2. AC-2: Every accepted demo/AEE claim has positive artifact, required negative artifact, platform/credential posture, review state, and non-claims.
3. AC-3: WP-20 demo/proof ownership is corrected without absorbing WP-21/WP-21A reduction work.
4. AC-4: The validator rejects missing paths, duplicate ownership, planned-as-passed status, synthetic proof, and unsupported platform claims.
5. AC-5: Exact-head review has no unresolved actionable finding.

## Dependencies

- #5836 / WP-18 complete
- #5837 / WP-18A complete
- #5838 / WP-18B complete
- #5839 / WP-19 complete

## Inputs

- docs/milestones/v0.92/DEMO_MATRIX_v0.92.md
- docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md
- docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md
- docs/milestones/v0.92/QUALITY_GATE_v0.92.md
- docs/milestones/v0.91.6/features/AEE_MEMORY_ACP_BRIDGE_ACCOUNTING_v0.91.6.md

## Non Goals

- Feature implementation or rerouting child ownership
- Repository-wide reduction or Rust refactoring
- Synthetic proof generation
- Milestone quality, publication, or release approval
