# Structured Intent Prompt

Template: 1.0.0

Issue: 5733

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Map every claimed v0.91.8 feature or launch surface to owning issue, exact proof or explicit blocker/non-claim/deferred disposition, and public claim boundary.

## Required Outcome

DEMO_MATRIX_v0.91.8.md and FEATURE_PROOF_COVERAGE_v0.91.8.md agree with current issue and proof truth, backed by a focused deterministic validator.

## Scope

- docs/milestones/v0.91.8/DEMO_MATRIX_v0.91.8.md
- docs/milestones/v0.91.8/FEATURE_PROOF_COVERAGE_v0.91.8.md
- docs/milestones/v0.91.8/review/wp15_demo_matrix_5733
- adl/tools/validate_v0918_demo_matrix.py

## Authority

- This issue owns matrix and feature-proof coverage truth only.
- Owning issues retain authority over runtime, demo, deployment, podcast, Unity, and v0.92 handoff execution proof.
- Planning, fixtures, screenshots, retained packets, and metadata must not be promoted into runtime proof.

## Assumptions

- none

## Operator Constraints

- never touch or switch primary main
- use typed C-SDLC v2 lifecycle
- use FastWork for this worktree
- do not rerun demos owned by other issues
- do not claim release readiness or v0.92 activation readiness
