# Release Readiness - v0.91.1

## Status

Not release ready.

This document is the compact reviewer-entry surface for the active `v0.91.1`
release tail. Internal review and third-party review are complete. The
accepted-finding remediation disposition is also complete. The milestone is not
complete until the release ceremony finishes.

## Review Entry Points

- [../../../CHANGELOG.md](../../../CHANGELOG.md)
- [../../../README.md](../../../README.md)
- [../../../adl/Cargo.toml](../../../adl/Cargo.toml)
- [README.md](README.md)
- [WBS_v0.91.1.md](WBS_v0.91.1.md)
- [SPRINT_v0.91.1.md](SPRINT_v0.91.1.md)
- [WP_ISSUE_WAVE_v0.91.1.yaml](WP_ISSUE_WAVE_v0.91.1.yaml)
- [DEMO_MATRIX_v0.91.1.md](DEMO_MATRIX_v0.91.1.md)
- [FEATURE_PROOF_COVERAGE_v0.91.1.md](FEATURE_PROOF_COVERAGE_v0.91.1.md)
- [QUALITY_GATE_v0.91.1.md](QUALITY_GATE_v0.91.1.md)
- [MILESTONE_CHECKLIST_v0.91.1.md](MILESTONE_CHECKLIST_v0.91.1.md)
- [RELEASE_PLAN_v0.91.1.md](RELEASE_PLAN_v0.91.1.md)
- [RELEASE_EVIDENCE_v0.91.1.md](RELEASE_EVIDENCE_v0.91.1.md)
- [RELEASE_NOTES_v0.91.1.md](RELEASE_NOTES_v0.91.1.md)
- [ADL_v0.91.1_THIRD_PARTY_REVIEW_HANDOFF.md](ADL_v0.91.1_THIRD_PARTY_REVIEW_HANDOFF.md)
- [review/WP22_REMEDIATION_QUEUE.md](review/WP22_REMEDIATION_QUEUE.md)
- [END_OF_MILESTONE_REPORT_v0.91.1.md](END_OF_MILESTONE_REPORT_v0.91.1.md)
- [NEXT_MILESTONE_HANDOFF_v0.91.1.md](NEXT_MILESTONE_HANDOFF_v0.91.1.md)
- [NEXT_MILESTONE_REVIEW_PASS_v0.91.1.md](NEXT_MILESTONE_REVIEW_PASS_v0.91.1.md)
- [SPRINT_2_CLOSEOUT_v0.91.1.md](SPRINT_2_CLOSEOUT_v0.91.1.md)
- [SPRINT_3_CLOSEOUT_v0.91.1.md](SPRINT_3_CLOSEOUT_v0.91.1.md)

## Current Issue State

- `WP-01` through `WP-23A` are closed with landed docs truth
- `WP-24` remains open for the final ceremony band
- Sprint 2 is complete
- Sprint 3 runtime/comms/inhabitant work is complete through `WP-17`
- `WP-18` records the milestone quality-gate posture
- `WP-19` aligns milestone docs, reviewer-entry surfaces, root README,
  changelog, and active version truth before review

## Current Blockers

- release ceremony is pending

## Ceremony Gate Status

- historical closed-issue closeout residue that previously blocked `WP-24`
  has been normalized in `#2998`
- `bash adl/tools/check_milestone_closed_issue_sor_truth.sh --version v0.91.1`
  now passes against the full closed `v0.91.1` issue set
- from the bound `WP-24` worktree on
  `codex/2846-v0-91-1-wp-24-release-ceremony`,
  `bash adl/tools/release_ceremony.sh --version v0.91.1 --target-branch codex/2846-v0-91-1-wp-24-release-ceremony --allow-dirty`
  now passes its full preflight without `--skip-sor-gate`
- the remaining release-tail dependency is the ceremony itself, not stale local
  lifecycle-record truth

## Version Truth

- active milestone package: `v0.91.1`
- crate version: `0.91.1`
- `v0.91` is prior and structurally complete
- `v0.91.2` is the follow-on tooling/productization milestone
