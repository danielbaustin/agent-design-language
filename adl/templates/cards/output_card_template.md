# <issue-slug>

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID:
Run ID:
Version:
Title:
Branch:
Status: IN_PROGRESS

Execution:
- Actor:
- Model:
- Provider:
- Start Time:
- End Time:

## Summary

## Artifacts produced
- 

## Actions taken
- 

## Main Repo Integration (REQUIRED)
- Main-repo paths updated:
- Worktree-only paths remaining: none | list explicitly
- Integration state: worktree_only | pr_open | merged
- Verification scope: worktree | pr_branch | main_repo
- Integration method used:
- Verification performed:
  - `git status`
  - `ls <path>` / equivalent path check
- Result:

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
- Results:

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
    status: PASS | FAIL | PARTIAL | NOT_RUN
    checks_run:
      - ""
  determinism:
    status: PASS | FAIL | PARTIAL | NOT_RUN
    replay_verified: true | false | unknown
    ordering_guarantees_verified: true | false | unknown
  security_privacy:
    status: PASS | FAIL | PARTIAL | NOT_RUN
    secrets_leakage_detected: true | false | unknown
    prompt_or_tool_arg_leakage_detected: true | false | unknown
    absolute_path_leakage_detected: true | false | unknown
  artifacts:
    status: PASS | FAIL | PARTIAL | NOT_RUN
    required_artifacts_present: true | false | unknown
    schema_changes:
      present: true | false | unknown
      approved: true | false | not_applicable | unknown
```

## Determinism Evidence
- Determinism tests executed:
- Fixtures or scripts used:
- Replay verification (same inputs -> same artifacts/order):
- Ordering guarantees (sorting / tie-break rules used):
- Artifact stability notes:

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed:
- Prompt / tool argument redaction verified:
- Absolute path leakage check:
- Sandbox / policy invariants preserved:

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s):
- Run artifact root:
- Replay command used for verification:
- Replay result:

## Artifact Verification
- Primary proof surface:
- Required artifacts present:
- Artifact schema/version checks:
- Hash/byte-stability checks:
- Missing/optional artifacts and rationale:

## Decisions / Deviations

## Follow-ups / Deferred work

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
