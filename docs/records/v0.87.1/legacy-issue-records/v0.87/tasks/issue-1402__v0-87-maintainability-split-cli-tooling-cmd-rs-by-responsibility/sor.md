# v0-87-maintainability-split-cli-tooling-cmd-rs-by-responsibility

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1402
Run ID: issue-1402
Version: v0.87
Title: [v0.87][maintainability] Split cli/tooling_cmd.rs by responsibility
Branch: codex/1402-v0-87-maintainability-split-cli-tooling-cmd-rs-by-responsibility
Status: IN_PROGRESS

Execution:
- Actor: Codex
- Model: gpt-5.4
- Provider: OpenAI
- Start Time: 2026-04-08T03:05:00Z
- End Time: 2026-04-08T03:28:13Z

## Summary
- Split `adl/src/cli/tooling_cmd.rs` into a thin 76-line dispatcher plus focused command/helper modules for card prompts, structured-prompt validation, review-surface checks, review-contract validation, markdown parsing, and shared CLI helpers.
- Extracted the `tooling_cmd` tests into `adl/src/cli/tooling_cmd/tests.rs`, preserving the existing fixture-driven command coverage while making the production façade easier to navigate.

## Artifacts produced
- `adl/src/cli/tooling_cmd.rs`
- `adl/src/cli/tooling_cmd/card_prompt.rs`
- `adl/src/cli/tooling_cmd/common.rs`
- `adl/src/cli/tooling_cmd/markdown.rs`
- `adl/src/cli/tooling_cmd/review_contract.rs`
- `adl/src/cli/tooling_cmd/review_surface.rs`
- `adl/src/cli/tooling_cmd/structured_prompt.rs`
- `adl/src/cli/tooling_cmd/tests.rs`

## Actions taken
- Kept `real_tooling` and the public `adl tooling ...` dispatch contract in `adl/src/cli/tooling_cmd.rs` while moving cohesive command families and helpers into `adl/src/cli/tooling_cmd/`.
- Isolated structured-prompt parsing and validation logic into `structured_prompt.rs`, the card-prompt generation flow into `card_prompt.rs`, and review-output/repo-review checks into dedicated modules.
- Moved markdown/front-matter helpers into `markdown.rs` and path/git/content-safety helpers into `common.rs`.
- Extracted the inline `#[cfg(test)]` module into `adl/src/cli/tooling_cmd/tests.rs` and rewired the parent file to `mod tests;`.
- Preserved existing command behavior instead of broadening scope into help-flow changes for subcommands whose `--help` behavior was already historically inconsistent.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none yet; execution is still isolated in the dedicated `#1402` issue worktree pending commit, push, and PR.
- Worktree-only paths remaining:
  - `adl/src/cli/tooling_cmd.rs`
  - `adl/src/cli/tooling_cmd/card_prompt.rs`
  - `adl/src/cli/tooling_cmd/common.rs`
  - `adl/src/cli/tooling_cmd/markdown.rs`
  - `adl/src/cli/tooling_cmd/review_contract.rs`
  - `adl/src/cli/tooling_cmd/review_surface.rs`
  - `adl/src/cli/tooling_cmd/structured_prompt.rs`
  - `adl/src/cli/tooling_cmd/tests.rs`
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: direct execution in the dedicated issue worktree created by `adl/tools/pr.sh start 1402`; main-repo transfer has not happened yet.
- Verification performed:
  - `git status --short` verified the `tooling_cmd` refactor is present only in the issue worktree before PR packaging.
  - `wc -l adl/src/cli/tooling_cmd.rs adl/src/cli/tooling_cmd/*.rs | sort -nr` verified the façade and extracted module sizes after the split.
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
  - `cargo test -q tooling_cmd` verified the extracted `tooling_cmd` Rust test surface still passes after the split.
  - `cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- tooling lint-prompt-spec --input .adl/v0.87/tasks/issue-1402__v0-87-maintainability-split-cli-tooling-cmd-rs-by-responsibility/sip.md` verified the public `lint-prompt-spec` dispatch path still validates a real card surface.
  - `cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- tooling validate-structured-prompt --type sip --input .adl/v0.87/tasks/issue-1402__v0-87-maintainability-split-cli-tooling-cmd-rs-by-responsibility/sip.md` verified the public structured-prompt validator still works through the CLI surface.
  - `cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- tooling review-card-surface --input .adl/v0.87/tasks/issue-1402__v0-87-maintainability-split-cli-tooling-cmd-rs-by-responsibility/sip.md --output .adl/v0.87/tasks/issue-1402__v0-87-maintainability-split-cli-tooling-cmd-rs-by-responsibility/sor.md` verified the review-surface command still emits the expected YAML summary.
  - `cargo fmt --check` verified the extracted module tree is rustfmt-clean.
