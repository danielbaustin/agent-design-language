---
name: pr-finish
description: Perform truthful closeout for an executed issue by validating the output record, staging the intended paths, creating or updating the draft PR, and stopping before silent merge. Use when bounded issue execution is complete and the next step is reviewable PR publication or update.
---

# PR Finish

This skill owns the closeout and publication phase of the PR workflow.

Its job is to:
- confirm the issue has completed its bounded execution work
- validate the output record and finish inputs truthfully
- stage only the intended tracked paths
- create or update the reviewable PR surface
- emit a structured finish result
- stop before silent merge or issue closure unless explicitly directed

This is a procedural execution skill with write side effects.

## Current Compatibility Model

Current repo truth:
1. `pr-init` bootstraps the issue
2. `pr-ready` determines structural readiness
3. `pr-run` performs the bounded implementation work
4. `pr-finish` performs truthful closeout and PR publication/update
5. `pr-janitor` monitors the in-flight PR after publication

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
   - `adl pr finish`
4. Validate the declared staged paths and PR metadata.
5. Publish or update the draft PR surface.
6. Emit a structured finish result and stop.

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
- confirm the output record exists and is not still a bootstrap stub
- confirm the intended staged paths are explicit or deterministically derived
- confirm validation claims in the output record are truthful

### 3. Run Finish Through The Repo Control Plane

Prefer repo-native finish commands rather than manual git/PR surgery.

This skill may:
- stage the intended tracked paths
- validate finish/body linkage
- create or update the reviewable draft PR

This skill must not:
- silently widen scope
- silently merge
- silently close the issue

### 4. Stop Boundary

The normal handoff is to:
- `pr-janitor`
- human review
- explicit merge/closeout direction

## Output

Return a concise structured result including:
- target issue
- branch/worktree used
- output-record status
- staged paths
- validation performed
- PR publication/update result
- recommended next step
