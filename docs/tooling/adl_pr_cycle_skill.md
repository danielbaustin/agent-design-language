# `adl_pr_cycle` Skill Contract

This document mirrors the intended v0.85 contract for the local Codex skill at:

- `$CODEX_HOME/skills/adl_pr_cycle/SKILL.md`

It exists so the skill update remains reviewable in the tracked repo.

## Lifecycle

The skill now models the real control-plane workflow as:

- `preflight`
- `pr init`
- `pr create`
- `pr start`
- `codex`
- `pr run` when the issue's proof surface requires bounded runtime execution
- `pr finish`
- `report`

## Truth Boundaries

- The five-command control-plane exists in repo truth.
- The browser/editor adapter remains narrower than the full control plane.
- In v0.85, browser-direct adapter support remains bounded to:
  - `adl/tools/editor_action.sh start`
- The skill must not imply direct browser/editor invocation of:
  - `pr init`
  - `pr create`
  - `pr run`
  - `pr finish`

## Execution Expectations

- Prefer repo-local execution clones under:
  - `.worktrees/adl-wp-<issue_num>`
- Use canonical cards under:
  - `.adl/cards/<issue_num>/input_<issue_num>.md`
  - `.adl/cards/<issue_num>/output_<issue_num>.md`
- Never stage or commit `.adl/**` files.
- Always emit a report under:
  - `.adl/reports/pr-cycle/<issue_num>/<timestamp_utc_z>/report.md`

## Proof Surface

The skill should stay aligned with:

- `docs/default_workflow.md`
- `docs/tooling/editor/command_adapter.md`
- `docs/tooling/editor/five_command_demo.md`
- `docs/tooling/editor/five_command_regression_suite.md`
- `bash adl/tools/test_five_command_regression_suite.sh`
