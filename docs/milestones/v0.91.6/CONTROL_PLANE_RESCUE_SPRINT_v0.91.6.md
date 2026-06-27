# v0.91.6 Control-Plane Rescue Sprint

## Purpose

This sprint is the release-rescue gate for `v0.91.6`.

It exists because the milestone is no longer blocked by lack of individual
workflow components. It is blocked by integration: PR finish, prompt cards,
watchers, validation lanes, closeout truth, and build capacity must work as one
boring path before the release tail resumes.

The sprint does not add product scope. It clears the path for finishing the
milestone truthfully.

## Sprint Issue

- Umbrella: `#4588`
- Status: complete rescue gate; release-tail continuation resumed under `#4604`
- Execution mode: sequential core with prep-scout and watcher side lanes
- Release-tail resume gate: satisfied by the retained `#4588` sprint truth;
  the closeout-tail wave is now governed by `#4604`.

## Current Sprint State

- Gate 0 `#4539`: complete. Watcher classified the merged issue as
  `closeout_needed`; SRP review truth was repaired from SOR evidence; repo-native
  `pr.sh closeout 4539 --version v0.91.6` passed.
- Gate 1 `#4557`: complete. Watcher classified the merged issue as
  `closeout_needed`; repo-native `pr.sh closeout 4557 --version v0.91.6`
  passed.
- Gate 2 `#4584`: closed with reconciled evidence. The retained watcher packet
  conservatively classifies the closed issue as `closeout_needed`, while the
  retained doctor packet records lifecycle `closed`, ready status `PASS`, and
  doctor status `PASS`; the stale execution claim was released during `#4588`
  sprint closeout hygiene.
- Gate 3 `#4585`: closed with reconciled evidence. The retained watcher packet
  conservatively classifies the closed issue as `closeout_needed`, while the
  retained doctor packet records lifecycle `closed`, ready status `PASS`, and
  doctor status `PASS`; the stale execution claim was released during `#4588`
  sprint closeout hygiene.
- Gate 3A `#4593`: closed with reconciled evidence. This narrow pr-janitor
  validation repair became part of the rescue path after the validation burden
  surfaced during PR repair. The retained watcher packet conservatively
  classifies the closed issue as `closeout_needed`, while the retained doctor
  packet records lifecycle `closed`, ready status `PASS`, and doctor status
  `PASS`; the stale watcher claim was released during `#4588` sprint closeout
  hygiene.
- Gate 4 `#4586`: complete. PR `#4600` merged, issue `#4586` closed, and
  repo-native `pr.sh closeout 4586 --version v0.91.6` passed STP/SIP/SOR
  validation after merge.
- Gate 5 `#4587`: complete. PR `#4599` merged, issue `#4587` closed, and
  repo-native `pr.sh closeout 4587 --version v0.91.6` passed STP/SIP/SOR
  validation after merge.
- Gate 6 `#4590`: complete. PR `#4596` merged, issue `#4590` closed, and
  repo-native `pr.sh closeout 4590 --version v0.91.6` passed STP/SIP/SOR
  validation after merge.
- Gate 7 `#4591`: complete. PR `#4597` merged, issue `#4591` closed, and
  repo-native `pr.sh closeout 4591 --version v0.91.6` passed STP/SIP/SOR
  validation after merge.
- Follow-on blocker `#4598`: merged and closed through PR `#4601`.
  Local finish validation passed, including `git diff --check`, `cargo fmt`,
  C-SDLC owner validation, SOR contract validation, and the focused
  `pr_cmd_finish` lane (`162/162` passed). Publication proved the token-aware
  push path far enough to create PR `#4601`; the push emitted
  `fatal: failed to store: 100001`, but branch publication continued without
  exposing the token. The first merge tail timed out after about fifteen
  minutes because `adl-ci` remained `IN_PROGRESS`; a later watcher pass saw
  `adl-ci` complete successfully, and rerunning `pr.sh finish --merge` merged
  PR `#4601` and closed `#4598`. The explicit `pr.sh closeout 4598` pass then
  validated STP/SIP/SOR and confirmed the worktree was already pruned.
