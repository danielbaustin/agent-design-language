---
name: issue-watcher
description: Watch one issue, PR, branch, or dependency gate for readiness while work is waiting, classify the state, route PR blockers to pr-janitor, and stop before merge, closeout, or implementation.
---

# Issue Watcher

This skill owns bounded wait-window monitoring for one workflow target.

Its job is to:
- inspect one issue, PR, branch, or dependency gate
- classify whether the target is ready, pending, blocked, merged, closed, or action-required
- identify the next lifecycle handoff without performing it silently
- route PR check, conflict, or review blockers to `pr-janitor`
- emit a concise watcher result and stop

This is an observation and routing skill, not a repair or execution skill.

## When To Use It

Use this skill when:
- a prerequisite issue or PR is waiting to merge before the next work item can start
- the operator wants a watcher on a dependency gate during another task
- an issue should be rechecked for readiness without opening the implementation lane
- a PR's high-level state is needed, but detailed CI repair belongs to `pr-janitor`

Do not use this skill for:
- bootstrapping missing issue cards
- implementing the issue
- diagnosing and fixing failed PR checks directly
- merging PRs, closing issues, or running closeout
- long-running daemon or scheduler behavior outside the current invocation

## Required Inputs

At minimum, gather:
- `repo_root`
- one concrete primary target:
  - `target.issue_number`
  - `target.pr_number`
  - `target.pr_url`
  - `target.branch`
  - `target.dependency_issue_number`

Useful additional inputs:
- `target.expected_state`
- `target.expected_checks`
- `target.dependency_notes`
- `policy.monitor_checks`
- `policy.monitor_merge_state`
- `policy.monitor_closure_state`
- `policy.allow_pr_inference`

If no concrete primary target is present, stop and report `blocked`.

## Quick Start

1. Resolve exactly one primary watch target.
2. Inspect the target's live state using GitHub or repo-native metadata.
3. If watching an issue, inspect linked PRs only when inference is allowed and unambiguous.
4. Classify the observed state.
5. Recommend the next handoff.
6. Stop without mutating issue, PR, branch, or implementation state.

## Workflow

### 1. Resolve The Target

Prefer the most concrete selector supplied:
1. explicit PR number
2. explicit PR URL
3. explicit issue number
4. explicit branch
5. explicit dependency issue number

Exactly one primary target should drive the watch. A dependency list may be
used as context, but it must not create multiple independent watch tasks in one
invocation.

If multiple primary targets disagree, report `blocked`.

### 2. Inspect State

Inspect where applicable:
- issue open/closed state
- issue labels, milestone, and dependency notes when relevant
- linked PR state when inference is allowed
- PR draft/open/merged/closed state
- check summary
- mergeability or conflict state
- review state only at the high-level blocker/routing level

Use direct GitHub metadata or `gh` where available. Do not infer readiness only
from stale local cards when live GitHub state is required.

When watching PR checks, apply
`adl/tools/skills/docs/CI_RUNTIME_POLICY_GUIDE.md`. Continue watching stable
check names (`adl-ci` and `adl-coverage`), but classify their meaning through
the CI lane when needed:
- `healthy_with_path_policy_skip`
- `healthy_with_full_runtime_validation`
- `pending_stable_checks`
- `blocked_failed_closed_or_unexpected_skip`

If the path-policy lane is unclear, route to `pr-janitor` rather than claiming
the PR is ready.

### 3. Classify The Result

Use:
- `ready`
  - the watched gate is satisfied and the next work item may proceed
- `pending`
  - the target is healthy but still waiting on checks, review, draft state, or merge
- `blocked`
  - the target is ambiguous, missing, conflicted, or requires human/process action
- `action_required`
  - a concrete next skill or operator action is needed
- `merged`
  - the watched PR has merged
- `closed`
  - the watched issue or PR is closed without being a merge-ready success path

## Routing Rules

- If a PR has failed checks, merge conflicts, or requested changes, route to `pr-janitor`.
- If an issue is structurally unready, route to `pr-ready`.
- If cards are missing, route to `pr-init`.
- If the watched prerequisite is merged and the next issue is structurally ready, route to `pr-run`.
- If the issue or PR outcome is settled and only lifecycle cleanup remains, route to `pr-closeout`.
- If human review or explicit approval is needed, route to the operator.

## Stop Boundary

This skill must not:
- modify source files, cards, issues, PRs, labels, branches, or worktrees
- rerun or repair CI
- merge, close, reopen, or mark PRs ready for review
- create a long-running background loop
- collapse into `pr-janitor`, `pr-run`, or `pr-closeout`

When ADL expects structured output, follow `references/output-contract.md`.
