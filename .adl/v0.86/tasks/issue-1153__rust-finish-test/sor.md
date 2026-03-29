# ADL Output Card

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1153
Run ID: issue-1153
Version: v0.86
Title: rust-finish-test
Branch: codex/1153-rust-finish-test
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5 Codex
- Provider: Test
- Start Time: 2026-03-29T20:19:06Z
- End Time: 2026-03-29T20:19:09Z

## Summary

Finish test summary.

## Artifacts produced
- Code:
  - `adl/src/cli/pr_cmd.rs`
- Generated runtime artifacts: not_applicable for this tooling task

## Actions taken
- Added Rust finish handling.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: branch-local validation before draft PR publication
- Verification performed:
  - `cargo test --manifest-path adl/Cargo.toml pr_cmd`
    Verified Rust `pr` command tests.
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
  - `cargo test --manifest-path adl/Cargo.toml pr_cmd`
    Verified Rust `pr` command tests.
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
      - "cargo test --manifest-path adl/Cargo.toml pr_cmd"
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
- Determinism tests executed:
  - `cargo test --manifest-path adl/Cargo.toml pr_cmd`
- Fixtures or scripts used:
  - direct Rust unit coverage
- Replay verification (same inputs -> same artifacts/order):
  - PASS
- Ordering guarantees (sorting / tie-break rules used):
  - Stable section ordering.
- Artifact stability notes:
  - not_applicable beyond deterministic record rendering.

## Security / Privacy Checks
- Secret leakage scan performed:
  - Verified test output uses repo-relative paths only.
- Prompt / tool argument redaction verified:
  - Verified issue template text is not emitted in PR bodies.
- Absolute path leakage check:
  - PASS
- Sandbox / policy invariants preserved:
  - PASS

## Replay Artifacts
- Trace bundle path(s): not_applicable for this tooling task
- Run artifact root: not_applicable for this tooling task
- Replay command used for verification:
  - `cargo test --manifest-path adl/Cargo.toml pr_cmd`
- Replay result:
  - PASS

## Artifact Verification
- Primary proof surface:
  - `adl/src/cli/pr_cmd.rs`
- Required artifacts present:
  - yes
- Artifact schema/version checks:
  - none
- Hash/byte-stability checks:
  - not_applicable
- Missing/optional artifacts and rationale:
  - none

## Decisions / Deviations
- Kept the fixture minimal while satisfying completed-phase validation.

## Follow-ups / Deferred work
- none
