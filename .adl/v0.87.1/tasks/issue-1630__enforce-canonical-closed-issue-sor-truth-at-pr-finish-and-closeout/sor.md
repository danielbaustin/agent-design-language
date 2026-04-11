# enforce-canonical-closed-issue-sor-truth-at-pr-finish-and-closeout

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1630
Run ID: issue-1630
Version: v0.87.1
Title: [v0.87.1][tools] Enforce canonical closed-issue SOR truth at pr finish and closeout
Branch: codex/1630-enforce-canonical-closed-issue-sor-truth-at-pr-finish-and-closeout
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5-codex
- Provider: OpenAI Codex desktop
- Start Time: 2026-04-11T22:52:00Z
- End Time: 2026-04-11T23:15:38Z

## Summary

Added a canonical closed-issue SOR truth gate in the Rust lifecycle layer so `pr finish` refuses to publish already-closed issues whose root canonical `sor.md` still reports stale lifecycle truth, while `closeout` now proves that its post-normalization result is truthful before pruning worktree residue.

## Artifacts produced
- Updated `adl/src/cli/pr_cmd.rs` to enforce the closed-issue SOR truth gate during `pr finish` after canonical output sync and before publication proceeds.
- Updated `adl/src/cli/pr_cmd/lifecycle.rs` with a reusable canonical closed-issue SOR truth verifier plus post-closeout proof enforcement.
- Updated `adl/src/cli/tests/pr_cmd_inline/finish.rs` with a regression proving `pr finish` rejects stale closed-issue canonical SOR truth before any PR publication attempt.

## Actions taken
- Reviewed the existing `finish`, `closeout`, and lifecycle reconciliation paths to find the smallest enforcement seam that would not accidentally break doctor's self-healing close-bundle behavior.
- Added `ensure_closed_completed_issue_bundle_truth(...)` in the lifecycle module to verify the canonical root `sor.md` for closed issues reports `Status: DONE`, `Integration state: merged`, `Verification scope: main_repo`, `Worktree-only paths remaining: none`, and no duplicate task-bundle residue.
- Wired the new truth check into `pr finish` only when GitHub reports the issue is already `CLOSED` with `COMPLETED` state, so open-issue publication behavior is unchanged.
- Wired the same truth check into `closeout` after reconciliation so closeout now proves its own normalized result instead of assuming it.
- Added focused regression coverage for stale closed-issue rejection and normalized lifecycle truth acceptance.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths are updated on the issue branch via PR 1636
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: managed issue worktree with repo-native `pr finish` publication to branch push and open pull request
- Verification performed:
  - `git status --short`
    - verified the branch contains only the intended 1630 code and test changes before publication.
  - `git diff --check`
    - verified the patch is whitespace-clean before publication.
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
  - `cargo test --manifest-path adl/Cargo.toml ensure_closed_completed_issue_bundle_truth_rejects_stale_fields_and_duplicates -- --nocapture`
    - verified the new lifecycle truth checker rejects duplicate bundle residue plus stale closed-issue SOR lifecycle fields.
  - `cargo test --manifest-path adl/Cargo.toml ensure_closed_completed_issue_bundle_truth_accepts_normalized_bundle -- --nocapture`
    - verified the lifecycle truth checker accepts an already normalized canonical closed-issue bundle.
  - `cargo test --manifest-path adl/Cargo.toml real_pr_finish_rejects_closed_issue_with_stale_canonical_sor_truth -- --nocapture`
    - verified `pr finish` fails before PR publication when GitHub reports the issue as closed/completed but the canonical root SOR still reports stale truth.
  - `cargo test --manifest-path adl/Cargo.toml real_pr_closeout_reconciles_closed_completed_issue_bundle -- --nocapture`
    - verified `closeout` still normalizes and prunes successfully, with the new post-normalization truth check passing.
  - `cargo fmt --manifest-path adl/Cargo.toml --all`
    - normalized formatting for the touched Rust files.
  - `git diff --check`
    - verified the final patch is whitespace-clean.
- Results: PASS

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "cargo test --manifest-path adl/Cargo.toml ensure_closed_completed_issue_bundle_truth_rejects_stale_fields_and_duplicates -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml ensure_closed_completed_issue_bundle_truth_accepts_normalized_bundle -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml real_pr_finish_rejects_closed_issue_with_stale_canonical_sor_truth -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml real_pr_closeout_reconciles_closed_completed_issue_bundle -- --nocapture"
      - "cargo fmt --manifest-path adl/Cargo.toml --all"
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
- Determinism tests executed: targeted lifecycle truth-check unit tests and finish/closeout regression tests.
- Fixtures or scripts used: Rust lifecycle and inline `pr_cmd` fixture tests under `adl/src/cli/pr_cmd/lifecycle.rs` and `adl/src/cli/tests/pr_cmd_inline/finish.rs`.
- Replay verification (same inputs -> same artifacts/order): rerunning the same stale closed-issue fixtures yields the same bounded finish failure, while rerunning the normalized lifecycle fixture yields the same closeout success behavior.
- Ordering guarantees (sorting / tie-break rules used): duplicate bundle residue is surfaced from the sorted task-bundle match list, so the reported duplicate paths remain stable for identical repository state.
- Artifact stability notes: the tests prove stable failure/success behavior for identical canonical SOR fixture content; temporary fixture roots vary by test run as expected, but the user-facing gating outcome remains stable.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual review only; the added gate reads canonical repository-local issue bundle files and does not widen any credential or secret-handling path.
- Prompt / tool argument redaction verified: the new diagnostics reference issue numbers, repository-relative artifact classes, and canonical SOR field names only.
- Absolute path leakage check: committed code and recorded validation commands remain repository-relative; temporary fixture paths appear only in transient local test output, not in tracked artifacts.
- Sandbox / policy invariants preserved: the change stays inside the existing Rust lifecycle/tooling boundary and does not broaden network scope beyond the already-authorized GitHub issue-state lookup.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not_applicable; this tooling issue is proven by repository-local Rust regression tests rather than a runtime trace bundle.
- Run artifact root: temporary Rust fixture repositories created by the targeted lifecycle and finish tests.
- Replay command used for verification: `cargo test --manifest-path adl/Cargo.toml real_pr_finish_rejects_closed_issue_with_stale_canonical_sor_truth -- --nocapture`
- Replay result: PASS

## Artifact Verification
- Primary proof surface: `adl/src/cli/pr_cmd.rs`, `adl/src/cli/pr_cmd/lifecycle.rs`, and `adl/src/cli/tests/pr_cmd_inline/finish.rs`
- Required artifacts present: yes; the lifecycle gate, finish wiring, and regression coverage are all present in the worktree branch.
- Artifact schema/version checks: no schema changes; existing completed-phase SOR schema remains unchanged.
- Hash/byte-stability checks: not_applicable; proof is behavioral Rust test coverage, not a byte-stable generated artifact.
- Missing/optional artifacts and rationale: no separate demo artifact is required for this tooling guardrail issue.

## Decisions / Deviations
- Kept doctor's existing reconciliation behavior untouched for this issue so the new hard gate only affects `pr finish` on already-closed issues and post-normalization `closeout` proof.
- Used the existing older non-versioned local 1630 slug instead of regenerating the bundle, because the issue already had a coherent canonical local prompt/task bundle and generating another one would have added more residue.

## Follow-ups / Deferred work
- `#1632` remains the follow-on for automatic post-merge normalization so this guardrail failure becomes rare in day-to-day use rather than only explicit and actionable.
- `#1631` remains the separate residue-cleanup issue for legacy tracked/local record duplication patterns that predate this guardrail.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
