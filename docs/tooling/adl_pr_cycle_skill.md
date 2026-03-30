---
name: adl_pr_cycle
description: Deterministic Codex.app workflow for the real ADL authoring control plane: pr init, pr start, pr run, and pr finish, with bounded editor-adapter truth and required reporting.
---

# adl_pr_cycle

This tracked file is the canonical source for the local Codex skill installed at:

- `$CODEX_HOME/skills/adl_pr_cycle/SKILL.md`

Install or resync the local skill with:

```bash
bash adl/tools/install_adl_pr_cycle_skill.sh
```

## Skill Prompt

```text
You are running skill: adl_pr_cycle.

Inputs:
- issue_num (required)
- slug (required)
- title (required)
- paths (required, comma-separated tracked repo paths)
- version (optional; when omitted, infer from the issue labels or current milestone band)
- mode (optional: apply|suggest, default apply)
- run_cmd (optional; required only when the issue's proof surface needs a bounded runtime execution or replay step)
- open_pr (optional, default true)
- merge (optional, default false)
- delete_branch (optional, default false)

Hard guardrails:
1) Deterministic state machine only:
   preflight -> issue_ready -> init -> start -> codex -> run_if_required -> finish -> report
2) Never work on main.
3) Use the repo-local execution clone when available:
   .worktrees/adl-wp-<issue_num>
4) Do not edit outside:
   - <paths>
   - .adl/cards
   - .adl/logs
   - .adl/reports
5) Never stage or commit .adl/** files.
6) Retry transient command failures at most 2 times.
7) Always produce a report file even on failure.
8) Browser/editor direct support remains bounded to:
   adl/tools/editor_action.sh start
   Do not imply direct browser/editor invocation of pr init, pr run, or pr finish.

Procedure:
1) Preflight
   - Validate required inputs are non-empty.
   - Compute branch: codex/<issue_num>-<slug>.
   - Compute task_id: issue-<zero-padded issue_num>.
   - Resolve version:
     - use the explicit input when provided
     - otherwise infer from the issue labels/current milestone band
   - Compute:
     - stp_path=.adl/<version>/tasks/<task_id>__<slug>/stp.md
     - input_card=.adl/cards/<issue_num>/input_<issue_num>.md
     - output_card=.adl/cards/<issue_num>/output_<issue_num>.md
     - report_dir=.adl/reports/pr-cycle/<issue_num>/<timestamp_utc_z>/
   - Prefer executing from .worktrees/adl-wp-<issue_num> when that repo-local clone exists and is writable.
2) Init
   - Run:
     bash ./adl/tools/pr.sh init <issue_num> --slug <slug> [--version <version>]
   - Confirm canonical STP exists at <stp_path>.
3) Issue-ready check
   - Confirm the GitHub issue already exists and matches the intended issue_num.
   - Do not invoke `pr create`; issue creation/reconciliation is outside the ADL command surface.
4) Start
   - Run:
     bash ./adl/tools/pr.sh start <issue_num> --slug <slug> [--version <version>]
   - Confirm canonical cards exist:
     .adl/cards/<issue_num>/input_<issue_num>.md
     .adl/cards/<issue_num>/output_<issue_num>.md
5) Codex
   - Read the input card.
   - Execute only within the allowed edit fence.
   - Tee Codex output to .adl/logs/<issue_num>/codex.log when possible.
6) Run (conditional, but use the real command surface when required)
   - If the issue's proof surface requires bounded runtime execution, replay, or emitted run artifacts, run:
     bash ./adl/tools/pr.sh run ...
     using the documented issue-specific arguments or the provided run_cmd.
   - If the issue is docs-only or otherwise does not require runtime execution, state that explicitly in the report and output card instead of inventing a run step.
7) Finish
   - Run:
     bash ./adl/tools/pr.sh finish <issue_num> --title "<title>" --paths "<paths>" -f .adl/cards/<issue_num>/input_<issue_num>.md --output-card .adl/cards/<issue_num>/output_<issue_num>.md
   - If open_pr=false, include --no-open.
   - If merge=true, include --merge only when an open PR already exists or open_pr=true.
8) Report (always)
   - Write:
     .adl/reports/pr-cycle/<issue_num>/<timestamp_utc_z>/report.md
   - Include:
     - Input values
     - Derived branch and task paths
     - Whether repo-local .worktrees/adl-wp-<issue_num> was used
     - Commands attempted (in order)
     - Modified tracked files excluding .adl/**
     - Validation/check results
     - Whether pr run was executed or truthfully skipped
     - PR URL (if available)
     - Exactly one next action command

Truth boundaries:
- The active authoring control-plane exists in repo truth.
- The browser/editor adapter remains narrower than the full control plane.
- Use docs/tooling/editor/command_adapter.md and docs/tooling/editor/five_command_demo.md as the canonical proof surfaces for that boundary.
- Use bash adl/tools/test_five_command_regression_suite.sh as the canonical regression proof surface for the full implemented lifecycle.

Failure policy:
- Fail fast on non-transient errors.
- On failure, still write the report and include one next-action command.
```
