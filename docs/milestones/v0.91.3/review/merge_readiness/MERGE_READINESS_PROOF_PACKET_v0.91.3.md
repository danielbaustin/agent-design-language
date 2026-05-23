# v0.91.3 Merge-Readiness Proof Packet

## Scope

Provide the first bounded `WP-06` governed merge-readiness gate surface for the
`ct_demo_001` transition.

This packet proves that ADL can preserve a structured, reviewable snapshot of
GitHub issue, PR, CI, branch, review, and evidence truth inside one merge-gate
record without pretending to replace human merge authority or live GitHub
reconciliation.

## Tracked Proof Surfaces

- `docs/milestones/v0.91.3/review/merge_readiness/ct_demo_001_merge_gate.md`
- `docs/milestones/v0.91.3/review/merge_readiness/ct_demo_001_merge_gate_snapshot.json`
- `adl/tools/validate_merge_readiness_packet.py`
- `adl/tools/test_merge_readiness_packet.sh`

## Validation

- `python3 adl/tools/validate_merge_readiness_packet.py docs/milestones/v0.91.3/review/merge_readiness`
- `bash adl/tools/test_merge_readiness_packet.sh`
- `cargo test --manifest-path adl/Cargo.toml cognitive_transition_manifest_fixture_points_at_wp06_merge_gate -- --nocapture`

## What This Proves

- the transition manifest can link to a tracked merge-readiness gate surface
- the gate record and structured snapshot capture issue, PR, CI, review, and
  evidence truth in one bounded location
- the gate fails closed when required merge-truth sections disappear
- the gate fails closed when the structured snapshot drifts from the tracked
  merge-gate markdown or linked tracked artifacts
- the gate preserves human merge review as a required authority boundary

## What This Does Not Prove

- automatic merging
- live GitHub API reconciliation at validation time
- bypass of GitHub protection rules
- replacement of Sprint 4 quality gate or release closeout
- full v0.91.4 merge-readiness hardening
