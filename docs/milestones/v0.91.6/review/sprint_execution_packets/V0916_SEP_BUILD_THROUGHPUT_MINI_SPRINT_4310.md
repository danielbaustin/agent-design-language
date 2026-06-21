# v0.91.6 Build Throughput Improvements Mini-Sprint SEP

Status: `closed_child_wave_ready_for_umbrella_closeout`
Date: 2026-06-21
Sprint umbrella: `#4310`
Execution mode: `hybrid`
Sprint review path:
`docs/milestones/v0.91.6/review/V0916_BUILD_THROUGHPUT_MINI_SPRINT_REVIEW_4310.md`

This Sprint Execution Packet records the bounded execution contract for the
build-throughput improvements mini-sprint. It does not replace issue-local
`SIP -> STP -> SPP -> SRP -> SOR` truth, and it does not authorize scope
outside the listed child issues.

## Sprint Goal

Make ADL local Rust build and validation costs more observable, reduce repeated
local rebuild cost, and keep build artifacts from filling the system disk
without hiding a larger workspace refactor inside this mini-sprint.

## Scope Boundary

In scope:

- `#4311`
- `#4312`
- `#4313`
- `#4314`
- `#4315`
- `#4316`
- umbrella readiness, review, and closeout truth for `#4310`

Out of scope:

- large Rust workspace refactors
- CI-wide or production validation workflow replacement
- distributed build rollout
- live AWS resource creation for `#4316`
- secret handling changes outside the listed child issues

## Child Issue Wave

| Issue | Role | Status | Notes |
|---|---|---|---|
| `#4315` | baseline measurement and hotspot report | completed | PR `#4346` merged; retained measurement packet established the sprint baseline and hotspot framing. |
| `#4311` | `sccache` evaluation and local enablement guidance | completed | PR `#4348` merged; result is a measured local opt-in recommendation, not mandatory tracked repo state. |
| `#4312` | faster-linker evaluation | completed | PR `#4349` merged; result is a measured `rust-lld` local opt-in recommendation on the actual developer platform. |
| `#4313` | target-dir relocation strategy | completed | PR `#4350` merged; result is a measured local relocation strategy and per-worktree target layout guidance. |
| `#4314` | safe cleanup policy for stale build artifacts | completed | PR `#4353` merged; result is a report-first cleanup policy aligned with the target-relocation truth. |
| `#4316` | remote validation / CodeBuild evaluation | completed | PR `#4355` merged; evaluation recommends no CodeBuild pilot in `v0.91.6`. |

## Recommended Execution Order

1. `#4315` completed first through PR `#4346` to establish the baseline and hotspot framing.
2. `#4311` completed through PR `#4348`.
3. `#4312` completed through PR `#4349`.
4. `#4313` completed through PR `#4350`.
5. `#4314` completed through PR `#4353` after the target-layout guidance existed.
6. `#4316` completed through PR `#4355` as an evaluation-only lane with no live AWS mutation.
7. `#4310` may close after the umbrella SOR and retained review packet record the completed child wave truthfully.

## Candidate Parallel Lanes

| Lane | Issues | Candidate reason | Risk to watch |
|---|---|---|---|
| local build tuning | `#4311`, `#4312` | Cache and linker evaluation can be measured independently after the common baseline exists. | Both may touch shared build/setup docs or shell guidance and need reconciliation before publication. |
| local artifact management | `#4313`, `#4314` | These are conceptually related and may share findings. | Not safe by default because cleanup guidance must not outrun target-relocation truth. |
| remote evaluation | `#4316` | Evaluation-only documentation lane with no live AWS mutation. | Must stay source-backed, secret-safe, and clearly separate from local build improvements. |

## Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| baseline-independent local tuning | `#4311`, `#4312` | After `#4315`, cache and linker experiments can proceed independently if they do not edit the same setup/report surfaces. | Reconcile final measurement vocabulary, command set, and opt-in guidance before PR publication. |
| remote evaluation lane | `#4316` | `#4316` is evaluation-only and should stay in documentation/review surfaces rather than local build plumbing. | Keep AWS docs citations explicit, do not create resources, and do not present evaluation output as implemented runtime capacity. |