- Current active work: child issue closeout is complete for the rescue sprint.
  The remaining `#4588` work is to publish this consolidated sprint truth and
  route release-tail continuation from the merged evidence.
- New blocker captured: `#4590` owns independent-binary resolution for
  workflow-critical ADL commands so normal finish, validation, and closeout
  paths stop invoking Cargo as ordinary control flow.
- New adoption issue captured: `#4591` owns the shared operating docs and
  installed skill guidance refresh so every session sees the current watcher,
  prep-scout, scheduler, binary-first, and closeout contract.
- Prep status: `#4590` and `#4591` have issue-local SIP/STP/SPP/VPP/SRP
  readiness after prompt-template renderer repairs to explicit planning and
  validation budgets, and both have been published as ready PRs.

## Ordered Rescue Work

| Order | Issue | Role | Completion Requirement | Wait-State Owner |
| --- | --- | --- | --- | --- |
| 0 | `#4539` | Operational completion gate closeout normalization | COMPLETE: watcher packet retained; SRP truth repaired; closeout passed. | complete |
| 1 | `#4557` | Publication validation lane contract | COMPLETE: watcher packet retained; closeout passed. | complete |
| 2 | `#4584` | Older SOR import compatibility | CLOSED/RECONCILED: watcher says `closeout_needed`; retained doctor packet says lifecycle `closed`, ready `PASS`, doctor `PASS`; stale claim released. | complete for release-tail unless operator requires rerun |
| 3 | `#4585` | Watcher operationalization | CLOSED/RECONCILED: watcher says `closeout_needed`; retained doctor packet says lifecycle `closed`, ready `PASS`, doctor `PASS`; stale claim released. | complete for release-tail unless operator requires rerun |
| 3A | `#4593` | Narrow pr-janitor validation repair | CLOSED/RECONCILED: watcher says `closeout_needed`; retained doctor packet says lifecycle `closed`, ready `PASS`, doctor `PASS`; stale watcher claim released. | complete for release-tail unless operator requires rerun |
| 4 | `#4586` | Fast SRP/SOR fact sync | COMPLETE: PR `#4600` merged; issue `#4586` closed; closeout validated STP/SIP/SOR. | complete |
| 5 | `#4587` | Remote build relief | COMPLETE: PR `#4599` merged; issue `#4587` closed; closeout validated STP/SIP/SOR. | complete |
| 6 | `#4590` | Independent ADL workflow binaries | COMPLETE: PR `#4596` merged; issue `#4590` closed; closeout validated STP/SIP/SOR. | complete |
| 7 | `#4591` | C-SDLC operating docs and skills refresh | COMPLETE: PR `#4597` merged; issue `#4591` closed; closeout validated STP/SIP/SOR. | complete |
| 8 | `#4598` | Deterministic finish-ready publication and git push auth | COMPLETE: PR `#4601` merged; issue `#4598` closed; closeout validated STP/SIP/SOR; worktree already pruned. Retain observations about `fatal: failed to store: 100001` during push and the long `adl-ci` wait. | complete |

## Golden Path

Every rescue child must follow this path:

```text
session claim -> pr run -> issue goal -> focused validation -> bounded review -> pr finish -> pr watch -> pr closeout
```

No step should be replaced by chat memory, manual GitHub mutation, or silent
local cleanup.

## Prep-Scout Policy

Use prep scouts to remove idle time without creating overlapping write sets.

While one child issue is being executed or closed out:

- a prep scout may inspect the next child issue's cards, source prompt, likely
  touched files, validation lane, and blockers;
- a prep scout must not edit files, create issues, mutate GitHub, or claim the
  issue;
- the prep-scout handoff must name the next issue, current doctor state, likely
  first command, expected validation, and any blockers;
- the executing session still owns the actual `pr run` and issue goal when the
  child becomes active.

