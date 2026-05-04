# v0-90-5-tools-stop-pr-finish-tests-from-leaking-orphaned-post-merge-closeout-watchers

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-2699
Run ID: issue-2699
Version: v0.90.5
Title: [v0.90.5][tools] Stop PR-finish tests from leaking orphaned post-merge closeout watchers
Branch: retrospective-no-branch
Status: DONE

Execution:
- Actor: issue-wave bootstrap
- Model: not_applicable
- Provider: not_applicable
- Start Time: 2026-05-04T03:52:20Z
- End Time: 2026-05-04T03:52:20Z

## Summary

This bootstrap-only duplicate was closed without execution after the malformed issue body was replaced by canonical issue `#2700`.

## Artifacts produced
- Local ignored output-card scaffold at `.adl/v0.90.5/tasks/issue-2699__v0-90-5-tools-stop-pr-finish-tests-from-leaking-orphaned-post-merge-closeout-watchers/sor.md`
- Tracked implementation artifacts: not_applicable until execution begins

## Actions taken
- Opened the local issue bundle during the first bootstrap attempt.
- Determined the authored issue body was shell-mangled and unsuitable as the canonical tracker.
- Closed duplicate issue `#2699` and superseded it with canonical issue `#2700`.
- Stopped before any branch binding, worktree creation, implementation, or PR publication for this duplicate.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none
- Worktree-only paths remaining: none
- Integration state: closed_no_pr
- Verification scope: main_repo
- Integration method used: local ignored record updated to reflect duplicate closure and supersession by `#2700`
- Verification performed:
  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase bootstrap --input .adl/v0.90.5/tasks/issue-2699__v0-90-5-tools-stop-pr-finish-tests-from-leaking-orphaned-post-merge-closeout-watchers/sor.md`
    Verified bootstrap SOR contract compliance for the local pre-run scaffold.
  - `gh issue close 2699 --reason 'not planned' --comment 'Closing as a malformed duplicate of #2700. The first create attempt was shell-mangled and stripped the key repo inputs and command names from the authored body. Please use #2700 as the canonical tracked issue for this watcher-leak fix.'`
    Verified the duplicate issue was closed and redirected to the canonical tracker.
- Result: PASS

Rules:
- Final artifacts must exist in the main repository, not only in a worktree.
- Do not leave docs, code, or generated artifacts only under a `adl-wp-*` worktree.
- Prefer git-aware transfer into the main repo (`git checkout <branch> -- <path>` or commit + cherry-pick).
- If artifacts exist only in the worktree, the task is NOT complete.
- `Integration state` describes lifecycle state of the integrated artifact set, not where verification happened.
- `Verification scope` describes where the verification commands were run.
- `worktree_only` means at least one required path still exists only outside the main repository path.
- `pr_open` should pair with truthful `Worktree-only paths remaining` content; list those paths when they still exist only in the worktree or say `none` only when the branch contents are fully represented in the main repository path.
- If `Integration state` is `pr_open`, verify the actual proof artifacts rather than only the containing directory or card path.
- If `Integration method used` is `direct write in main repo`, `Verification scope` should normally be `main_repo` unless the deviation is explained.
- If `Verification scope` and `Integration method used` differ in a non-obvious way, explain the difference in one line.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By `pr finish`, `Status` should normally be `DONE` (or `FAILED` if the run failed and the record is documenting that failure).

## Validation
- Validation commands and their purpose:
  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase bootstrap --input .adl/v0.90.5/tasks/issue-2699__v0-90-5-tools-stop-pr-finish-tests-from-leaking-orphaned-post-merge-closeout-watchers/sor.md`
    Verified bootstrap SOR contract compliance for the local output scaffold.
- Results:
  - PASS

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

Rules:
- Replace the example values below with one actual final value per field.
- Do not leave pipe-delimited enum menus or placeholder text in a finished record.

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "bash adl/tools/validate_structured_prompt.sh --type sor --phase bootstrap --input .adl/v0.90.5/tasks/issue-2699__v0-90-5-tools-stop-pr-finish-tests-from-leaking-orphaned-post-merge-closeout-watchers/sor.md"
  determinism:
    status: NOT_RUN
    replay_verified: unknown
    ordering_guarantees_verified: unknown
  security_privacy:
    status: PARTIAL
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: PASS
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: not_run; bootstrap scaffold creation has not been replay-verified for this issue yet.
- Fixtures or scripts used: `adl/tools/pr.sh` issue-wave opening flow.
- Replay verification (same inputs -> same artifacts/order): not yet verified for this specific issue record.
- Ordering guarantees (sorting / tie-break rules used): not_applicable for a single-card bootstrap write.
- Artifact stability notes: repository-relative paths only; execution-time proof artifacts are not expected yet.

## Security / Privacy Checks
- Secret leakage scan performed: limited content review only; no secrets were intentionally recorded in the scaffold.
- Prompt / tool argument redaction verified: not_applicable for bootstrap scaffold generation.
- Absolute path leakage check: repository-relative paths only in the scaffold.
- Sandbox / policy invariants preserved: yes; local ignored issue-record path only.

## Replay Artifacts
- Trace bundle path(s): not_applicable until execution begins
- Run artifact root: not_applicable until execution begins
- Replay command used for verification: not_run
- Replay result: NOT_RUN

## Artifact Verification
- Primary proof surface: this local pre-run SOR scaffold and its bootstrap validation result
- Required artifacts present: local output card scaffold only; tracked implementation artifacts are not expected yet
- Artifact schema/version checks: bootstrap SOR validator passed
- Hash/byte-stability checks: not_run
- Missing/optional artifacts and rationale: execution proofs, demos, and tracked outputs are intentionally absent before implementation begins

## Decisions / Deviations
- Closed this issue as `closed_no_pr` because it was superseded by canonical issue `#2700` before any branch, worktree, or PR was created.
- Preserved the duplicate-closeout trail in the local record instead of deleting the malformed bootstrap residue entirely.

## Follow-ups / Deferred work
- No further work in this duplicate bundle; execute and publish the real fix under `#2700`.
