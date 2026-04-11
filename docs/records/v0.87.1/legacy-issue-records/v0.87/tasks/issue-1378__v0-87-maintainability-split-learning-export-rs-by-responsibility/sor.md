# v0-87-maintainability-split-learning-export-rs-by-responsibility

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1378
Run ID: issue-1378
Version: v0.87
Title: [v0.87][maintainability] Split learning_export.rs by responsibility
Branch: codex/1378-v0-87-maintainability-split-learning-export-rs-by-responsibility
Status: IN_PROGRESS

Execution:
- Actor: Codex
- Model: gpt-5.4
- Provider: OpenAI
- Start Time: 2026-04-08T02:08:00Z
- End Time: 2026-04-08T02:17:16Z

## Summary
- Split `adl/src/learning_export.rs` into a directory module with a thin public facade and focused submodules for dataset export, shared helpers, bundle-v1 export, trace-bundle-v2 export/import, and tests.
- Preserved the `learning_export` public API and kept caller/test compatibility surfaces unchanged while targeted determinism, import, smoke, and formatting checks passed in the execution worktree.

## Artifacts produced
- `adl/src/learning_export/mod.rs`
- `adl/src/learning_export/dataset.rs`
- `adl/src/learning_export/shared.rs`
- `adl/src/learning_export/bundle_v1.rs`
- `adl/src/learning_export/trace_bundle_v2.rs`
- `adl/src/learning_export/tests.rs`

## Actions taken
- Refined the `#1378` issue bundle and synced the canonical source prompt back to GitHub before execution started.
- Started worktree `adl-wp-1378` on branch `codex/1378-v0-87-maintainability-split-learning-export-rs-by-responsibility`.
- Replaced the monolithic `adl/src/learning_export.rs` file with a directory module that re-exports the existing public API from smaller internal files.
- Migrated the in-file unit tests into `adl/src/learning_export/tests.rs` so behavior and helper coverage stayed attached to the module after the split.
- Ran targeted Rust tests and `cargo fmt --check` to confirm the refactor preserved behavior and formatting.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none yet; execution is still in the issue worktree pending commit/push/PR.
- Worktree-only paths remaining:
  - `adl/src/learning_export/mod.rs`
  - `adl/src/learning_export/dataset.rs`
  - `adl/src/learning_export/shared.rs`
  - `adl/src/learning_export/bundle_v1.rs`
  - `adl/src/learning_export/trace_bundle_v2.rs`
  - `adl/src/learning_export/tests.rs`
  - deletion of `adl/src/learning_export.rs`
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: direct execution in the dedicated issue worktree created by `adl/tools/pr.sh start 1378`; main-repo transfer has not happened yet.
- Verification performed:
  - `git status --short` verified the refactor exists only in the issue worktree and is not yet integrated into the main repository path.
  - `find adl/src -maxdepth 2 \( -path 'adl/src/learning_export*' -o -name 'learning_export.rs' \) | sort` verified the new module tree exists and the old single-file path is removed from the worktree filesystem.
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
  - `cargo fmt --all` formatted the new module tree after the file split.
  - `cargo test -q learning_export::tests::export_jsonl_deterministic_for_fixture_runs` verified JSONL export remained byte-stable.
  - `cargo test -q learning_export::tests::export_bundle_v1_is_deterministic_and_path_safe` verified bundle-v1 determinism and path redaction remained intact.
  - `cargo test -q learning_export::tests::export_trace_bundle_v2_is_deterministic_and_manifest_hashes_match` verified trace-bundle manifest determinism and hash accounting remained intact.
  - `cargo test -q learning_export::tests::import_trace_bundle_v2_accepts_valid_bundle_and_returns_activation_log_path` verified valid trace-bundle import compatibility remained intact.
  - `cargo test -q learning_export::tests::import_trace_bundle_v2_rejects_missing_or_unsorted_manifest_surfaces` verified error-path checks for missing/unsorted manifests remained intact.
  - `cargo test -q --test cli_smoke learn_export_bundle_v1_is_deterministic` verified the CLI still emits the expected bundle-v1 root and deterministic artifacts.
  - `cargo test -q --test cli_smoke learn_export_trace_bundle_v2_is_deterministic_and_sanitized` verified the CLI still emits the expected trace-bundle-v2 root with sanitized deterministic artifacts.
  - `cargo fmt --check` verified the final worktree remained rustfmt-clean after the refactor.
