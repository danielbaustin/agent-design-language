# Structured Task Prompt

Template: 1.0.0

Issue: 5733

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Reconcile the two canonical v0.91.8 coverage docs and add focused deterministic validation.

## Deliverables

- updated v0.91.8 demo matrix
- updated v0.91.8 feature-proof coverage index
- machine-readable coverage ledger or deterministic validator
- explicit live demo, retained proof, blocker, non-claim, and deferred classifications
- bounded exact-head review and ready PR closing #5733

## Acceptance

1. AC-1: Every matrix row names an owning issue and exact evidence or an explicit blocker/non-claim/deferred disposition.
2. AC-2: #5354 convergence output is consumed as input; #5733 does not rerun or replace integrated convergence.
3. AC-3: Demo evidence, runtime proof, documentation-only evidence, and planned work are not conflated.
4. AC-4: Unity, Runtime v3, ADL v2, C-SDLC v2, Observatory, distributed workcell, Podcast Studio, and v0.92 handoff rows preserve actual claim boundaries.
5. AC-5: Matrix and feature-proof coverage have no contradictory status or ownership entries.
6. AC-6: Focused validation and one bounded pre-PR review pass at the exact publication revision.

## Dependencies

- #5354 final reviewed convergence packet
- current accepted revisions and proof packets named by the existing matrix
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml

## Inputs

- docs/milestones/v0.91.8/DEMO_MATRIX_v0.91.8.md
- docs/milestones/v0.91.8/FEATURE_PROOF_COVERAGE_v0.91.8.md
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- .csdlc/evidence/5354/convergence-proof.v1.json
- .csdlc/issues/5354
- .csdlc/issues/5605

## Non Goals

- implementing or rerunning product demos owned by other issues
- converting planning, fixtures, screenshots, or metadata into runtime proof
- claiming release readiness or v0.92 activation readiness
- blocking this issue on typed closeout receipts
