# v0.91.6 Build Throughput Improvements Mini-Sprint Activity Log

Status: `child_wave_complete`
Date: 2026-06-21
Sprint umbrella: `#4310`

This log records sprint-preparation events for the bounded child wave `#4315`,
`#4311`, `#4312`, `#4313`, `#4314`, and `#4316`.

## 2026-06-20 Preparation

- Verified sprint umbrella `#4310` with `pr.sh doctor`; result:
  `DOCTOR_STATUS=PASS`, `LIFECYCLE_STATE=pre_run`,
  `CARD_LIFECYCLE_PR_RUN_READINESS=ready`.
- Verified child issues `#4311`, `#4312`, `#4313`, `#4314`, `#4315`, and
  `#4316` each pass `pr.sh doctor` with no preflight blockers, no open PRs,
  and `CARD_LIFECYCLE_PR_RUN_READINESS=ready`.
- Read the sprint source prompt, `STP`, `SPP`, and the build-throughput source
  doc at `.adl/docs/TBD/ADL_BUILD_IMPROVEMENTS.md`.
- Added the sprint execution packet at
  `docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_SEP_BUILD_THROUGHPUT_MINI_SPRINT_4310.md`.
- Declared the sprint review path at
  `docs/milestones/v0.91.6/review/V0916_BUILD_THROUGHPUT_MINI_SPRINT_REVIEW_4310.md`.
- Declared measurement-first sequencing:
  `#4315` first, then local tuning lanes, then cleanup policy, with `#4316`
  remaining evaluation-only and safe to defer truthfully if needed.

## Open Watch Items

- No child execution watch items remain for the bounded `#4310` wave.
- Umbrella closeout remains: publish the final sprint packet and review update,
  record final `#4310` SOR truth, and merge the umbrella closeout work.

## 2026-06-20 To 2026-06-21 Child Execution

- Completed `#4315` through PR `#4346`; the retained report established the
  build-throughput baseline, warm-build comparisons, and hotspot framing for
  the rest of the sprint.
- Completed `#4311` through PR `#4348`; the retained report recorded measured
  local `sccache` wins as an opt-in recommendation.
- Completed `#4312` through PR `#4349`; the retained report recorded measured
  `rust-lld` wins as an opt-in recommendation on the actual host platform.
- Completed `#4313` through PR `#4350`; the retained report recorded the
  target-directory relocation strategy and per-worktree isolation guidance.
- Completed `#4314` through PR `#4353`; the retained report recorded the
  report-first cleanup policy aligned with the target-relocation truth.
- Completed `#4316` through PR `#4355`; the retained report recommended no
  CodeBuild pilot in `v0.91.6` and preserved the no-live-AWS boundary.
- Verified each child issue now reports `lifecycle_state: "closed"` and
  `ready_status: "PASS"` through the repo-native doctor path with no open PRs.
- Recorded one sprint-level process remediation note: child GitHub closure
  completed before the canonical ignored `.adl` root issue bundle reflected
  terminal `SRP`/`SOR` truth. That local closeout drift was normalized before
  the umbrella closeout proceeded.
