# Default Workflow (adl_pr_cycle + pr.sh)

This is the default contributor path for ADL development:

`preflight -> start -> codex -> finish -> report`

## 1) Start Issue Branch + Local Task Bundle

```bash
./adl/tools/pr.sh start <issue_num> --slug <slug>
```

Canonical local task bundle:
- `.adl/<scope>/tasks/<task-id>__<slug>/stp.stub.md`
- `.adl/<scope>/tasks/<task-id>__<slug>/stp.md`
- `.adl/<scope>/tasks/<task-id>__<slug>/sip.md`
- `.adl/<scope>/tasks/<task-id>__<slug>/sor.md`

Current workflow creates compatibility links under `.adl/cards/<issue_num>/` for adjacent tools that still consume the legacy path shape.
Current workflow compatibility paths:
- `.adl/cards/<issue_num>/input_<issue_num>.md`
- `.adl/cards/<issue_num>/output_<issue_num>.md`

Canonical local prompt bundle:
- `.adl/v0.85/tasks/<task-id>__<slug>/stp.stub.md`
- `.adl/v0.85/tasks/<task-id>__<slug>/stp.md`
- `.adl/v0.85/tasks/<task-id>__<slug>/sip.md`
- `.adl/v0.85/tasks/<task-id>__<slug>/sor.md`

Until `pr.sh` is migrated fully, `adl/tools/sync_task_bundle_prompts.sh` refreshes the canonical local task-bundle view from the current compatibility paths.

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

## 2) Implement and Validate

Typical local preflight:

```bash
./adl/tools/batched_checks.sh
```

## 3) Finish PR

```bash
./adl/tools/pr.sh finish <issue_num> \
  --title "<title>" \
  --paths "<comma-separated paths>" \
  -f .adl/v0.85/tasks/<task-id>__<slug>/sip.md \
  --output .adl/v0.85/tasks/<task-id>__<slug>/sor.md
```

## Common Pitfalls and Remediations

- Dirty worktree at `start`:
  - Commit/stash first, then re-run `pr.sh start`.
- Wrong paths at `finish`:
  - Ensure `--paths` only includes intended repo paths; do not include local `.adl` artifacts.
- Missing local task-bundle artifacts:
  - Re-run `pr.sh start <issue_num> --slug <slug>` to seed the canonical local task bundle and compatibility links.
  - Ensure `--paths` only includes intended repo paths; do not include `.adl/cards`.
- Missing card files:
  - Re-run `pr.sh start <issue_num> --slug <slug>` to seed canonical card paths.
- Missing local task bundle:
  - Run `adl/tools/sync_task_bundle_prompts.sh --scope v0.85` to rebuild `.adl/v0.85/tasks/` from the current compatibility paths.
- Missing local task-bundle artifacts:
  - Re-run `pr.sh start <issue_num> --slug <slug>` to seed the canonical local task bundle and compatibility links.
- Worktree branch base problems:
  - Update from `origin/main`, then re-run `start`.
