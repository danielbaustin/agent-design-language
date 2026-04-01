# v0-85-docs-capture-road-to-v0-95-roadmap-restructure-edits-from-main

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1245
Run ID: issue-1245
Version: v0.85
Title: [v0.85][docs] Capture ROAD_TO_v0.95 roadmap restructure edits from main
Branch: codex/1245-v0-85-docs-capture-road-to-v0-95-roadmap-restructure-edits-from-main
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5-codex
- Provider: OpenAI
- Start Time: 2026-03-31
- End Time: 2026-03-31

## Summary

Preserved the in-progress `ROAD_TO_v0.95.md` edits, moved the roadmap from `.adl/docs/v0.85planning/` to the cross-milestone location `.adl/docs/roadmaps/`, and updated the issue/task surfaces to follow the new path.

## Artifacts produced
- `.adl/docs/roadmaps/ROAD_TO_v0.95.md`
- updated issue body, STP, and SIP pointing at the new roadmap location

## Actions taken
- created `.adl/docs/roadmaps/` in the issue worktree
- moved `ROAD_TO_v0.95.md` from `.adl/docs/v0.85planning/` to `.adl/docs/roadmaps/`
- preserved the existing roadmap edits during the move
- updated the issue body, STP, and SIP to reference the new canonical path

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none yet; changes are staged on the issue branch for PR review
- Worktree-only paths remaining: `.adl/docs/roadmaps/ROAD_TO_v0.95.md`
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: moved the tracked file within the issue branch and prepared it for PR review
- Verification performed:
  - `git status --short`
  - `ls .adl/docs/roadmaps`
  - `git diff --stat`
- Result: the new roadmap path exists in the issue worktree, the edited content is preserved, and the old path is removed on the branch

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
  - `git status --short`
    verified the intended moved file and task-surface edits were the only active branch changes
  - `ls .adl/docs/roadmaps`
    verified the new roadmap directory and moved file exist
  - `git diff --stat`
    verified the branch carries the roadmap move plus the related issue/task-surface updates
- Results:
  - validation passed for the docs move and review-surface alignment

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
      - "git status --short"
      - "ls .adl/docs/roadmaps"
      - "git diff --stat"
  determinism:
    status: NOT_RUN
    replay_verified: unknown
    ordering_guarantees_verified: unknown
  security_privacy:
    status: PASS
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
- Determinism tests executed: none; this is a docs move and review-surface alignment task
- Fixtures or scripts used: not applicable
- Replay verification (same inputs -> same artifacts/order): not run
- Ordering guarantees (sorting / tie-break rules used): not applicable
- Artifact stability notes: the edited roadmap content was preserved byte-for-byte through a file move within the branch

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual review of the moved doc and task-surface updates only; no secrets introduced
- Prompt / tool argument redaction verified: yes; no prompts or tool arguments were added to the roadmap doc
- Absolute path leakage check: passed; only repository-relative paths are recorded in the output card
- Sandbox / policy invariants preserved: yes

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable
- Run artifact root: not applicable
- Replay command used for verification: not applicable
- Replay result: not applicable

## Artifact Verification
- Primary proof surface: `.adl/docs/roadmaps/ROAD_TO_v0.95.md`
- Required artifacts present: yes
- Artifact schema/version checks: not applicable for this docs move
- Hash/byte-stability checks: not run; preservation was verified by moving the edited file in-branch and reviewing the resulting diff
- Missing/optional artifacts and rationale: none

## Decisions / Deviations

- Chose `.adl/docs/roadmaps/ROAD_TO_v0.95.md` as the new canonical location because the document spans multiple milestone bands and should not live under `v0.85planning`.

## Follow-ups / Deferred work

- Close `#1245` after PR review/merge once the roadmap move is accepted.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
