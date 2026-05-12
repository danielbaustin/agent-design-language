# Release Evidence - v0.91.1

## Purpose

Track the milestone package, proof surfaces, and evidence gaps required for a
truthful `v0.91.1` release decision.

## Current Core Package

- [../../../CHANGELOG.md](../../../CHANGELOG.md)
- [../../../README.md](../../../README.md)
- [../../../adl/Cargo.toml](../../../adl/Cargo.toml)
- [README.md](README.md)
- [WBS_v0.91.1.md](WBS_v0.91.1.md)
- [SPRINT_v0.91.1.md](SPRINT_v0.91.1.md)
- [WP_ISSUE_WAVE_v0.91.1.yaml](WP_ISSUE_WAVE_v0.91.1.yaml)
- [WP_EXECUTION_READINESS_v0.91.1.md](WP_EXECUTION_READINESS_v0.91.1.md)
- [DEMO_MATRIX_v0.91.1.md](DEMO_MATRIX_v0.91.1.md)
- [FEATURE_PROOF_COVERAGE_v0.91.1.md](FEATURE_PROOF_COVERAGE_v0.91.1.md)
- [QUALITY_GATE_v0.91.1.md](QUALITY_GATE_v0.91.1.md)
- [MILESTONE_CHECKLIST_v0.91.1.md](MILESTONE_CHECKLIST_v0.91.1.md)
- [RELEASE_NOTES_v0.91.1.md](RELEASE_NOTES_v0.91.1.md)
- [ADL_v0.91.1_THIRD_PARTY_REVIEW_HANDOFF.md](ADL_v0.91.1_THIRD_PARTY_REVIEW_HANDOFF.md)
- [review/WP22_REMEDIATION_QUEUE.md](review/WP22_REMEDIATION_QUEUE.md)
- [END_OF_MILESTONE_REPORT_v0.91.1.md](END_OF_MILESTONE_REPORT_v0.91.1.md)
- [NEXT_MILESTONE_HANDOFF_v0.91.1.md](NEXT_MILESTONE_HANDOFF_v0.91.1.md)
- [NEXT_MILESTONE_REVIEW_PASS_v0.91.1.md](NEXT_MILESTONE_REVIEW_PASS_v0.91.1.md)
- [SPRINT_2_CLOSEOUT_v0.91.1.md](SPRINT_2_CLOSEOUT_v0.91.1.md)
- [SPRINT_3_CLOSEOUT_v0.91.1.md](SPRINT_3_CLOSEOUT_v0.91.1.md)

## Current Landed Evidence

- `WP-02` runtime/polis architecture package
- `WP-03` lifecycle-state implementation
- `WP-04` observatory active surface implementation
- `WP-05` citizen standing implementation
- `WP-06` citizen state substrate
- `WP-07` through `WP-12` memory, ToM, capability, intelligence, governed
  learning, and ANRM/Gemma runtime proof surfaces
- `WP-13` ACIP hardening
- `WP-14` A2A adapter boundary
- `WP-15` runtime inhabitant proof
- `WP-16` observatory-visible flagship demo
- `WP-17` milestone demo matrix and feature-proof coverage closeout
- `WP-18` milestone quality-gate record
- `WP-19` review-ready docs package and reviewer-entry surface alignment
- `WP-20` internal review packet
- `WP-21` external / third-party review handoff and zero-finding record
- `WP-22` accepted-finding disposition record with zero accepted findings
- `WP-23` next-milestone planning and downstream handoff record
- `WP-23A` next-milestone review-pass record

## In-Flight Evidence

- merged `WP-23A` next-milestone review-pass record from PR `#2996`
- `#2998` closeout-truth normalization evidence for the historical
  release-ceremony blocker
- `WP-24` release-ceremony candidate package and end-of-milestone report

## Ceremony Preflight Evidence

- `bash adl/tools/check_milestone_closed_issue_sor_truth.sh --version v0.91.1`
  passes after restoring missing canonical local bundles and normalizing stale
  completed-phase `SIP`/`SOR` truth
- from the bound `WP-24` worktree on
  `codex/2846-v0-91-1-wp-24-release-ceremony`,
  `bash adl/tools/release_ceremony.sh --version v0.91.1 --target-branch codex/2846-v0-91-1-wp-24-release-ceremony`
  now passes preflight without `--skip-sor-gate`
- the prior blocker was historical root-bundle reconciliation drift, not
  missing landed milestone work

## Evidence Still Missing

- final merge of the `WP-24` ceremony package
