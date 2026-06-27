# Issue Lifecycle Shepherd Contract

This document defines the first-class issue-lifecycle shepherd contract for ADL.

It does not create a new autonomous super-skill. Instead, it makes the
end-to-end ownership model above `pr-init`, `pr-run`, `pr-finish`,
`issue-watcher`, `pr-janitor`, and `pr-closeout` explicit, reviewable, and
machine-friendly.

## Purpose

The issue-lifecycle shepherd exists so one tracked issue never disappears into:

- chat/session memory
- healthy-but-ownerless PR waiting states
- ambiguous janitor versus watcher routing
- merged-but-unclosed local task bundles

The shepherd contract ties together:

- lifecycle routing
- execution binding
- PR publication handoff
- healthy wait-state monitoring
- blocker remediation routing
- terminal closeout

## Non-Authority Boundary

The shepherd is coordination, not authority.

It must not claim authority to:

- merge a PR
- close an issue
- override human review findings
- bypass editor-skill or workflow-conductor policy
- convert blocked state into success by narration

Human review, merge authority, and final GitHub closure remain explicit
authority surfaces outside the shepherd contract.

## Canonical Lifecycle States

The canonical issue-lifecycle shepherd states are:

1. `pre_run`
   - The issue exists and is being routed or readied.
   - Typical owner: `workflow-conductor` or `pr-ready`.
2. `execution_bound`
   - The issue has a bound branch/worktree and an active issue goal.
   - Typical owner: `pr-run`.
3. `publication_ready`
   - Bounded implementation is complete and waiting for truthful PR publication.
   - Typical owner: `pr-finish`.
4. `pr_waiting`
   - A PR exists and is in a healthy waiting state such as checks running,
     waiting for review, or green-but-unmerged.
   - Typical owner: `issue-watcher`.
5. `janitor_active`
   - A PR exists and has actionable blockers such as failed checks, conflicts,
     stale branch state, or review-requested remediation.
   - Typical owner: `pr-janitor`.
6. `merged_needs_closeout`
   - The PR outcome is settled, but local closeout truth is not yet finalized.
   - Typical owner: `pr-closeout`.
7. `closed_no_pr`
   - The issue settled without PR merge, for example duplicate, superseded, or
     other explicit no-PR closeout.
   - Typical owner: `pr-closeout`.
8. `settled`
   - Local closeout truth is finalized and no active shepherding remains.
9. `blocked`
   - The lifecycle cannot continue truthfully until a concrete blocker is
     resolved, for example missing bootstrap, card-local readiness defects, or
     ambiguous live state.
   - Typical owner: `workflow-conductor` or the currently active lifecycle
     skill surfacing the blocker.

## Canonical Transition Rules

- `pre_run -> execution_bound`
  - requires readiness/bind success and issue-goal creation
- `execution_bound -> publication_ready`
  - requires bounded implementation and truthful local validation
- `publication_ready -> pr_waiting`
  - requires PR publication/update with no active blocker
- `publication_ready -> janitor_active`
  - requires PR publication/update plus an actionable blocker
- `pr_waiting -> janitor_active`
  - triggered by failed checks, conflicts, or actionable review requests
- `pr_waiting -> merged_needs_closeout`
  - triggered by merged PR state
- `janitor_active -> pr_waiting`
  - triggered when the blocker is repaired and the PR returns to a healthy
    waiting state
- `janitor_active -> merged_needs_closeout`
  - triggered when the PR merges after remediation
- `merged_needs_closeout -> settled`
  - requires truthful local closeout completion
- `closed_no_pr -> settled`
  - requires truthful local closeout completion with explicit no-PR rationale
- `any_state -> blocked`
  - triggered when the next truthful lifecycle step is prevented by a concrete
    blocker rather than an ordinary waiting state
- `blocked -> pre_run | execution_bound | publication_ready | pr_waiting | janitor_active | merged_needs_closeout | closed_no_pr | settled`
  - triggered when the blocker is resolved and the lifecycle can resume at the
    appropriate truthful state

