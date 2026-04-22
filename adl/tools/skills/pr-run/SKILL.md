---
name: pr-run
description: Execute a prepared issue after doctor review. Use when an issue is structurally ready, should bind or confirm its execution branch and worktree, then perform the bounded implementation work, validations, and truthful output recording without silently finishing or janitoring the PR.
---

# PR Run

This skill owns the execution phase of the PR workflow.

Its job is to:
- confirm the issue has already passed doctor-style readiness review
- bind or confirm the issue's execution branch and worktree
- perform the bounded implementation work for the issue
- run the smallest truthful validation set for the changed surface
- update the execution record/output card truthfully
- stop before PR monitoring, janitoring, merge, or closeout

When bounded card cleanup is needed, this skill may compose with:
- `stp-editor` for STP drift that blocks execution understanding
- `sip-editor` for truthful run-bound SIP normalization
- `sor-editor` for truthful in-flight output-card updates

This is an execution skill. It is allowed to write code, docs, tests, and related issue-scoped artifacts when the issue requires them.

## Design Basis

This skill should track the repository's canonical PR tooling docs.

At the moment, the canonical repo docs are:
- `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`
- `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`

Within this skill bundle, the operational details live in:
- `references/run-playbook.md`
- `references/output-contract.md`

If those docs move, prefer the moved tracked canonical copies over stale path references. Do not silently invent a new execution model from memory when the repo docs have changed.

## Current Compatibility Model

The intended workflow model treats `run` as the execution-time binder and implementation step.

Current repo truth:
1. issue creation/bootstrap is handled earlier by `pr-init`
2. doctor review happens before execution
3. `pr run` is the branch/worktree binding and implementation surface
4. later truthful closeout/publication belongs to `pr-finish`
5. in-flight PR monitoring and blocker response belong to `pr-janitor`

This skill should not bootstrap a brand-new issue from scratch. It may resolve an existing prepared issue, bind the execution context, and execute it.

Important lifecycle rule:
- branch and worktree creation are intentionally deferred until just before execution
- do not create or expect a bound issue worktree earlier in the lifecycle unless the repo is still carrying compatibility state
- late binding is preferred because it reduces unnecessary rebasing and branch drift across prepared-but-not-started issues
- the primary checkout may be used to invoke the repo-native issue-mode run/bind command only while it is tracked-clean on main
- after binding, all tracked implementation edits happen in the issue worktree, not in the primary checkout
- ignored local `.adl` planning or review notes may remain local-only, but that exception must not widen into tracked repo edits on main

## Entry Conditions

Run this skill when all of the following are true:
- there is a concrete prepared issue to execute
- doctor-style readiness has already been checked or can be checked immediately before binding
- the task should continue through bounded implementation and validation

Concrete targets may include:
- an issue number
- a task-bundle path
- a branch/worktree target that maps unambiguously to an issue

Do not use this skill for:
- initial issue bootstrap when the root bundle does not exist
- qualitative card review as a standalone step
- PR janitoring after a draft PR is already in flight
- silent merge or closeout

## Required Inputs

At minimum, gather:
- repository root
- one concrete execution target:
  - issue number
  - task-bundle path
  - branch
  - worktree path

Useful additional inputs:
- slug
- version
- source_prompt_path
- stp_path
- sip_path
- sor_path
- doctor_result or explicit doctor status
- validation policy
- branch binding policy
- worktree policy

If there is no concrete target, stop and report `blocked`.

## Quick Start

1. Resolve the concrete issue target.
2. Confirm doctor status before implementation, using doctor JSON first.
3. Use compatibility readiness/preflight aliases only if the canonical doctor surface is unavailable.
4. Bind or confirm the issue branch and worktree using repo-native `run` behavior.
5. Verify that the worktree-local STP, SIP, and SOR execution bundle now exists.
6. Read the source prompt, STP, SIP, and current output card.
7. Perform only the bounded work required for the issue.
8. Run the smallest truthful validation set.
9. Update the output card or execution record truthfully.
10. Stop before janitor/closeout.

## Workflow

### 1. Resolve Execution Target

Identify the target issue/task context using the most concrete available input.

Prefer this order:
1. explicit issue number plus slug/version if provided
2. task-bundle path
3. worktree path
4. branch

If multiple surfaces disagree materially on issue identity, report `blocked`.

### 2. Confirm Doctor / Readiness State

Before implementation:
- prefer an explicit prior doctor result when available
- otherwise run the repo-native doctor JSON surface first to confirm the issue is structurally ready
- fall back to compatibility readiness/preflight checks only when the canonical doctor surface is unavailable

Execution-readiness and scheduling/preflight should be distinguished the same way `pr-ready` distinguishes them.

Default rule:
- if execution readiness is `blocked`, stop
- if execution readiness is `ready` or `ready_with_repairs`, execution may proceed
- if preflight is currently blocked, follow the caller's policy:
  - default ADL/Codex behavior is to stop and report `blocked_now`
  - if the caller explicitly wants execution despite the gate, record that the run proceeded under override

### 3. Bind Branch And Worktree

Use the repo's canonical `run` surface to create or confirm the issue execution branch and worktree at the last responsible moment.

Preferred behavior:
- if the issue does not yet have a bound branch/worktree, create it now as part of execution
- if the issue already has the correct bound branch/worktree because of compatibility or prior execution, reuse it
- if binding is needed, use repo-native run commands rather than manual git surgery
- keep branch/worktree naming traceable to the issue id and slug
- if the primary checkout has tracked changes while on main, stop with `unsafe_root_checkout_execution` and move the work into the issue worktree before continuing
- if an issue is already bound, run implementation reads/writes from the bound worktree path reported by doctor/conductor evidence