- Results:
  - All listed commands passed.

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
      - "cargo test -q tooling_cmd"
      - "cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- tooling lint-prompt-spec --input .adl/v0.87/tasks/issue-1402__v0-87-maintainability-split-cli-tooling-cmd-rs-by-responsibility/sip.md"
      - "cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- tooling validate-structured-prompt --type sip --input .adl/v0.87/tasks/issue-1402__v0-87-maintainability-split-cli-tooling-cmd-rs-by-responsibility/sip.md"
      - "cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- tooling review-card-surface --input .adl/v0.87/tasks/issue-1402__v0-87-maintainability-split-cli-tooling-cmd-rs-by-responsibility/sip.md --output .adl/v0.87/tasks/issue-1402__v0-87-maintainability-split-cli-tooling-cmd-rs-by-responsibility/sor.md"
      - "cargo fmt --check"
  determinism:
    status: PARTIAL
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
- Determinism tests executed: `cargo test -q tooling_cmd` reran the existing fixture-based command tests after the module split.
- Fixtures or scripts used: the extracted `adl/src/cli/tooling_cmd/tests.rs` fixture helpers and the real `#1402` input/output cards under `.adl/v0.87/tasks/...`.
- Replay verification (same inputs -> same artifacts/order): not separately rerun as a byte-for-byte replay check because this issue is an internal module split, not an artifact-schema change.
- Ordering guarantees (sorting / tie-break rules used): preserved indirectly by the unchanged validator tests and review-surface output checks.
- Artifact stability notes: the refactor preserved command outputs and review-surface summary generation while only changing internal file boundaries.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: the preserved `tooling_cmd` tests still cover secret-like token and absolute-host-path validation helpers after extraction.
- Prompt / tool argument redaction verified: unchanged validator logic remained behind the same CLI commands and passed the extracted test surface.
- Absolute path leakage check: `cargo run ... tooling review-card-surface ...` and the extracted tests continued to emit repo-relative references without new absolute host paths.
- Sandbox / policy invariants preserved: yes; this issue only reorganized Rust source and test modules inside the dedicated worktree.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable; this maintainability refactor does not produce trace bundles.
- Run artifact root: not applicable.
- Replay command used for verification: not applicable.
- Replay result: not applicable.

## Artifact Verification
- Primary proof surface: the extracted `tooling_cmd` module tree plus the `cargo test -q tooling_cmd` and direct CLI validation commands listed above.
- Required artifacts present: yes; the façade file and all extracted module files exist in the issue worktree.
- Artifact schema/version checks: not applicable because this issue does not change runtime artifact schemas.
- Hash/byte-stability checks: not run; this issue preserves command behavior but does not introduce new artifact formats.
- Missing/optional artifacts and rationale: no dedicated demo artifacts were required for this bounded internal refactor.

## Decisions / Deviations
- Kept `adl/src/cli/tooling_cmd.rs` as the stable façade file rather than converting the parent into `mod.rs`, which reduced wiring churn in `adl/src/cli/mod.rs` while still shrinking the hotspot from 2413 lines to 76.
- Used real card inputs for the public CLI smoke checks instead of `--help` for every subcommand, because some tooling subcommands already had pre-existing help-path behavior that falls through into argument validation and this refactor intentionally did not change those semantics.
- Finished the final `tests.rs` wiring locally after the dedicated subagent stopped on the correct conflict boundary, preserving the safe extracted test file it had already created.

## Follow-ups / Deferred work
- Commit, push, PR creation, and CI watch ownership are still pending after this execution snapshot.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