- Results:
  - All listed commands passed in the `adl` crate from the `adl-wp-1378` execution worktree.

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
      - "cargo test -q learning_export::tests::export_jsonl_deterministic_for_fixture_runs"
      - "cargo test -q learning_export::tests::export_bundle_v1_is_deterministic_and_path_safe"
      - "cargo test -q learning_export::tests::export_trace_bundle_v2_is_deterministic_and_manifest_hashes_match"
      - "cargo test -q learning_export::tests::import_trace_bundle_v2_accepts_valid_bundle_and_returns_activation_log_path"
      - "cargo test -q learning_export::tests::import_trace_bundle_v2_rejects_missing_or_unsorted_manifest_surfaces"
      - "cargo test -q --test cli_smoke learn_export_bundle_v1_is_deterministic"
      - "cargo test -q --test cli_smoke learn_export_trace_bundle_v2_is_deterministic_and_sanitized"
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
- Determinism tests executed: `export_jsonl_deterministic_for_fixture_runs`, `export_bundle_v1_is_deterministic_and_path_safe`, `export_trace_bundle_v2_is_deterministic_and_manifest_hashes_match`, `learn_export_bundle_v1_is_deterministic`, and `learn_export_trace_bundle_v2_is_deterministic_and_sanitized`.
- Fixtures or scripts used: in-module fixture runs under `adl/src/learning_export/tests.rs` plus CLI smoke fixtures under `adl/tests/cli_smoke/exports_and_remote.rs`.
- Replay verification (same inputs -> same artifacts/order): verified by repeated exports to separate output roots and byte-for-byte comparison of JSONL and manifest outputs in both unit and smoke tests.
- Ordering guarantees (sorting / tie-break rules used): preserved explicit/derived run-id sorting, step-record sorting by `step_id`, manifest file-entry sorting by relative path, and suggestion/category canonicalization.
- Artifact stability notes: the refactor was structural only; bundle roots, manifest names, and serialized export/import semantics were intentionally left unchanged.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: covered by `export_bundle_v1_is_deterministic_and_path_safe` and `learn_export_trace_bundle_v2_is_deterministic_and_sanitized`, which assert no token-like secrets appear in generated artifacts.
- Prompt / tool argument redaction verified: not specifically applicable beyond export payload sanitization; the issue does not change prompt/tool recording code.
- Absolute path leakage check: covered by the bundle-v1 and trace-bundle sanitization tests, which assert generated artifacts do not contain common Unix home-path prefixes or other host-path leakage.
- Sandbox / policy invariants preserved: yes; the refactor only reorganized `learning_export` internals and did not alter execution-policy, sandbox, or runtime contract code.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): temporary test outputs under the Rust test harness for `trace_bundle_v2/` exports.
- Run artifact root: temporary per-test directories under `std::env::temp_dir()`.
- Replay command used for verification: no standalone replay command was required; import compatibility was verified by `cargo test -q learning_export::tests::import_trace_bundle_v2_accepts_valid_bundle_and_returns_activation_log_path`.
- Replay result: PASS for import-path compatibility; full replay execution was out of scope for this bounded structural refactor.

## Artifact Verification
- Primary proof surface: Rust unit tests in `adl/src/learning_export/tests.rs` plus CLI smoke tests in `adl/tests/cli_smoke/exports_and_remote.rs`.
- Required artifacts present: yes; the new `adl/src/learning_export/` module files exist in the worktree and replace the deleted `adl/src/learning_export.rs` monolith.
- Artifact schema/version checks: verified indirectly by passing import/export tests that still exercise `DATASET_VERSION`, `BUNDLE_VERSION`, and `TRACE_BUNDLE_VERSION` surfaces without schema changes.
- Hash/byte-stability checks: verified by manifest hash assertions and repeated-export byte comparisons in both unit and smoke tests.
- Missing/optional artifacts and rationale: no new external artifacts were required because this issue is a code-only maintainability refactor.

## Decisions / Deviations
- Kept the public `learning_export` API as a façade in `mod.rs` so callers in `adl/src/cli/commands.rs` and downstream tests stayed unchanged.
- Moved the existing in-file unit tests into `adl/src/learning_export/tests.rs` rather than expanding CLI smoke coverage further, because the bounded issue goal was a structural split with behavior preservation.
- Used targeted proof coverage instead of a full workspace test sweep because the changed surface is tightly contained and the selected tests lock down the compatibility-sensitive export/import behaviors.

## Follow-ups / Deferred work
- Main-repo integration, commit, push, and PR creation are still pending after this execution record snapshot.
- CI watch/repair ownership should begin as soon as the PR is opened so the branch does not sit on red checks unattended.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
