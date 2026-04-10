# v0-87-1-tools-prevent-local-git-refresh-helpers-from-checking-out-main-in-managed-worktree-flows

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1528
Run ID: issue-1528
Version: v0.87.1
Title: [v0.87.1][tools] Prevent local git refresh helpers from checking out main in managed worktree flows
Branch: codex/1528-v0-87-1-tools-prevent-local-git-refresh-helpers-from-checking-out-main-in-managed-worktree-flows
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5.4
- Provider: ChatGPT
- Start Time: 2026-04-09T18:24:00-07:00
- End Time: 2026-04-09T18:40:00-07:00

## Summary

Fixed the tracked PR execution path that switched the primary checkout to `main` after creating an issue worktree. The run/start flow now creates branches from `origin/main`, creates or reuses the issue worktree, and leaves the user's primary checkout on its current branch.

## Artifacts produced
- Updated PR lifecycle implementation in `adl/src/cli/pr_cmd.rs`.
- Updated shell compatibility help in `adl/tools/pr.sh`.
- Updated focused tests in `adl/src/cli/tests/pr_cmd_inline/repo_helpers.rs`, `adl/tools/test_pr_cards_primary_root.sh`, and `adl/tools/test_pr_start_worktree_safe.sh`.

## Actions taken
- Removed the Rust `ensure_primary_checkout_on_main` call and helper so `pr run/start` no longer switches the primary checkout to `main`.
- Updated linked-worktree card test setup to create the fixture branch from `origin/main`, not local `main`.
- Aligned the linked-worktree card test with current worktree-local `.adl` artifact isolation.
- Updated the worktree safety script to seed authored issue prompts and assert that dirty/non-main primary checkout state remains untouched.
- Updated start command help text to state that the primary checkout remains on its current branch.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths from this issue are present on main via merged PR #1531.
- Worktree-only paths remaining: none for required tracked artifacts; issue branch changes have merged to main via PR #1531.
- Integration state: merged
- Verification scope: worktree
- Integration method used: issue branch/worktree changes were published and merged via PR #1531.
- Verification performed:
  - `git status --short --branch`
  - focused validation commands listed below
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
  - `cargo fmt --manifest-path adl/Cargo.toml --check`: verified Rust formatting remained stable.
  - `bash -n adl/tools/pr.sh adl/tools/test_pr_start_worktree_safe.sh adl/tools/test_pr_cards_primary_root.sh`: verified edited shell scripts parse.
  - `bash adl/tools/test_pr_cards_primary_root.sh`: verified linked-worktree card artifacts are isolated in the worktree and the fixture uses `origin/main`.
  - `bash adl/tools/test_pr_start_worktree_safe.sh`: verified issue worktree creation/reuse leaves the primary checkout on its current branch, including a dirty non-main branch.
  - `cargo test --manifest-path adl/Cargo.toml pr_cmd -- --nocapture`: verified PR lifecycle Rust coverage after removing the primary-checkout switch.
- Results: all listed validation commands passed.

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
      - "cargo fmt --manifest-path adl/Cargo.toml --check"
      - "bash -n adl/tools/pr.sh adl/tools/test_pr_start_worktree_safe.sh adl/tools/test_pr_cards_primary_root.sh"
      - "bash adl/tools/test_pr_cards_primary_root.sh"
      - "bash adl/tools/test_pr_start_worktree_safe.sh"
      - "cargo test --manifest-path adl/Cargo.toml pr_cmd -- --nocapture"
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
- Determinism tests executed: focused shell lifecycle tests and Rust `pr_cmd` tests.
- Fixtures or scripts used: temporary git repositories created by the shell tests and Rust tests.
- Replay verification (same inputs -> same artifacts/order): the shell tests recreate clean temporary repositories and verify stable worktree, branch, and artifact paths.
- Ordering guarantees (sorting / tie-break rules used): not changed by this issue.
- Artifact stability notes: generated task bundle paths remain derived from issue number, version, and slug.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: no secrets were added; changed files are code/tests/help text only.
- Prompt / tool argument redaction verified: no prompt or tool argument logging behavior was added.
- Absolute path leakage check: output record uses repository-relative paths except this local execution card path, which remains ignored workflow state.
- Sandbox / policy invariants preserved: all tracked edits were made in the #1528 issue worktree, not on `main`.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): none; no runtime trace was required.
- Run artifact root: not applicable for this tooling fix.
- Replay command used for verification: `bash adl/tools/test_pr_start_worktree_safe.sh` and `cargo test --manifest-path adl/Cargo.toml pr_cmd -- --nocapture`.
- Replay result: passed.

## Artifact Verification
- Primary proof surface: PR diff plus focused validation commands.
- Required artifacts present: yes, all tracked changed files are present in the issue branch.
- Artifact schema/version checks: no schema changes.
- Hash/byte-stability checks: not applicable; no binary or generated release artifact was produced.
- Missing/optional artifacts and rationale: no demo trace is required for this workflow safety fix.

## Decisions / Deviations

- The local ignored `adl/.local/fix-git.sh` was hardened separately in the primary checkout because it is not tracked and therefore cannot be carried by this PR.
- The actual tracked root cause was broader than the local helper: `pr run/start` itself switched the primary checkout to `main`; that behavior was removed.

## Follow-ups / Deferred work

- No follow-up is required for the tracked root-cause fix.
- If the team wants `adl/.local/fix-git.sh` to be durable for everyone, promote it into tracked tooling in a separate issue instead of keeping it under ignored `.local`.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
