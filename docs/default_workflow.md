# Default Workflow (adl_pr_cycle + pr.sh)

This is the default contributor path for ADL development:

`preflight -> init -> create -> start -> codex -> run_if_required -> finish -> report`

Tracked mirror of the local skill contract:

- `docs/tooling/adl_pr_cycle_skill.md`

Install or resync the local skill with:

```bash
bash adl/tools/install_adl_pr_cycle_skill.sh
```

The five-command control-plane surface is:

- `pr init`
- `pr create`
- `pr start`
- `pr run`
- `pr finish`

The browser/editor adapter remains narrower:

- browser-direct adapter support exists only for `adl/tools/editor_action.sh start`
- direct browser/editor execution of `pr init`, `pr create`, `pr run`, and `pr finish` is not part of the v0.85 adapter surface

## 1) Initialize Canonical STP

```bash
bash ./adl/tools/pr.sh init <issue_num> --slug <slug> --version v0.85
```

Canonical local task bundle:
- `.adl/<scope>/tasks/<task-id>__<slug>/stp.md`
- `.adl/<scope>/tasks/<task-id>__<slug>/`

Minimum v0.85 init contract:
- canonical task-bundle directory
- validated `stp.md`
- no implied SIP/SOR creation yet

## 2) Reconcile GitHub Issue From Canonical STP

```bash
bash ./adl/tools/pr.sh create <issue_num> --stp .adl/v0.85/tasks/<task-id>__<slug>/stp.md
```

## 3) Start Issue Branch + Local Cards

```bash
bash ./adl/tools/pr.sh start <issue_num> --slug <slug>
```

Compatibility card paths:
- `.adl/cards/<issue_num>/input_<issue_num>.md`
- `.adl/cards/<issue_num>/output_<issue_num>.md`

Preferred execution clone:
- `.worktrees/adl-wp-<issue_num>`

Structured Card Templates v2 (required sections):
- Input card:
  - `System Invariants (must remain true)`
  - `Reviewer Checklist (machine-readable hints)`
  - `Card Automation Hooks (prompt generation)`
- Output card:
  - `Determinism Evidence`
  - `Security / Privacy Checks`
  - `Replay Artifacts`
  - `Artifact Verification`

These sections are designed to support deterministic replay/security verification and
machine-parsable prompt automation.

## 4) Implement

Read the input card, stay inside the issue edit fence, and make the tracked repo changes.

## 5) Run (when the issue requires a bounded runtime proof surface)

```bash
bash ./adl/tools/pr.sh run <adl-file> [run arguments...]
```

Use `pr run` when the issue's proof surface requires emitted run artifacts, replay, or bounded runtime execution.
For docs-only or non-runtime issues, skip `pr run` truthfully and record that in the output card/report rather than inventing a hidden step.

## 6) Validate

Typical local preflight:

```bash
./adl/tools/batched_checks.sh
```

Canonical regression proof surface for the implemented five-command story:

```bash
bash adl/tools/test_five_command_regression_suite.sh
```

Bounded lifecycle proof/demo:

- `docs/tooling/editor/five_command_demo.md`

## 7) Finish PR

```bash
bash ./adl/tools/pr.sh finish <issue_num> \
  --title "<title>" \
  --paths "<comma-separated paths>" \
  -f .adl/cards/<issue_num>/input_<issue_num>.md \
  --output-card .adl/cards/<issue_num>/output_<issue_num>.md
```

## 8) Report

Write a per-issue report under:

- `.adl/reports/pr-cycle/<issue_num>/<timestamp_utc_z>/report.md`

## Common Pitfalls and Remediations

- Dirty repo-local execution clone:
  - Commit/stash first, then re-run the relevant command from `.worktrees/adl-wp-<issue_num>`.
- Wrong paths at `finish`:
  - Ensure `--paths` only includes intended repo paths; do not include local `.adl` artifacts.
- Missing canonical STP:
  - Re-run `pr.sh init <issue_num> --slug <slug> --version v0.85`.
- Stale GitHub issue body:
  - Re-run `pr.sh create <issue_num> --stp .adl/v0.85/tasks/<task-id>__<slug>/stp.md`.
- Missing card files:
  - Re-run `pr.sh start <issue_num> --slug <slug>` to seed canonical card paths.
- Browser/editor overclaims:
  - Use `docs/tooling/editor/command_adapter.md` as the truth boundary; only `start` is browser-direct in v0.85.
- Worktree branch base problems:
  - Update from `origin/main`, then re-run `start` in the repo-local execution clone.