## Shared Evidence Requirements

Every lifecycle-shepherd handoff should preserve:

- issue identity
- branch/worktree identity when bound
- current shepherd state
- next expected owner or skill
- whether shepherding remains active
- whether merge/issue close authority remains human-owned
- whether closeout is still required

Acceptable durable evidence surfaces include:

- workflow-conductor routing artifacts
- PR finish result/output artifacts
- issue-watcher results
- PR janitor results
- PR closeout results
- task-bundle records when those records are the canonical issue truth surface

## Wait-State Evidence Rule

Healthy waiting is active lifecycle work. It must leave durable evidence.

When an issue or PR enters `pr_waiting`, `janitor_active`, or
`merged_needs_closeout`, the owning workflow must retain one of:

- a repo-native `pr.sh watch <issue-or-pr> --json` packet;
- a task-bundle, SRP, SOR, or closeout summary that names the retained watcher
  packet path and its disposition; or
- an explicit not-applicable reason when the issue never entered a wait state.

Watcher routing must use the packet's top-level `classification`,
`tail_owner`, and `next_skill` fields as the authoritative lifecycle routing
surface. Nested fields such as `linked_pr.validation.disposition` explain the
check result and may support diagnosis, but they are not the routing key.

Current watcher classifications route as follows:

- `pr_open` or `checks_running` keeps the issue in watcher-owned wait state
  with `next_skill: issue-watcher`.
- `checks_failed`, requested changes, merge-conflict blockers, or other
  actionable blockers route to `pr-janitor`.
- `checks_green_but_draft` routes to `pr-janitor` because the draft-state
  transition is an actionable PR-tail task.
- `checks_green` preserves the `next_skill: human_review` handoff and the
  merge-authority boundary before completion is claimed.
- `merged_pending_closeout` or `closeout_needed` routes to `pr-closeout`.
- `closed` requires explicit no-PR or already-settled closeout rationale.
- `ready_for_run` or `blocked` is pre-publication readiness truth and should
  route to the packet's declared `next_skill`.

Issue closeout is not clean when a known wait state occurred but the final SOR,
SRP, closeout artifact, or sprint execution packet has no watcher packet
reference and no not-applicable reason.

## Shared Output Shape

When a lifecycle or observation surface needs to record shepherd state, use
this common block:

```yaml
lifecycle_shepherd:
  active: true | false
  state: pre_run | execution_bound | publication_ready | pr_waiting | janitor_active | merged_needs_closeout | closed_no_pr | settled | blocked
  owner_skill: workflow-conductor | pr-ready | pr-run | pr-finish | issue-watcher | pr-janitor | pr-closeout | human_review | none
  next_skill: pr-init | pr-ready | pr-run | pr-finish | issue-watcher | pr-janitor | pr-closeout | stp-editor | sip-editor | spp-editor | srp-editor | sor-editor | human_review | none
  closeout_required: true | false
  authority_boundary:
    merge_authority_human_only: true | false
    issue_close_authority_human_only: true | false
    review_authority_human_only: true | false
```

Rules:

- `active` must stay `true` until the issue is truly settled.
- `state: pr_waiting` is healthy active work, not abandonment.
- `state: merged_needs_closeout` must keep `closeout_required: true`.
- `state: settled` requires `active: false`.
- `state: blocked` is for concrete blockers, not healthy waiting.
- authority-boundary fields should stay `true` for normal ADL workflow issue
  work unless another explicit contract proves otherwise.

## Relationship To Existing Skills

- `workflow-conductor` owns routing into the next bounded skill.
- `pr-run` owns bind and bounded implementation execution.
- `pr-finish` owns publication handoff into PR-tail shepherding.
- `issue-watcher` owns healthy wait-state observation.
- `pr-janitor` owns bounded blocker remediation while a PR is in flight.
- `pr-closeout` owns terminal local settlement.

This document does not replace those skills. It defines the shared issue-local
ownership model above them.
