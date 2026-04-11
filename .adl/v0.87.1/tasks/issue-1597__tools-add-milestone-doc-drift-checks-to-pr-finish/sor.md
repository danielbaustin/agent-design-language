# tools-add-milestone-doc-drift-checks-to-pr-finish

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1597
Run ID: issue-1597
Version: v0.87.1
Title: [v0.87.1][tools] Add milestone-doc drift checks to pr finish
Branch: codex/1597-tools-add-milestone-doc-drift-checks-to-pr-finish
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5
- Provider: openai
- Start Time: 2026-04-11T17:20:00Z
- End Time: 2026-04-11T17:52:47Z

## Summary
Added a bounded finish-time milestone-doc drift guard so `pr finish` blocks publication when the active milestone package is structurally stale, missing linked docs, or still carrying placeholder/template drift.

## Artifacts produced
- `adl/src/cli/pr_cmd.rs`
- `adl/src/cli/pr_cmd_validate.rs`

## Actions taken
- Added `validate_milestone_doc_drift_for_finish(...)` to inspect changed milestone-doc branches only when they touch the active `docs/milestones/<scope>/` package.
- Enforced canonical milestone README / feature-doc linkage checks plus placeholder/template rejection for changed milestone markdown surfaces.
- Added validator-level tests for a coherent package pass case and a missing feature-doc failure case.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `.adl/v0.87.1/tasks/issue-1597__tools-add-milestone-doc-drift-checks-to-pr-finish/sor.md`
- Worktree-only paths remaining: none
- Integration state: merged
- Verification scope: main_repo
- Integration method used: normalized the canonical root SOR directly on `main` after verifying the issue is already closed and linked to merged PR `#1602`
- Verification performed:
  - `gh issue view 1597 --json title,url,state,stateReason,closedByPullRequestsReferences`
    - verified the issue is closed and captured the final closure metadata used for this normalization pass
  - `gh pr view 1602 --json state,url`
    - verified the linked closing PR remains available as the final publication surface
  - `ls .adl/v0.87.1/tasks/issue-1597__tools-add-milestone-doc-drift-checks-to-pr-finish/sor.md`
    - verified the canonical root SOR path exists on the main repository path
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `cargo test --manifest-path adl/Cargo.toml milestone_doc_drift -- --nocapture`
    - verified the bounded milestone-doc drift validator accepts a coherent package and rejects a broken linked feature-doc reference.
  - `git diff --check`
    - verified there are no whitespace or malformed patch artifacts in the final branch diff.
- Results:
  - all listed commands passed

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
      - "cargo test --manifest-path adl/Cargo.toml milestone_doc_drift -- --nocapture"
      - "git diff --check"
  determinism:
    status: PASS
    replay_verified: true
    ordering_guarantees_verified: true
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
- Determinism tests executed: validator unit tests for missing-link rejection and coherent-package acceptance.
- Fixtures or scripts used: deterministic temporary milestone packages assembled inside the Rust validator tests.
- Replay verification (same inputs -> same artifacts/order): yes; the same coherent/broken fixture inputs produce the same pass/fail validator result.
- Ordering guarantees (sorting / tie-break rules used): finish computes changed paths first, then runs the milestone-doc guard before any PR create/edit action.
- Artifact stability notes: the guard is intentionally structural and version-local rather than a broader semantic review system.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual inspection of the validator inputs and tests for secret-free path-only checks.
- Prompt / tool argument redaction verified: yes; the validator reads tracked milestone docs and changed paths only.
- Absolute path leakage check: output record uses repository-relative paths only.
- Sandbox / policy invariants preserved: yes; the guard blocks publication before network PR actions when drift is detected.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable for this tooling issue.
- Run artifact root: not applicable.
- Replay command used for verification: `cargo test --manifest-path adl/Cargo.toml milestone_doc_drift -- --nocapture`
- Replay result: PASS.

## Artifact Verification
- Primary proof surface: `adl/src/cli/pr_cmd_validate.rs` and the validator test cases.
- Required artifacts present: true
- Artifact schema/version checks: not applicable beyond the bounded milestone-package integrity checks themselves.
- Hash/byte-stability checks: not performed; deterministic validator tests are the proving surface here.
- Missing/optional artifacts and rationale: no demo artifact or runtime trace bundle is required for this finish-time validation issue.

## Decisions / Deviations
- Kept the guard structural and limited to milestone-doc PRs instead of broadening into general repo-review automation.

## Follow-ups / Deferred work
- If later milestones need broader doc-semantic review, that should land as a separate bounded tool rather than widening this finish-time guard.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
