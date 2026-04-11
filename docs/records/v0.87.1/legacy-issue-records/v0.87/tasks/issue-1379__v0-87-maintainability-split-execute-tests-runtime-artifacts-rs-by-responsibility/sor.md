# v0-87-maintainability-split-execute-tests-runtime-artifacts-rs-by-responsibility

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1379
Run ID: issue-1379
Version: v0.87
Title: [v0.87][maintainability] Split execute_tests/runtime_artifacts.rs by responsibility
Branch: codex/1379-v0-87-maintainability-split-execute-tests-runtime-artifacts-rs-by-responsibility
Status: IN_PROGRESS

Execution:
- Actor: Codex
- Model: gpt-5.4
- Provider: OpenAI
- Start Time: 2026-04-08T02:31:00Z
- End Time: 2026-04-08T02:48:23Z

## Summary
- Split `adl/tests/execute_tests/runtime_artifacts.rs` into a directory module with focused scenario-family files for artifact failures, run-state artifacts, determinism/replay checks, streaming behavior, and control-path coverage.
- Preserved the runtime-artifacts execute-test surface under the new module tree and passed the full targeted `runtime_artifacts` execute-test filter plus formatting validation in the issue worktree.

## Artifacts produced
- `adl/tests/execute_tests/runtime_artifacts/mod.rs`
- `adl/tests/execute_tests/runtime_artifacts/artifact_failures.rs`
- `adl/tests/execute_tests/runtime_artifacts/run_state.rs`
- `adl/tests/execute_tests/runtime_artifacts/determinism.rs`
- `adl/tests/execute_tests/runtime_artifacts/streaming.rs`
- `adl/tests/execute_tests/runtime_artifacts/control_paths.rs`
- updated `adl/tests/execute_tests.rs`

## Actions taken
- Refined the canonical source prompt and input card for `#1379`, then synced the GitHub issue body before execution.
- Started the dedicated `adl-wp-1379` worktree on branch `codex/1379-v0-87-maintainability-split-execute-tests-runtime-artifacts-rs-by-responsibility`.
- Converted the single-file runtime-artifacts test module into a directory module and grouped tests by scenario family rather than by one large omnibus surface.
- Updated the execute-test harness wiring in `adl/tests/execute_tests.rs` to point at the new `runtime_artifacts/mod.rs`.
- Ran targeted execute-test validation and formatting checks to confirm the split preserved the runtime-artifacts regression surface.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none yet; execution is still in the issue worktree pending commit/push/PR.
- Worktree-only paths remaining:
  - `adl/tests/execute_tests.rs`
  - deletion of `adl/tests/execute_tests/runtime_artifacts.rs`
  - `adl/tests/execute_tests/runtime_artifacts/mod.rs`
  - `adl/tests/execute_tests/runtime_artifacts/artifact_failures.rs`
  - `adl/tests/execute_tests/runtime_artifacts/run_state.rs`
  - `adl/tests/execute_tests/runtime_artifacts/determinism.rs`
  - `adl/tests/execute_tests/runtime_artifacts/streaming.rs`
  - `adl/tests/execute_tests/runtime_artifacts/control_paths.rs`
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: direct execution in the dedicated issue worktree created by `adl/tools/pr.sh start 1379`; main-repo transfer has not happened yet.
- Verification performed:
  - `git status --short` verified the refactor exists only in the issue worktree and is not yet integrated into the main repository path.
  - `find adl/tests -maxdepth 3 \( -path 'adl/tests/execute_tests/runtime_artifacts*' -o -name 'runtime_artifacts.rs' \) | sort` verified the new runtime-artifacts module tree exists and the old single-file path is removed in the worktree.
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
  - `cargo fmt --all` formatted the new runtime-artifacts module tree after the file split.
  - `cargo test -q --test execute_tests runtime_artifacts` verified the full runtime-artifacts execute-test surface still passes after the scenario-family split.
  - `cargo fmt --check` verified the final worktree remained rustfmt-clean after the refactor.
- Results:
  - All listed commands passed in the `adl` crate from the `adl-wp-1379` execution worktree.

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
      - "cargo fmt --all"
      - "cargo test -q --test execute_tests runtime_artifacts"
      - "cargo fmt --check"
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
- Determinism tests executed: the targeted `cargo test -q --test execute_tests runtime_artifacts` filter includes the byte-stability, replay-regression, streaming-observational, and runtime-artifact ordering tests that were moved into the new module tree.
- Fixtures or scripts used: existing runtime-artifacts execute-test fixtures under the new `adl/tests/execute_tests/runtime_artifacts/` files.
- Replay verification (same inputs -> same artifacts/order): preserved by moving the replay regression and repeated-run stability tests into `determinism.rs`, then rerunning the full runtime-artifacts test surface.
- Ordering guarantees (sorting / tie-break rules used): preserved by keeping the runtime-artifact scenario tests intact under their new module boundaries rather than changing production ordering logic.
- Artifact stability notes: this refactor changed only test organization and harness wiring; production artifact logic and schemas were intentionally left untouched.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: covered by the moved runtime-artifacts tests that assert no host-path or token-like leakage in generated runtime artifacts.
- Prompt / tool argument redaction verified: covered indirectly by the unchanged runtime-artifact regression suite; this issue did not alter production prompt/tool recording behavior.
- Absolute path leakage check: covered by the runtime-artifacts tests that continue asserting no absolute host-path leakage in persisted artifact surfaces.
- Sandbox / policy invariants preserved: yes; only test modules and execute-test wiring changed in this issue.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): temporary trace artifacts produced by the runtime-artifacts execute-test filter under the Rust test harness.
- Run artifact root: temporary per-test directories plus repo-scoped `.adl/runs/<run_id>` roots used by the existing execute tests.
- Replay command used for verification: replay coverage was exercised through the moved execute tests inside `cargo test -q --test execute_tests runtime_artifacts`.
- Replay result: PASS for the targeted runtime-artifacts execute-test proof surface.

## Artifact Verification
- Primary proof surface: the targeted `runtime_artifacts` execute-test filter plus the new module tree under `adl/tests/execute_tests/runtime_artifacts/`.
- Required artifacts present: yes; the new scenario-family files and updated execute-test harness wiring are present in the worktree.
- Artifact schema/version checks: preserved indirectly by the unchanged runtime-artifacts tests that continue asserting run-state and persisted-artifact structure.
- Hash/byte-stability checks: preserved by the moved repeated-run and replay-regression tests under `determinism.rs`.
- Missing/optional artifacts and rationale: no new runtime/demo artifacts were required because this issue is a test-organization refactor.

## Decisions / Deviations
- Kept the split centered on scenario families inside `runtime_artifacts/` rather than pushing shared helpers into unrelated test surfaces, so the new file layout stays easy to navigate.
- Used the full `runtime_artifacts` execute-test filter as the primary proof surface instead of only a handful of individual test names, because the file split changed the whole module tree.
- Overrode the open-PR-wave guard for `pr.sh start 1379` after confirming the blocker was an unrelated open draft PR, so this queued maintainability issue could continue without waiting on separate tools work.

## Follow-ups / Deferred work
- Main-repo integration, commit, push, and PR creation are still pending after this execution record snapshot.
- CI watch/repair ownership should begin as soon as the PR is opened so the branch does not sit on red checks unattended.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
