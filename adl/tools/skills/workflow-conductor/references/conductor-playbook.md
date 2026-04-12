# Workflow Conductor Playbook

## Purpose

This playbook defines the lightweight routing behavior for `workflow-conductor`.

The conductor should:
- inspect the current issue/workflow state
- choose the next appropriate ADL skill
- apply skill/editor/subagent policy
- write one bounded routing artifact
- stop after routing and compliance recording

It should not perform the selected skill's underlying work.

## Routing Order

Prefer the strongest available state evidence in this order:

1. explicit doctor JSON result
2. concrete task bundle paths and card state
3. explicit branch/worktree state
4. explicit PR state
5. bounded issue metadata
6. explicit observed operator state such as subagent assignment

## Preferred Skill Selection

- missing bootstrap/root bundle -> `pr-init`
- STP-only card defect -> `stp-editor`
- SIP-only card defect -> `sip-editor`
- SOR-only card defect -> `sor-editor`
- issue structurally pre-run -> `pr-ready`
- issue ready for execution or binding -> `pr-run`
- execution done, publication needed -> `pr-finish`
- PR in flight with checks/conflicts/review blockers -> `pr-janitor`
- PR open and healthy -> no janitor; hand off to review/wait state
- merged or intentionally closed issue/PR -> `pr-closeout`

## Resume Rule

If early workflow steps are already complete, do not restart them.

Examples:
- if bootstrap exists and doctor is the next truthful step, do not route back to `pr-init`
- if cards are clean and execution is already bound, do not route back to `pr-ready`
- if the PR exists and is failing CI, do not route to `pr-finish`; route to `pr-janitor`

## Editor Rule

If the blocker is card-local and the matching editor skill exists, route to the matching editor skill instead of allowing ad hoc card edits.

## Policy Rule

If policy requires:
- skills
- editor skills
- subagents

the conductor should record compliance or explicit blocker-driven bypass.

Never silently downgrade a required policy to an optional one.
If required policy fails and no explicit bypass is allowed, return `blocked`.

## Deterministic Helper

The bundle may use a small deterministic route-selection helper to evaluate
synthetic or real state snapshots. That helper must:
- only select the next skill
- never perform the selected skill's underlying work
- remain bounded to routing/compliance facts

The bundle may also use a route-only collection entrypoint to derive those
state snapshots from a real issue, task bundle, branch, worktree, or PR.
That collector must remain read-mostly, except for writing the declared routing
artifact.
