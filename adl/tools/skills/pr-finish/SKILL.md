---
name: pr-finish
description: Perform truthful closeout for an executed issue by validating the output record, staging the intended paths, creating or updating the draft PR, and stopping before silent merge. Use when bounded issue execution is complete and the next step is reviewable PR publication or update.
---

# PR Finish

This skill owns the closeout and publication phase of the PR workflow.

Its job is to:
- confirm the issue has completed its bounded execution work
- validate and, when needed, normalize the output record before PR publication
- stage only the intended tracked paths
- ensure the finalized SOR is normalized on the canonical local-only `.adl` task-bundle surface without treating it as tracked publication payload
- create or update the reviewable PR surface
- hand off the active issue session into explicit PR shepherding rather than
  treating draft publication as the natural stopping point for issue work
- emit a structured finish result
- stop before silent merge or issue closure unless explicitly directed

When finish is blocked by output-card truth drift, this skill may invoke `sor-editor` for bounded SOR normalization before retrying finish.

This is a procedural execution skill with write side effects.

## Current Compatibility Model

Current repo truth:
1. `pr-init` bootstraps the issue
2. `pr-ready` determines structural readiness
3. `pr-run` performs the bounded implementation work
4. `pr-finish` performs truthful closeout and PR publication/update
5. `pr-janitor` is auto-attached after publication through the repo hook and monitors the in-flight PR
6. `pr-closeout` finalizes the local issue state after merge or intentional closure

## Required Inputs

At minimum, gather:
- repository root
- one concrete finish target:
  - issue number
  - branch
  - worktree path
- finish title
- staged path policy or explicit paths
- output card path

Useful additional inputs:
- input card path
- PR mode (`draft`, `update_only`, `ready`)
- validation mode
- open/merge policy

## Quick Start

1. Resolve the concrete issue/branch target.
2. Confirm the execution output record is present and truthful.
3. Prefer repo-native finish commands:
   - `adl/tools/pr.sh finish`
4. Validate the declared staged paths and PR metadata.
5. Publish or update the draft PR surface.
6. Record the janitor/shepherding handoff for the active issue session:
   after publication, the issue remains active until PR outcome and closeout
   truth settle.
7. Emit a structured finish result and stop.

## Workflow

### 1. Resolve Finish Target

Identify the target using the most concrete available input.

Prefer this order:
1. explicit issue number
2. explicit worktree path
3. explicit branch

If multiple surfaces disagree materially, report `blocked`.

### 2. Validate Finish Preconditions

Before closeout:
- confirm the issue work actually exists on the branch/worktree
- confirm finish is running from the bound issue worktree when the issue branch is checked out in a worktree
- confirm the output record exists and is not still a bootstrap stub
- confirm the output record is finalized before any PR create/update action
- confirm the intended staged paths are explicit or deterministically derived
- confirm validation claims in the output record are truthful

When PR checks are used as validation evidence, apply
`adl/tools/skills/docs/CI_RUNTIME_POLICY_GUIDE.md`:
- `adl-ci` and `adl-coverage` are stable check names, not proof that every
  expensive Rust phase ran
- if coverage was skipped by path policy, record
  `Coverage: skipped by path policy` plus the classifier `reason`
- if full coverage ran, record the coverage artifact or gate evidence, such as
  `coverage-summary.json`
- do not claim full coverage from a green `adl-coverage` check unless the
  coverage-required lane actually ran
- do not cite docs-only PR-level coverage skips as release coverage evidence

## Observability Expectations

- When finish or publication claims depend on logging/observability behavior,
  record the exact proof surface instead of implying the contract from a green
  PR alone.
- If a machine-readable command path was part of the change, preserve whether
  the issue proved stdout-only payload safety, stderr observability, and any
  compatibility-log redirection behavior.
- Treat `adl_event` lines during finish as current workflow evidence, but do
  not claim machine-readable cleanliness unless the issue explicitly proved it.

### 3. Run Finish Through The Repo Control Plane

Prefer repo-native finish commands rather than manual git/PR surgery.

This skill may:
- stage the intended tracked paths
- validate finish/body linkage
- create or update the reviewable draft PR
- record that the next active phase is PR shepherding through `pr-janitor`
  unless an explicit blocker changes the handoff

This skill must not:
- publish from the primary checkout or another checkout when the issue branch is bound elsewhere
- silently widen scope
- silently merge
- silently close the issue

Use the normal no-closing publication path when the issue-local execution is
done but the parent lifecycle surface must remain open, for example sprint
umbrellas, review-hold closeout packets, or milestone truth updates with routed
follow-ons. In the repo-native wrapper this is the `--no-close` finish mode.

If the repo control plane reports `mismatched_publication_surface`, stop and
rerun finish from the bound issue worktree. If it reports
`rebind_to_issue_worktree_required`, re-establish the issue worktree through the
repo-native lifecycle before attempting publication.

### 4. Stop Boundary

The normal handoff is to:
- `pr-janitor`
- `pr-closeout` after merge or intentional closure is known
- human review
- explicit merge/closeout direction

Healthy waiting rule:
- a green or review-waiting PR after finish is still active issue work in the
  shepherding tail, not an abandoned or naturally complete stop state
- when the bounded session objective includes publication plus shepherding,
  do not treat draft creation alone as sufficient completion truth

## Output

Return a concise structured result including:
- target issue
- branch/worktree used
- output-record status
- staged paths
- validation performed
- PR publication/update result
- recommended next step
