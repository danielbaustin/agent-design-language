# v0-90-5-records-repair-missing-duplicate-task-bundle-records

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-2704
Run ID: issue-2704
Version: v0.90.5
Title: [v0.90.5][records] Repair missing/duplicate task-bundle records: #2683 and #2699 duplicate residue
Branch: codex/2704-v0-90-5-records-repair-missing-duplicate-task-bundle-records
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5-codex
- Provider: openai
- Start Time: 2026-05-04T15:57:10Z
- End Time: 2026-05-04T15:57:10Z

## Summary

Completed the bounded `#2704` records-surgery pass. The issue prompt and root cards were tightened to the real `#2683`/`#2699` repair scope, the closed local records for `#2683` and `#2699` were normalized through editor-style cleanup, and the v0.90.5 closeout/residue guards were rerun cleanly.

## Artifacts produced
- Updated local issue prompt at `.adl/v0.90.5/bodies/issue-2704-v0-90-5-records-repair-missing-duplicate-task-bundle-records.md`
- Updated local task bundle at `.adl/v0.90.5/tasks/issue-2704__v0-90-5-records-repair-missing-duplicate-task-bundle-records`
- Updated local continuity records for `#2683` and `#2699`

## Actions taken
- Routed `#2704` through `workflow-conductor` from the issue surface and again from the bound task-bundle surface.
- Normalized the `#2704` source prompt and STP from bootstrap stub wording to the concrete records-repair scope.
- Normalized the run-bound `#2704` SIP to reflect the bound branch/worktree and records-only validation plan.
- Normalized `#2683` SIP/SOR truth so the closed issue points at merged PR `#2691` instead of reading like an unstarted scaffold.
- Normalized `#2699` SOR duplicate-closeout residue, including the completed-phase branch field required for `closed_no_pr` validation.
- Reran the bounded local records validation surfaces after the editor passes.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated:
  - `.adl/v0.90.5/bodies/issue-2704-v0-90-5-records-repair-missing-duplicate-task-bundle-records.md`
  - `.adl/v0.90.5/tasks/issue-2704__v0-90-5-records-repair-missing-duplicate-task-bundle-records/stp.md`
  - `.adl/v0.90.5/tasks/issue-2704__v0-90-5-records-repair-missing-duplicate-task-bundle-records/sip.md`
  - `.adl/v0.90.5/tasks/issue-2704__v0-90-5-records-repair-missing-duplicate-task-bundle-records/sor.md`
  - `.adl/v0.90.5/tasks/issue-2683__v0-90-5-daily-coverage-blockers-2026-05-02/stp.md`
  - `.adl/v0.90.5/tasks/issue-2683__v0-90-5-daily-coverage-blockers-2026-05-02/sip.md`
  - `.adl/v0.90.5/tasks/issue-2683__v0-90-5-daily-coverage-blockers-2026-05-02/sor.md`
  - `.adl/v0.90.5/tasks/issue-2699__v0-90-5-tools-stop-pr-finish-tests-from-leaking-orphaned-post-merge-closeout-watchers/sor.md`
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: main_repo
- Integration method used: issue-worktree records normalization followed by bounded local guard reruns; publication has not happened yet
 - Verification performed:
  - `gh issue view 2683 --json number,state,closedAt,title`
    Verified `#2683` is closed.
  - `gh pr list --state all --head codex/2683-v0-90-5-daily-coverage-blockers-2026-05-02 --json number,state,title,headRefName,url`
    Verified merged PR `#2691` is the publication surface for `#2683`.
  - `gh issue view 2699 --comments --json number,state,closedAt,comments`
    Verified `#2699` is closed with duplicate-redirection evidence to `#2700`.
  - `bash adl/tools/check_milestone_closed_issue_sor_truth.sh --version v0.90.5`
    Verified the closed-issue SOR truth guard passes for v0.90.5 after the records cleanup.
  - `bash adl/tools/check_no_tracked_adl_issue_record_residue.sh`
    Verified there is no remaining tracked ADL issue-record residue.
  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase completed --input .adl/v0.90.5/tasks/issue-2683__v0-90-5-daily-coverage-blockers-2026-05-02/sor.md`
    Verified the normalized completed SOR for `#2683`.
  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase completed --input .adl/v0.90.5/tasks/issue-2699__v0-90-5-tools-stop-pr-finish-tests-from-leaking-orphaned-post-merge-closeout-watchers/sor.md`
    Verified the normalized completed SOR for `#2699`.
  - `gh pr view 2709 --json number,state,isDraft,url,headRefName,baseRefName`
    Verified draft PR `#2709` is open on the `codex/2704-v0-90-5-records-repair-missing-duplicate-task-bundle-records` branch.
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
- Validation profile: local records-surgery verification
- Validation commands and their purpose:
  - `gh issue view 2683 --json number,state,closedAt,title`
    Verified `#2683` is closed.
  - `gh pr list --state all --head codex/2683-v0-90-5-daily-coverage-blockers-2026-05-02 --json number,state,title,headRefName,url`
    Verified merged PR `#2691` published the recorded `#2683` branch.
  - `gh issue view 2699 --comments --json number,state,closedAt,comments`
    Verified `#2699` remains closed as a duplicate redirected to `#2700`.
  - `bash adl/tools/check_milestone_closed_issue_sor_truth.sh --version v0.90.5`
    Verified the milestone closeout-truth guard passes after the repair.
  - `bash adl/tools/check_no_tracked_adl_issue_record_residue.sh`
    Verified the ADL issue-record residue guard remains clean.
  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase completed --input .adl/v0.90.5/tasks/issue-2683__v0-90-5-daily-coverage-blockers-2026-05-02/sor.md`
    Verified `#2683`'s completed SOR.
  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase completed --input .adl/v0.90.5/tasks/issue-2699__v0-90-5-tools-stop-pr-finish-tests-from-leaking-orphaned-post-merge-closeout-watchers/sor.md`
    Verified `#2699`'s completed SOR.
  - `gh pr view 2709 --json number,state,isDraft,url,headRefName,baseRefName`
    Verified the draft PR publication surface for `#2704`.
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
      - "gh issue view 2683 --json number,state,closedAt,title"
      - "gh pr list --state all --head codex/2683-v0-90-5-daily-coverage-blockers-2026-05-02 --json number,state,title,headRefName,url"
      - "gh issue view 2699 --comments --json number,state,closedAt,comments"
      - "bash adl/tools/check_milestone_closed_issue_sor_truth.sh --version v0.90.5"
      - "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh"
      - "bash adl/tools/validate_structured_prompt.sh --type sor --phase completed --input .adl/v0.90.5/tasks/issue-2683__v0-90-5-daily-coverage-blockers-2026-05-02/sor.md"
      - "bash adl/tools/validate_structured_prompt.sh --type sor --phase completed --input .adl/v0.90.5/tasks/issue-2699__v0-90-5-tools-stop-pr-finish-tests-from-leaking-orphaned-post-merge-closeout-watchers/sor.md"
      - "gh pr view 2709 --json number,state,isDraft,url,headRefName,baseRefName"
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
- Determinism tests executed: not_run; this pass normalized local records against fixed GitHub and guard surfaces only.
- Fixtures or scripts used: none
- Replay verification (same inputs -> same artifacts/order): not_run
- Ordering guarantees (sorting / tie-break rules used): not_applicable
- Artifact stability notes: repository-relative paths only; the edited records surfaces are stable local text artifacts.

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
- Primary proof surface:
  - `.adl/v0.90.5/bodies/issue-2704-v0-90-5-records-repair-missing-duplicate-task-bundle-records.md`
  - `.adl/v0.90.5/tasks/issue-2704__v0-90-5-records-repair-missing-duplicate-task-bundle-records`
  - `.adl/v0.90.5/tasks/issue-2683__v0-90-5-daily-coverage-blockers-2026-05-02`
  - `.adl/v0.90.5/tasks/issue-2699__v0-90-5-tools-stop-pr-finish-tests-from-leaking-orphaned-post-merge-closeout-watchers/sor.md`
- Required artifacts present: yes for the records-surgery scope
- Artifact schema/version checks: completed SOR validation passed for the closed issue records; root issue records are prepared for publication
- Hash/byte-stability checks: not_run
- Missing/optional artifacts and rationale: no runtime or demo artifacts were required because this issue only repaired local records truth

## Decisions / Deviations
- Treated the original `#2683` "missing bundle" report as stale evidence because the canonical local bundle already existed by the time this issue executed.
- Published no runtime or behavior changes; the issue is bounded strictly to local records truth.

## Follow-ups / Deferred work
- Monitor draft PR `#2709` through the normal janitor/review path.