This keeps the queue warm without making parallel sessions collide.

## Watcher Policy

Watcher use is mandatory for rescue and release-tail wait states.

Run `adl/tools/pr.sh watch <issue> --json` whenever a child issue is waiting on:

- CI or validation checks;
- human or subagent review;
- mergeability or branch conflicts;
- upstream issue truth;
- closeout after merge or intentional closure;
- operator decision.

The watcher packet must be retained or summarized in the child SOR/SRP/closeout
record. A watcher is advisory and may route to `pr-janitor`, `pr-closeout`, or
operator decision, but it must not mutate GitHub or repo state by itself.

## Scheduler Truth

The Cognitive Scheduler is operational as an advisory planning surface, not as
autonomous runtime authority.

Current truth from the retained runtime proof:

- `adl/src/scheduler.rs` owns the deterministic scheduler economics and plan
  model.
- `adl scheduler plan --input <bundle.json> --json` is the first-class operator
  CLI surface.
- `adl/src/bin/run_v0916_integrated_runtime_soak.rs` calls
  `schedule_economics_bundle` and writes `scheduler/scheduler_plan.json` into
  the Soak #1 artifact root.
- `docs/milestones/v0.91.6/review/runtime/COGNITIVE_SCHEDULER_RUNTIME_ADVISORY_4544.md`
  records the bounded proof and non-claims.

Current non-claims:

- the scheduler does not mutate GitHub, worktrees, providers, PRs, branches, or
  cloud resources;
- the scheduler is not yet the sprint conductor, provider router, subagent
  allocator, or general runtime loop authority;
- future `v0.91.7` runtime soak work may consume scheduler decisions more
  deeply, but this sprint must not overclaim that state.

## Remote Build Lane

`#4587` owns build relief.

Current known state:

- Nessus is the immediate non-wuji remote validation lane.
- A fresh Nessus proof ran the focused `provider_communication` Rust test
  against current `origin/main` in `92` seconds and retained `summary.json` plus
  a bounded log bundle.
- EC2 was also tested through SSM. The attempted `c7i.4xlarge` Spot launch was
  blocked by `MaxSpotInstanceCountExceeded`, so the live proof used one
  short-lived on-demand `c7i.2xlarge` instance.
- The EC2 proof built `adl-pr-doctor` and ran the same focused
  `provider_communication` Rust test through SSM. Retained timing was `575`
  elapsed seconds, including `99` seconds for the binary build and `252`
  seconds for the focused test.
- A follow-up small Spot probe under this `#4588` sprint packet launched a
  `c7i.large` Spot instance, reached SSM `Online`, ran a smoke shell command
  successfully, then terminated the instance and removed the temporary security
  group, instance profile, and role. This proves smaller standard Spot capacity
  can work in the account; it does not prove Rust, `sccache`, or ADL validation
  on Spot yet.
- Temporary EC2/SSM resources were torn down before publication; retained
  teardown evidence reports the instance `terminated` and the temporary
  security group, instance profile, and role absent.
- The retained packets do not prove final AWS billing spend or end-to-end Spot
  build savings. The large Spot request was quota-blocked; the small Spot smoke
  succeeded; the ADL build proof still came from on-demand EC2.
- Routine EC2 use still needs the planned AWS orchestrator / Spot manager with
  quota discovery, instance fallback, SSM polling, log capture, teardown guards,
  cost accounting, and `sccache` bootstrap.

The remote build lane must prefer focused ADL build/validation commands over the
full test surface unless the touched issue actually requires broad validation.

## Independent Binary Lane

`#4590` owns the immediate Cargo-lock and command-startup blocker.

Current known state:

- ADL already declares first-class owner binaries in `adl/Cargo.toml`, including
  `adl`, `adl-csdlc`, `adl-prompt-template`,
  `adl-validate-structured-prompt`, and `adl-pr-*` binaries;
- some workflow scripts already prefer built binaries, but the policy is not
  universal;
