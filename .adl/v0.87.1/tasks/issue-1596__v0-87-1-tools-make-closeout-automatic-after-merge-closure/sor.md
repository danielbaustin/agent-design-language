# v0-87-1-tools-make-closeout-automatic-after-merge-closure

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1596
Run ID: issue-1596
Version: v0.87.1
Title: [v0.87.1][tools] Make closeout automatic after merge/closure
Branch: codex/1596-v0-87-1-tools-make-closeout-automatic-after-merge-closure
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5
- Provider: openai
- Start Time: 2026-04-11T17:00:00Z
- End Time: 2026-04-11T17:52:47Z

## Summary
Added a real Rust-owned `adl pr closeout` surface, reused the same closeout lifecycle path from doctor, and wired merge-mode finish into automatic closeout once the issue reaches `CLOSED/COMPLETED`.

## Artifacts produced
- `adl/src/cli/pr_cmd.rs`
- `adl/src/cli/pr_cmd/doctor.rs`
- `adl/src/cli/pr_cmd/lifecycle.rs`
- `adl/src/cli/pr_cmd_args.rs`
- `adl/src/cli/tests/pr_cmd_inline/basics.rs`
- `adl/src/cli/tests/pr_cmd_inline/lifecycle.rs`
- `adl/src/cli/usage.rs`
- `adl/tools/pr.sh`
- `adl/tools/skills/pr-closeout/SKILL.md`
- `adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`

## Actions taken
- Added `parse_closeout_args(...)` plus a new `real_pr_closeout(...)` command in the Rust PR control plane.
- Added shared lifecycle helpers to verify `CLOSED/COMPLETED` state, wait for post-merge closure, reconcile the canonical bundle, and safely prune the matching issue worktree.
- Reused the same closeout path from `doctor` for closed/completed issue recovery so drift repair no longer follows a separate code path.
- Taught `adl/tools/pr.sh` and the operator docs about the new `closeout` command and the automatic control-plane-triggered closeout behavior.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none yet; branch-local tracked edits are prepared for PR publication only
- Worktree-only paths remaining: `adl/src/cli/pr_cmd.rs`, `adl/src/cli/pr_cmd/doctor.rs`, `adl/src/cli/pr_cmd/lifecycle.rs`, `adl/src/cli/pr_cmd_args.rs`, `adl/src/cli/tests/pr_cmd_inline/basics.rs`, `adl/src/cli/tests/pr_cmd_inline/lifecycle.rs`, `adl/src/cli/usage.rs`, `adl/tools/pr.sh`, `adl/tools/skills/pr-closeout/SKILL.md`, `adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: branch-local tracked edits validated in the issue worktree and prepared for `pr finish`
- Verification performed:
  - `git status --short`
    - verified the bounded closeout-related tracked paths on the issue branch.
  - `git diff --check`
    - verified the final diff is clean and publication-safe.
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
  - `cargo test --manifest-path adl/Cargo.toml closeout -- --nocapture`
    - verified the new closeout argument parsing surface and closeout-focused command plumbing.
  - `cargo test --manifest-path adl/Cargo.toml real_pr_doctor_reconciles_closed_completed_issue_bundle_without_worktree -- --nocapture`
    - verified closed/completed reconciliation now runs through the shared closeout path and prunes the matching issue worktree safely.
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
      - "cargo test --manifest-path adl/Cargo.toml closeout -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml real_pr_doctor_reconciles_closed_completed_issue_bundle_without_worktree -- --nocapture"
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
- Determinism tests executed: closeout parser tests plus the closed/completed reconciliation lifecycle test.
- Fixtures or scripts used: deterministic Rust inline `pr_cmd` lifecycle fixtures with mocked GitHub issue state and synthetic duplicate bundle/worktree state.
- Replay verification (same inputs -> same artifacts/order): yes; the lifecycle fixture repeatedly produces the same reconciled `DONE` / `merged` / `main_repo` output and worktree-prune outcome.
- Ordering guarantees (sorting / tie-break rules used): closeout verifies closed/completed state before reconciliation and prunes the issue worktree only after bundle truth is normalized.
- Artifact stability notes: the closeout path now centralizes previously split post-merge reconciliation behavior in one lifecycle pipeline.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual inspection of the new closeout helpers and tests for secret-free lifecycle inputs.
- Prompt / tool argument redaction verified: yes; the closeout surface uses issue/branch/worktree metadata only.
- Absolute path leakage check: output record uses repository-relative paths only.
- Sandbox / policy invariants preserved: yes; pruning is issue-targeted and refuses dirty worktrees.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable for this tooling issue.
- Run artifact root: not applicable.
- Replay command used for verification: `cargo test --manifest-path adl/Cargo.toml real_pr_doctor_reconciles_closed_completed_issue_bundle_without_worktree -- --nocapture`
- Replay result: PASS.

## Artifact Verification
- Primary proof surface: the shared closeout lifecycle helpers in `adl/src/cli/pr_cmd/lifecycle.rs` and the closed/completed reconciliation fixture.
- Required artifacts present: true
- Artifact schema/version checks: CLI usage, `pr.sh`, and skill docs were kept aligned with the new closeout surface.
- Hash/byte-stability checks: not performed; lifecycle fixtures are the proving surface here.
- Missing/optional artifacts and rationale: no demo artifact or runtime trace bundle is required for this workflow-tooling issue.

## Decisions / Deviations
- Implemented a real `adl pr closeout` control-plane surface first, then reused it from merge-mode finish and doctor rather than embedding more post-merge heuristics in multiple places.

## Follow-ups / Deferred work
- `#1597` adds finish-time milestone-doc drift checks so release-tail doc issues are blocked before publication instead of being discovered later in review.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
