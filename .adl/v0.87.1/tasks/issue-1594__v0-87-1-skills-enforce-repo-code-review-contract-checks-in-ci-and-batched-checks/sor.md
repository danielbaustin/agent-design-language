# v0-87-1-skills-enforce-repo-code-review-contract-checks-in-ci-and-batched-checks

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1594
Run ID: issue-1594
Version: v0.87.1
Title: [v0.87.1][skills] Enforce repo-code-review contract checks in CI and batched checks
Branch: codex/1594-v0-87-1-skills-enforce-repo-code-review-contract-checks-in-ci-and-batched-checks
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5 Codex
- Provider: OpenAI
- Start Time: 2026-04-11T18:00:00Z
- End Time: 2026-04-11T18:07:27Z

## Summary
Wired the repo-code-review contract test into both GitHub CI and the local batched-check surface, and updated the tools README so the new guard is visible to operators.

## Artifacts produced
- `.github/workflows/ci.yaml`
- `adl/tools/batched_checks.sh`
- `adl/tools/README.md`

## Actions taken
- Added a dedicated `repo-code-review contract check` step to `adl-ci` before the Rust formatter, clippy, and test phases.
- Added the same contract script to `adl/tools/batched_checks.sh` so local batched validation matches CI coverage.
- Updated the tools README to note that batched checks include the repo-code-review contract guard.
- Opened PR `#1605` with the enforcement wiring and operator-facing note.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths are updated on the issue branch via PR 1605
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: committed on the issue branch, pushed to origin, and opened as PR `#1605`
- Verification performed:
  - `git status --short` to confirm the branch was clean after the publish commit
  - `git ls-files .github/workflows/ci.yaml adl/tools/batched_checks.sh adl/tools/README.md` to verify the tracked proof surface is present on the issue branch
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
  - `bash adl/tools/test_repo_code_review_skill_contracts.sh` to verify the repo-code-review manifest/schema/operator-guide contract remains intact
  - `bash -n adl/tools/batched_checks.sh` to verify the updated local batched-check wrapper remains syntactically valid
  - `git diff --check` to verify no whitespace or patch-format regressions remain
- Results: PASS. All targeted validations completed successfully.

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
      - "bash adl/tools/test_repo_code_review_skill_contracts.sh"
      - "bash -n adl/tools/batched_checks.sh"
      - "git diff --check"
  determinism:
    status: PASS
    replay_verified: true
    ordering_guarantees_verified: not_applicable
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
- Determinism tests executed: reran the repo-code-review contract script after wiring it into CI and batched checks
- Fixtures or scripts used: `adl/tools/test_repo_code_review_skill_contracts.sh`
- Replay verification (same inputs -> same artifacts/order): confirmed for the contract script; the same repository state produced the same PASS result
- Ordering guarantees (sorting / tie-break rules used): not applicable because this issue wires an existing deterministic script into fixed CI and local check order
- Artifact stability notes: the change surface is limited to one CI workflow, one local batch wrapper, and one README note

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual review of touched diffs; no secrets or credential material were introduced
- Prompt / tool argument redaction verified: yes; the change only adds a local contract script invocation and a README note
- Absolute path leakage check: passed via review of the final SOR and touched repo files; only repository-relative paths are recorded here
- Sandbox / policy invariants preserved: yes; the issue only adds deterministic validation steps and does not expand runtime privileges

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable; no ADL runtime trace bundle was produced for this validation-enforcement fix
- Run artifact root: not applicable; validation used repository-local scripts only
- Replay command used for verification: `bash adl/tools/test_repo_code_review_skill_contracts.sh`
- Replay result: PASS

## Artifact Verification
- Primary proof surface: PR `#1605` plus the tracked files `.github/workflows/ci.yaml`, `adl/tools/batched_checks.sh`, and `adl/tools/README.md`
- Required artifacts present: yes; all issue-specific tracked artifacts are present on the pushed branch
- Artifact schema/version checks: satisfied indirectly by the contract script added to both CI and batched checks
- Hash/byte-stability checks: not run; issue scope is bounded to deterministic validation wiring and documentation
- Missing/optional artifacts and rationale: no additional artifacts were required beyond the tracked CI, tooling, and documentation surfaces

## Decisions / Deviations
- Added a small README note even though the issue could have been closed without it, because operator discoverability is part of making the new guard usable.

## Follow-ups / Deferred work
- The repo-code-review contract script still contains an absolute `reference_doc` assertion and may merit a later portability cleanup, but that was intentionally left out of this enforcement-only issue.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
