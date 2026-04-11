---
name: pr-closeout
description: Finalize an issue after its implementation path is done by verifying GitHub issue/PR closure state, normalizing final STP/SIP/SOR truth, reconciling root/worktree artifacts, recording supersession or duplicate links when relevant, and pruning the local execution surface. Use when publication/review is over or a no-PR closure disposition is known and the next step is truthful local closeout rather than implementation or PR monitoring.
---

# PR Closeout

This skill owns the post-merge and post-closure cleanup phase of the PR workflow.

Its job is to:
- verify the PR is actually merged, intentionally closed, or not required for the final disposition
- verify the issue is actually closed
- normalize final STP, SIP, and SOR truth
- reconcile root and worktree card/artifact surfaces when they have diverged
- record deferral, supersession, or duplicate links when that is why the issue closed
- confirm no required artifacts remain only in the worktree
- prune the local worktree safely
- emit a structured closeout result
- stop before unrelated archival or repo-wide cleanup

This is a procedural execution skill with local write side effects.

## Current Compatibility Model

Current repo truth:
1. `pr-init` bootstraps the issue
2. `pr-ready` determines structural readiness
3. `pr-run` performs the bounded implementation work
4. `pr-finish` publishes or updates the reviewable PR surface
5. `pr-janitor` monitors the in-flight PR until human review and merge/closure settle
6. `pr-closeout` finalizes the local issue state after that PR outcome is known

The repo control plane may now trigger the same closeout behavior automatically after merge or explicit closed/completed state when the lifecycle evidence is unambiguous.

This skill is intentionally later than `pr-finish`.

## Required Inputs

At minimum, gather:
- repository root
- one concrete closeout target:
  - issue number
  - branch
  - worktree path
  - PR number
- one explicit closure outcome:
  - merged
  - intentionally_closed
  - closed_no_pr
  - superseded
  - duplicate

Useful additional inputs:
- root STP/SIP/SOR paths
- worktree-local STP/SIP/SOR paths
- merged PR URL
- merged commit or final branch state
- worktree prune policy
- local branch deletion policy

## Quick Start

1. Resolve the concrete issue/PR/worktree target.
2. Confirm the PR is actually merged or intentionally closed.
3. Confirm the issue closure state is consistent with that outcome.
4. Record any supersession, duplicate, or deferral references when they explain the closure.
5. Normalize final STP, SIP, and SOR truth.
6. Sync final output-card truth back to the root bundle if needed.
7. Confirm no required artifacts remain only in the worktree.
8. Prune the worktree safely.
9. Emit a structured closeout result and stop.

## Workflow

### 1. Resolve Closeout Target

Identify the target using the most concrete available input.

Prefer this order:
1. explicit issue number
2. explicit PR number
3. explicit worktree path
4. explicit branch

If multiple surfaces disagree materially, report `blocked`.

### 2. Verify Closure State

Before making closeout changes:
- confirm the PR is merged, intentionally closed, or not required for this issue disposition
- confirm the issue is closed
- confirm the branch/worktree maps to the intended issue

Do not treat:
- an open PR
- a merely draft PR
- a still-active branch under review

as closeout-complete state.

For no-PR dispositions such as:
- docs-only work resolved without a PR
- duplicate findings covered elsewhere
- obsolete or superseded issues

the closeout record must explicitly say why no PR merge exists.

### 3. Record Closure References

If the issue closed because it was:
- superseded
- duplicated
- covered elsewhere
- intentionally deferred into another issue

record the relevant issue, PR, or discussion links in the final closeout result and card surfaces that summarize disposition.

### 4. Finalize Card Truth

Normalize the cards to final truthful state:
- STP should reflect completed issue status without rewriting history
- SIP should no longer describe active execution state if the issue is closed out
- SOR should reflect final integration and validation truth

When root and worktree copies differ:
- prefer the final truthful completed record
- sync back to the root bundle when the workflow expects a canonical root copy

When card-local cleanup is needed, this skill may compose with:
- `stp-editor`
- `sip-editor`
- `sor-editor`

### 5. Reconcile Artifacts

Confirm:
- the required tracked artifacts are represented in the repository path the workflow considers canonical
- no required proof surface exists only in the worktree
- root/worktree bundle linkage is still truthful after closeout

If required artifacts remain only in the worktree, stop as `blocked`.

### 6. Prune Local Execution Surface

After final truth and artifact reconciliation:
- prune the issue worktree safely
- verify the worktree registration is gone

Optional policy-controlled cleanup may include:
- deleting the local issue branch after merge

Do not delete local branch state automatically unless the caller/policy explicitly allows it.

### 7. Stop Boundary

This skill must stop after truthful local closeout.

It must not:
- merge the PR
- reopen implementation
- silently clean unrelated worktrees
- perform repo-wide archival chores unrelated to the closed issue

The normal handoff is to:
- human review of the closed issue record
- reporting/index maintenance if explicitly requested

## Output

Return a concise structured result including:
- target issue and PR
- closure outcome
- closure references, if any
- final card/artifact reconciliation status
- worktree prune result
- remaining follow-up, if any
