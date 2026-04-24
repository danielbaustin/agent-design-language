# Default Workflow (adl_pr_cycle + pr.sh)

This is the default contributor path for ADL development:

`issue creation/bootstrap -> pr ready -> pr run -> codex -> run_if_required -> pr finish -> report`

Tracked mirror of the local skill contract:

- `docs/tooling/adl_pr_cycle_skill.md`

Install or resync the local skill with:

```bash
bash adl/tools/install_adl_pr_cycle_skill.sh
```

The active control-plane surface is:

- `pr init`
- `pr ready`
- `pr run`
- `pr finish`

The browser/editor adapter remains narrower:

- browser-direct adapter support remains narrower than the full repo-native control plane
- direct browser/editor execution of `pr ready`, `pr run`, and `pr finish` is not the canonical workflow surface

## 1) Initialize Canonical STP

```bash
bash ./adl/tools/pr.sh init <issue_num> --slug <slug> --version v0.87
```

Canonical local task bundle:
- `.adl/<scope>/tasks/<task-id>__<slug>/stp.md`
- `.adl/<scope>/tasks/<task-id>__<slug>/`

Minimum init contract:
- canonical task-bundle directory
- validated `stp.md`
- validated root `sip.md`
- validated root `sor.md`

## 2) Confirm GitHub Issue Exists

```bash
gh issue view <issue_num>
```

`pr.sh` no longer creates or reconciles GitHub issues. The issue must already exist before kickoff continues.

## 3) Confirm Readiness And Bind Run Phase

```bash
bash ./adl/tools/pr.sh ready <issue_num> --slug <slug> --version v0.87
bash ./adl/tools/pr.sh run <issue_num> --slug <slug> --version v0.87
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

Canonical regression proof surface for the implemented editing story:

```bash
bash adl/tools/test_five_command_regression_suite.sh
```

Bounded lifecycle proof/demo:

- `docs/tooling/editor/five_command_demo.md`

### Compression-Safe Validation

The v0.90 milestone compression pilot distinguishes execution compression from
validation compression.

Low-risk docs/static-tooling issues may use the
`FOCUSED_LOCAL_CI_GATED` profile in
`docs/milestones/v0.90/milestone_compression/FINISH_VALIDATION_PROFILES_v0.90.md`
when focused local checks directly prove the changed surface. The output record
must say that full local validation was not run, list the focused commands that
did run, and keep CI required before merge.

Use full local validation for runtime, schema, security, release, broad tooling,
or ambiguous changes.

For broad non-coverage Rust lanes, prefer `cargo nextest run` over raw
`cargo test` when the lane is executing the whole runnable test graph rather
than a narrow filtered proof. Keep `cargo llvm-cov` on its own coverage lanes,
and preserve doc-test signal explicitly with `cargo test --doc` where needed.

### Issue-Class Validation Rule

Before choosing validation, classify the issue using the narrowest truthful
changed-surface class.

Recommended classes:

- `docs-only`
- `milestone-package-truth`
- `workflow-docs`
- `tooling-focused`
- `rust-focused`
- `demo-focused`
- `review-remediation`
- `release-tail`

Default rule:

- docs-heavy classes do not require the full Rust test cycle
- bounded tooling classes use focused shell or unit checks
- runtime or schema classes use targeted Rust validation and widen only when the
  changed surface demands it
- release-tail classes rely on tracker, gap, closeout, review-truth, path, and
  guardrail checks unless tracked code changed

Always record:

- the chosen issue class
- the selected validation profile
- the exact commands run
- what was intentionally not run

Do not use full local validation as a reflex. Use it because the changed
surface needs it.

## 7) Finish PR

```bash
bash ./adl/tools/pr.sh finish <issue_num> \
  --title "<title>" \
  --paths "<comma-separated paths>" \
  -f .adl/cards/<issue_num>/input_<issue_num>.md \
  --output-card .adl/cards/<issue_num>/output_<issue_num>.md
```

Finish should only open or update the PR after the SOR is finalized, and the finalized output record should be synced to the canonical task bundle under:

- `.adl/<scope>/tasks/issue-<padded_issue>__<slug>/sor.md`

## 8) Report

Write a per-issue report under:

- `.adl/reports/pr-cycle/<issue_num>/<timestamp_utc_z>/report.md`

## Common Pitfalls and Remediations

- Dirty repo-local execution clone:
  - Commit/stash first, then re-run the relevant command from `.worktrees/adl-wp-<issue_num>`.
- Wrong paths at `finish`:
  - Ensure `--paths` only includes intended repo paths; do not include local `.adl` artifacts.
- Missing canonical STP:
  - Re-run `pr.sh init <issue_num> --slug <slug> --version v0.87`.
- Stale GitHub issue body:
  - Reconcile the GitHub issue outside `pr.sh`, then re-run `pr.sh init <issue_num>` if the local root bundle must be reseeded.
- Missing card files:
  - Re-run `pr.sh init <issue_num>` to reseed root bundle surfaces, then `pr.sh run <issue_num>` to bind the execution worktree if the issue is entering run phase.
- Browser/editor overclaims:
  - Use `docs/tooling/editor/command_adapter.md` as the truth boundary; do not treat browser/editor entrypoints as the canonical repo-native execution path.
- Worktree branch base problems:
  - Update from `origin/main`, then re-run `run` in the repo-local execution clone.
