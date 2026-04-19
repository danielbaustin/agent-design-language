# Issue Watcher Skill Input Schema

Schema id: `issue_watcher.v1`

## Purpose

Provide one structured invocation shape for the bounded `issue-watcher` skill.

The skill observes one issue, PR, branch, or dependency gate during a wait window
and reports whether the watched target is:

- `ready`
- `pending`
- `blocked`
- `action_required`
- `merged`
- `closed`

It does not mutate issues, PRs, branches, cards, or implementation files.

## Supported Modes

- `watch_issue`
- `watch_pr`
- `watch_pr_url`
- `watch_branch`
- `watch_dependency_gate`

## Top-Level Shape

```yaml
skill_input_schema: issue_watcher.v1
mode: watch_issue | watch_pr | watch_pr_url | watch_branch | watch_dependency_gate
repo_root: /absolute/path
target:
  issue_number: <u32 or null>
  pr_number: <u32 or null>
  pr_url: <url or null>
  branch: <string or null>
  dependency_issue_number: <u32 or null>
  expected_state: <ready | merged | closed | any | null>
  expected_checks:
    - <check name>
  dependency_notes:
    - <string>
policy:
  allow_pr_inference: true | false
  monitor_checks: true | false
  monitor_merge_state: true | false
  monitor_closure_state: true | false
  route_blockers: true
  stop_after_watch: true
```

## Mode Requirements

### `watch_issue`

Requires:

- `target.issue_number`

Use when:

- the issue itself is the watched gate
- linked PR inference is allowed and should be used only when unambiguous

### `watch_pr`

Requires:

- `target.pr_number`

Use when:

- a PR number is the canonical watched target

### `watch_pr_url`

Requires:

- `target.pr_url`

Use when:

- the operator supplied the PR URL rather than a number

### `watch_branch`

Requires:

- `target.branch`

Use when:

- the branch is known and may map to one in-flight PR

### `watch_dependency_gate`

Requires:

- `target.dependency_issue_number`

Use when:

- one issue or PR must finish before another issue may start

## Policy Fields

- `allow_pr_inference`
  - required
  - whether the skill may infer a linked PR from an issue or branch
- `monitor_checks`
  - required
  - whether check status should be summarized
- `monitor_merge_state`
  - required
  - whether mergeability/conflict status should be summarized
- `monitor_closure_state`
  - required
  - whether issue/PR closure state should be inspected
- `route_blockers`
  - must be `true`
  - the skill should recommend the next skill/operator action rather than repairing
- `stop_after_watch`
  - must be `true`
  - the skill must stop after one observation pass

## Example Invocation

```yaml
Use $issue-watcher at /Users/daniel/git/agent-design-language/adl/tools/skills/issue-watcher/SKILL.md with this validated input:

skill_input_schema: issue_watcher.v1
mode: watch_pr
repo_root: /Users/daniel/git/agent-design-language
target:
  issue_number: 2144
  pr_number: 2180
  pr_url: null
  branch: null
  dependency_issue_number: null
  expected_state: merged
  expected_checks:
    - adl-ci
    - adl-coverage
  dependency_notes:
    - WP-05 may start after WP-04 merges.
policy:
  allow_pr_inference: false
  monitor_checks: true
  monitor_merge_state: true
  monitor_closure_state: true
  route_blockers: true
  stop_after_watch: true
```

## Notes

- Watch exactly one primary target per invocation.
- Use dependency notes only as context; do not convert them into multiple watch targets.
- Route failed checks, conflicts, and requested review changes to `pr-janitor`.
- Route missing or structurally unready lifecycle surfaces to `pr-ready` or `pr-init`.
- Stop before mutating issue, PR, branch, worktree, or implementation state.

