# Issue-Bound Goal Terminal-State Policy

This document defines when an issue-bound Codex goal may truthfully be marked
`complete` in ADL workflow sessions.

## Purpose

Issue-bound goals exist to make token/time accounting, sprint rollups, and
workflow state truthful. A goal must not be marked complete merely because a
local session produced a patch, opened a draft PR, or reached a convenient
pause point.

## Default Rule

Default tracked implementation goals remain active until the declared terminal
boundary is satisfied.

They must not be marked complete while the issue or PR is:

- red
- pending
- conflicted
- draft
- missing required checks
- missing current SRP/SOR truth

## Allowed Goal Kinds

- `setup_only`
- `implementation`
- `watcher`
- `janitor`
- `review_only`
- `sprint_child`
- `sprint_umbrella`
- `tracked_issue`

## Terminal Boundaries

- `handoff_only`
  Use only when the goal explicitly declares setup-only, review-only, or
  equivalent handoff scope.
- `pr_green`
  Default tracked implementation boundary. Requires an open non-draft PR,
  green or explicitly skipped required checks, and current SRP/SOR truth.
- `merged`
  Use when the goal declares merge as the terminal state.
- `closed_no_pr`
  Use for intentional no-PR closure work.
- `closed_out`
  Use when the goal must carry through issue closure and closeout truth.
- `watch_target_reached`
  Use for watcher/janitor goals that must wait for an observed target state.
- `sprint_rollup_settled`
  Use for sprint umbrella goals that depend on child-issue terminal truth.

## Handoff Rule

Handoff-only completion is never implicit. If a goal is allowed to complete at
publication or review handoff, that narrower terminal boundary must be explicit
in the goal objective or the recorded workflow artifact.

## Recording Rule

When goal completion is captured in durable workflow artifacts, also record:

- goal kind
- declared terminal boundary
- issue/PR state used for evaluation
- review truth status
- closeout truth status when applicable
- whether completion was allowed
- the reason for the decision

## Tooling Hook

Use `adl/tools/check_issue_goal_terminal_state.py` for a fail-closed terminal
state evaluation:

```bash
python3 adl/tools/check_issue_goal_terminal_state.py \
  --goal-kind implementation \
  --pr-state open \
  --checks-state green \
  --review-truth current
```

Goal metrics capture helpers under
`adl/tools/skills/sprint-conductor/scripts/` may also record and enforce this
truth when completion is being written into durable issue or sprint artifacts.