- `adl/tools/test_prompt_template_workflow_integration.sh` still hardcodes
  `cargo run` for prompt-template and structured-prompt validation commands;
- `adl/tools/pr_delegate.sh` still contains a silent Cargo fallback path;
- concurrent sessions have observed Cargo package/build lock waits during
  small issue publication and validation.

The rescue standard is now binary-first: normal ADL workflow execution should
resolve independent binaries from explicit environment overrides, the primary
checkout build output, target directories, or `PATH`. Cargo may remain available
for explicit build/setup or an intentionally opted-in fallback, but it must not
be hidden normal control flow for `pr finish`, prompt-template validation,
watcher, or closeout paths.

Current publication status: completed through repo-native lifecycle. PR `#4596`
merged, issue `#4590` closed, and closeout validated STP/SIP/SOR. The retained
watcher packet remains point-in-time evidence from the pre-merge wait state.

## Operating Guidance Lane

`#4591` owns the documentation and skill-guidance drift exposed by the rescue
sprint.

Current known state:

- the actual v0.91.6 workflow now requires watcher-managed wait states,
  prep-scout handoffs, issue-bound goals, session-ledger awareness,
  binary-first command execution, and scheduler truth boundaries;
- those rules are partially visible in the rescue packet and root policy, but
  not yet consistently reflected in canonical workflow docs or the installed
  operational skill docs used by new sessions;
- the Cognitive Scheduler must be presented as advisory CLI/artifact truth for
  this milestone, not as autonomous runtime or sprint-conductor authority;
- the docs and skills must explain how `workflow-conductor`, `sprint-conductor`,
  `issue-watcher`, `pr-janitor`, `pr-ready`, `pr-run`, `pr-finish`, and
  `pr-closeout` fit together so sessions stop rediscovering the path through
  failed PRs.

This issue should update the shared operating surfaces before release-tail work
resumes in parallel.

Current publication status: completed through repo-native lifecycle. PR `#4597`
merged, issue `#4591` closed, and closeout validated STP/SIP/SOR. The retained
watcher packet remains point-in-time evidence from the pre-merge wait state.
The rescue-sprint doc path named by the issue exists in this `#4588` worktree
and becomes root-visible when the `#4588` packet lands.

## Stop Rules

Stop and route instead of improvising when:

- a child issue lacks a session claim;
- cards fail design-time readiness;
- `pr finish` discovers a lane or command-shape mismatch;
- workflow-critical commands fall back to Cargo without explicit opt-in or
  observable policy evidence;
- a session cannot determine the current watcher, prep-scout, scheduler,
  binary-first, or closeout contract from the shared docs/skills;
- SRP/SOR requires manual truth reconstruction that a facts packet should own;
- watcher reports `checks_failed`, `merge_conflict`, `closeout_needed`, or
  `unknown`;
- remote build setup would expose credentials or leave paid resources running;
- scheduler authority is requested beyond the advisory CLI and Soak #1 artifact
  path already proved by `#4544`.

## Release-Tail Resume Gate

The ordered release-tail issue wave may resume only after:

- `#4539` closeout truth is normalized or explicitly waived by the operator;
- `#4584`, `#4585`, and `#4593` have retained watcher and doctor packets that
  reconcile their closed state, final SRP/SOR truth, and any closeout-needed
  watcher residue;
- `#4586`, `#4587`, `#4590`, and `#4591` are merged and closed out;
- any exception to those child completion requirements is recorded as an
  explicit operator waiver in `#4588` with release-tail impact, retained watcher
  evidence, and follow-on owner;
- `#4598` is either executed and closed, or explicitly operator-waived as a
  ready release-tail follow-on with impact and owner recorded;
- `#4588` records a sprint review that names remaining risks;
- watcher and prep-scout policy is active for release-tail execution.

## Non-Claims

This sprint does not claim:

- autonomous scheduler runtime authority;
- product/runtime feature completion;
- EC2 cost approval;
- broad validation reduction for every future issue;
- automatic multi-agent execution without issue-local claims and worktree
  boundaries.