Actual outcome:

- The child wave landed sequentially in practice even though some lanes were
  conceptually parallel-safe.
- No parallelism claim is needed to justify the completed scope; the retained
  outputs are sufficient and the serial execution path remained within plan.

## Serial Gates

| Gate | Blocks | Exit condition |
|---|---|---|
| sprint readiness gate | all child execution | Satisfied on 2026-06-20 before child execution began. |
| measurement gate | `#4311`, `#4312`, `#4313`, `#4316` | Satisfied by merged PR `#4346` for `#4315`. |
| artifact-layout gate | `#4314` | Satisfied by merged PR `#4350` for `#4313`. |
| closeout gate | `#4310` closure | Child GitHub closure, local child `SRP`/`SOR` truth, retained sprint review, and umbrella `SOR` must all agree. The child-wave gate is now satisfied. |

## Parallelism Outcome Plan

- Preferred actual start is effectively sequential through `#4315`.
- After the measurement gate, use separate issue-bound sessions only where the
  touched files are still disjoint and the SEP lane remains truthful.
- If `#4311`, `#4312`, and `#4313` collide in shared docs or helper scripts,
  collapse back to sequential execution rather than forcing parallelism.
- Treat `#4314` as serial behind `#4313` unless a fresh truth check proves the
  cleanup surface can land independently.
- Treat `#4316` as the easiest lane to defer if local build wins consume the
  sprint budget; deferral is acceptable if recorded truthfully.

## Watcher Policy

- Every child issue PR must have an issue-local watcher or janitor handoff for
  checks, review state, mergeability, and closeout readiness.
- Healthy PR-open states are watcher-owned lifecycle states, not sprint stop
  points.
- Blocked, flaky, skipped, deferred, or out-of-scope findings must be recorded
  in the child issue cards and the sprint activity log.
- A child issue may not be treated as complete until GitHub issue/PR state,
  local lifecycle cards, and closeout truth agree.
- Child issue sessions must create the issue-bound session goal after bind and
  readiness succeed and before implementation starts, carrying both sprint and
  child issue context.

## PVF Notes

- `#4315`: prove reproducible enough baseline, incremental, focused-test, and
  owner-lane measurements without widening into refactor work.
- `#4311`: prove cache hit-rate and repeat-build impact or record an
  inconclusive result truthfully.
- `#4312`: prove linker observations on the actual developer platform rather
  than importing Linux assumptions.
- `#4313`: prove relocated-target builds work and keep worktree isolation safe.
- `#4314`: prove report-first cleanup guidance and one safe cleanup path
  without deleting active work.
- `#4316`: prove source-backed evaluation only; no AWS credentials, account
  setup, or resource mutation claims.

Validation must stay focused per child issue. Aggregate sprint proof must not
hide failed, pending, blocked, skipped, inconclusive, or deferred lanes.

## Sprint Review And Closeout

Sprint review must collect child issue PRs, changed implementation or
documentation surfaces, validation notes, blocked/deferred findings, and final
throughput recommendations. The review artifact is tracked at
`docs/milestones/v0.91.6/review/V0916_BUILD_THROUGHPUT_MINI_SPRINT_REVIEW_4310.md`.

Closeout must verify:

- all six child issues are closed or truthfully deferred;
- each completed child has PR, validation, SRP, SOR, and closeout truth
  aligned;
- the final report distinguishes proven local wins from inconclusive or
  deferred remote-build evaluation;
- no large workspace refactor or CI rollout was hidden inside the mini-sprint;
- the `#4310` SOR records final sprint-level integration truth.

## Current Preparation Notes

- Child issue execution completed for all six listed children on 2026-06-20
  through 2026-06-21.
- Live GitHub truth now reports `#4311` through `#4316` as closed with no open
  PRs remaining.
- Local child closeout truth initially drifted after merge because the
  canonical ignored `.adl` issue bundle still showed stale `SRP`/`SOR` state.
  That records-hygiene drift was normalized on 2026-06-21 before umbrella
  closeout continued.
- The bounded sprint result is five local throughput/report improvements plus
  one evidence-backed remote-evaluation deferral; no large workspace refactor
  or live AWS rollout was hidden in the sprint.
