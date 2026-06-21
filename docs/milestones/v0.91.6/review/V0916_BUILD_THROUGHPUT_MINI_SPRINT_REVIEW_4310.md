# v0.91.6 Build Throughput Improvements Mini-Sprint Review

Status: `retained_review_packet`
Date: 2026-06-21
Sprint umbrella: `#4310`
Execution packet:
`docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_SEP_BUILD_THROUGHPUT_MINI_SPRINT_4310.md`
Activity log:
`docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_BUILD_THROUGHPUT_MINI_SPRINT_ACTIVITY_LOG_4310.md`

This review summarizes the bounded `#4310` mini-sprint child wave. It does not
claim a broader build-system refactor, CI replacement, or live AWS rollout.

## Findings

No sprint-level blockers or regressions remain in the landed child wave.

### P2: Local child closeout truth drift had to be normalized before umbrella closeout

After the child PRs merged and the GitHub issues closed, the canonical ignored
`.adl` root issue bundle still showed stale child `SRP` and `SOR` state for
`#4311` through `#4316`. The child wave itself was complete, but umbrella
closeout could not proceed truthfully until that records-hygiene drift was
normalized. This is process residue, not a scope blocker in the landed
throughput work.

### P3: Issue-level elapsed and token metrics remain mostly unknown across the child wave

Validation seconds were captured for each child issue, but elapsed seconds and
token totals remain mostly `unknown` in the issue-local `SOR` records. The
sprint can still close truthfully because the retained artifacts and validation
results are present, but the metrics substrate remains incomplete.

## Child Issue Closure Truth

| Issue | PR | State | Evidence |
|---|---|---|---|
| `#4315` | `#4346` | closed / merged | PR `#4346` merged and the retained measurement packet is present at `docs/milestones/v0.91.6/review/build_throughput/BUILD_THROUGHPUT_MEASUREMENT_4315.md`. |
| `#4311` | `#4348` | closed / merged | PR `#4348` merged and the retained `sccache` packet is present at `docs/milestones/v0.91.6/review/build_throughput/SCCACHE_LOCAL_VALIDATION_4311.md`. |
| `#4312` | `#4349` | closed / merged | PR `#4349` merged and the retained linker packet is present at `docs/milestones/v0.91.6/review/build_throughput/RUST_LINKER_LOCAL_VALIDATION_4312.md`. |
| `#4313` | `#4350` | closed / merged | PR `#4350` merged and the retained target-relocation packet is present at `docs/milestones/v0.91.6/review/build_throughput/CARGO_TARGET_DIR_RELOCATION_4313.md`. |
| `#4314` | `#4353` | closed / merged | PR `#4353` merged and the retained cleanup-policy packet is present at `docs/milestones/v0.91.6/review/build_throughput/SAFE_BUILD_ARTIFACT_CLEANUP_4314.md`. |
| `#4316` | `#4355` | closed / merged | PR `#4355` merged and the retained CodeBuild evaluation packet is present at `docs/milestones/v0.91.6/review/build_throughput/CODEBUILD_REMOTE_VALIDATION_EVALUATION_4316.md`. |

## Scope Check

The completed child wave is exactly:

- `#4315`
- `#4311`
- `#4312`
- `#4313`
- `#4314`
- `#4316`

No additional implementation work was absorbed into the sprint umbrella.

## Outcome Summary

- `#4315` established the measurement baseline and hotspot framing that
  justified the rest of the sprint.
- `#4311` and `#4312` produced measured local opt-in recommendations rather
  than hard-wiring host-specific build defaults into tracked repo state.
- `#4313` and `#4314` produced a coherent local target-layout and cleanup
  policy story.
- `#4316` remained evaluation-only and concluded with a bounded deferral:
  no CodeBuild pilot in `v0.91.6`.

## Validation And Review Evidence

- Child issue validation seconds recorded in issue-local `SOR` cards sum to
  `1007` seconds across the sprint:
  `346` (`#4315`) + `433` (`#4311`) + `139` (`#4312`) + `84` (`#4313`) +
  `3` (`#4314`) + `2` (`#4316`).
- Each child issue now reports `lifecycle_state: "closed"` and
  `ready_status: "PASS"` through the repo-native doctor path.
- Each child issue has a retained report under
  `docs/milestones/v0.91.6/review/build_throughput/`.
- The local child `SRP` and `SOR` closeout truth was normalized before this
  umbrella review so the sprint closeout does not rely on stale local state.

## Non-Claims

- This review does not claim a repo-wide build refactor or CI throughput
  replacement landed in the sprint.
- This review does not claim remote validation infrastructure was implemented;
  `#4316` remained an evaluation-only lane.
- This review does not treat missing elapsed/token metrics as zero.