This skill may create or confirm:
- the issue execution branch
- the issue worktree
- the worktree-local execution bundle through the repo-native run path

It must not invent a different branch/worktree naming scheme than the repo's standard control plane.

It should treat a missing branch/worktree before execution as normal, not as readiness failure, when the issue has already passed doctor-style structural checks.

After binding succeeds, the worktree-local execution surfaces must exist:
- `stp.md`
- `sip.md`
- `sor.md`

If those execution surfaces are missing after the repo-native run/bind step:
- try the repo-native materialization path once if it is safe and deterministic
- otherwise stop as `blocked` or `failed`
- do not continue into implementation while pretending the execution bundle is complete

### 4. Execute The Issue

Read the relevant issue surfaces:
- source issue prompt
- root STP
- root SIP
- worktree-local STP/SIP/SOR if execution has already been bound

Then perform only the work required by the issue:
- code changes
- docs changes
- tests
- templates or validation artifacts

Boundaries:
- stay within the issue's required outcome and acceptance criteria
- do not silently absorb adjacent work packages
- if the issue exposes a missing follow-on, record it rather than widening scope

### 5. Validate Truthfully

Validation must match the changed surface.

Prefer:
- formatter checks
- focused tests
- targeted integration checks
- issue-specific proof commands

Do not run an oversized validation suite unless the changed surface truly requires it or the user asks for it.

#### CI Runtime Policy

Use `adl/tools/skills/docs/CI_RUNTIME_POLICY_GUIDE.md` when selecting and
recording the validation lane.

For docs, planning, and non-runtime tooling changes:
- run focused docs, tooling, contract, path, and guardrail checks
- record the lane as a docs-only or tooling-only path-policy validation surface
- do not imply Rust coverage ran

For runtime, source, test, or demo-affecting changes:
- expect Rust fmt, clippy, tests, demo smoke when required, and coverage gates
- treat a skipped `adl-coverage` lane as a blocker unless the path-policy
  `reason` proves the PR is truly non-runtime

For ambiguous changed-path classification:
- fail closed to full validation or record an explicit operator deferral
- do not use uncertainty as a validation waiver

When recording execution truth, distinguish `docs_only_path_policy_skip`,
`runtime_full_validation`, `failed_closed_full_validation`, and
`release_or_main_full_validation`.

### 6. Update The Execution Record

Update the output card or execution record truthfully:
- summary
- artifacts produced
- actions taken
- main repo integration status
- validation
- determinism/security notes where relevant
- follow-ups or deferred work

When the primary work needed here is bounded output-card normalization rather than broader implementation, prefer `sor-editor` over ad hoc card surgery.

Do not claim:
- `DONE` if the branch does not actually reflect the completed issue state
- `main_repo` integration if the work only exists on the issue branch/worktree
- passing validation that was not run

### 7. Stop Boundary

This skill must stop after bounded implementation and truthful execution recording.

It must not:
- silently merge the PR
- silently close the issue
- continue into PR monitoring/janitoring
- expand into unrelated repo-wide cleanup

The normal handoff is to:
- `pr-finish`
- `pr-janitor` after finish/publication when a PR is in flight
- human review

## Parallelism

This skill can run in parallel across distinct issues when write sets do not overlap.

Safe parallel examples:
- executing issue A and issue B in separate branches/worktrees
- one agent executing while another reviews a different issue

Unsafe parallel examples:
- two agents executing the same issue
- overlapping execution and janitoring on the same branch

## Preferred Commands

Canonical machine surface:
- `adl/tools/pr.sh doctor --json`
- `adl pr doctor --json`

Execution surface:
- `adl/tools/pr.sh run`
- `adl pr run`

Compatibility aliases:
- `adl/tools/pr.sh ready`
- `adl/tools/pr.sh preflight`
- `adl pr ready`
- `adl pr preflight`

Use the repo's existing templates, validators, and path logic. Prefer the repository control plane over manual git branching when possible.

## Output

Return status in a concise structured shape.

When writing an artifact for ADL, use the contract in `references/output-contract.md`.

Default result should make these explicit:
- target issue
- branch/worktree used
- execution readiness basis
- whether branch/worktree were created or reused
- artifacts produced
- validations run
- output card path
- recommended next step

## Failure Modes

Common failure modes:
- issue not actually ready for execution
- target identity drift across source prompt/STP/SIP/worktree
- branch/worktree binding mismatch
- implementation scope widening beyond the issue
- output card left stale or contradictory

If the target cannot be determined confidently, report `blocked`.

## Boundaries

This skill may:
- inspect repo state
- run doctor/readiness checks
- create or confirm the issue execution branch/worktree
- edit issue-scoped code, docs, tests, and execution artifacts
- run bounded validation commands
- update truthful execution records

This skill must not:
- bootstrap a missing issue from scratch
- silently override a structurally blocked doctor result
- silently merge/close
- widen into unrelated repo refactors

## ADL Compatibility

This skill is Codex-compatible through frontmatter discovery.

For stricter ADL execution, also use:
- `adl-skill.yaml`
- `references/run-playbook.md`
- `references/output-contract.md`

## Resources

- Playbook: `references/run-playbook.md`
- Output contract: `references/output-contract.md`
- PR tooling feature doc: `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`
- PR tooling architecture doc: `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`
