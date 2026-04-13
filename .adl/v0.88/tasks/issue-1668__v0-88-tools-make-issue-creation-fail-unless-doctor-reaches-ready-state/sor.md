# v0-88-tools-make-issue-creation-fail-unless-doctor-reaches-ready-state

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1668
Run ID: issue-1668
Version: v0.88
Title: [v0.88][tools] Make issue creation fail unless doctor reaches ready state
Branch: codex/1668-v0-88-tools-make-issue-creation-fail-unless-doctor-reaches-ready-state
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5 Codex
- Provider: OpenAI
- Start Time: 2026-04-13T00:48:37Z
- End Time: 2026-04-13T01:03:58Z

## Summary

Made `pr create` prove immediate doctor-ready structural state after bootstrap. The create path now fails with actionable output when the new issue bundle is not ready, while still leaving deterministic repair evidence in place.

## Artifacts produced
- Code:
  - `adl/src/cli/pr_cmd.rs`
  - `adl/src/cli/pr_cmd/doctor.rs`
  - `adl/src/cli/tests/pr_cmd_inline/basics.rs`
  - `adl/tools/pr.sh`
- Generated runtime artifacts: not_applicable for this CLI control-plane issue

## Actions taken
- Reused the canonical doctor-ready validation path from `pr create` immediately after root-bundle bootstrap.
- Added an end-to-end regression seam for post-bootstrap readiness failure and updated create-path expectations to match the stronger contract.
- Updated `pr.sh create` help text so the published operator contract matches the new ready-gated behavior.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths are updated on the issue branch via PR 1701
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: branch commit plus manual PR publication after `pr finish` validation hit the known ignored-bundle and legacy-residue blocker
- Verification performed:
  - `git status --short`
    Verified the bounded implementation file set was committed cleanly in the issue worktree before and after publication.
  - `git diff --check`
    Verified there are no whitespace or patch-application defects in the touched paths.
  - `gh pr view 1701 --repo danielbaustin/agent-design-language --json state,isDraft,url`
    Verified PR 1701 is open and non-draft on the issue branch.
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
  - `cargo fmt --manifest-path adl/Cargo.toml --all -- --check`
    Verified the Rust CLI changes and tests are formatted correctly.
  - `cargo test --manifest-path adl/Cargo.toml real_pr_create_creates_issue_and_bootstraps_root_bundle -- --nocapture`
    Verified the authored-body happy path now reaches immediate doctor-ready `pre_run` state without creating a worktree.
  - `cargo test --manifest-path adl/Cargo.toml real_pr_create_fails_when_post_bootstrap_ready_validation_fails -- --nocapture`
    Verified create fails deterministically when the freshly bootstrapped bundle is not ready after bootstrap.
  - `cargo test --manifest-path adl/Cargo.toml real_pr_create_ -- --nocapture`
    Verified the broader create-path regression slice, including generated-body and validation guard behavior, still matches the stronger contract.
- Results:
  - All listed validations passed.

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
      - "cargo fmt --manifest-path adl/Cargo.toml --all -- --check"
      - "cargo test --manifest-path adl/Cargo.toml real_pr_create_creates_issue_and_bootstraps_root_bundle -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml real_pr_create_fails_when_post_bootstrap_ready_validation_fails -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml real_pr_create_ -- --nocapture"
  determinism:
    status: PASS
    replay_verified: false
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
- Determinism tests executed:
  - `real_pr_create_creates_issue_and_bootstraps_root_bundle`
  - `real_pr_create_fails_when_post_bootstrap_ready_validation_fails`
  - `real_pr_create_`
- Fixtures or scripts used:
  - create-path Rust integration fixtures in `adl/src/cli/tests/pr_cmd_inline/basics.rs`
- Replay verification (same inputs -> same artifacts/order):
  - Not fully replay-verified; the test slice proves stable success and failure classes for the same create-path fixtures rather than byte-for-byte replay artifacts.
- Ordering guarantees (sorting / tie-break rules used):
  - Not applicable for this create-path guardrail change.
- Artifact stability notes:
  - The create path now produces stable `READY PASS pre_run` output for authored-body success and stable readiness failure for non-ready bundles.

## Security / Privacy Checks
- Secret leakage scan performed:
  - Manual review of the touched Rust/test/shell surfaces; no secret-bearing material was added.
- Prompt / tool argument redaction verified:
  - The change reuses existing doctor-ready diagnostics and does not add prompt/tool-argument recording to tracked artifacts.
- Absolute path leakage check:
  - `git diff --check` passed and the recorded SOR content uses repository-relative paths only.
- Sandbox / policy invariants preserved:
  - The work stayed within the bound worktree and did not create execution context during `pr create`.

## Replay Artifacts
- Trace bundle path(s): not_applicable for this CLI control-plane issue
- Run artifact root: not_applicable for this CLI control-plane issue
- Replay command used for verification: not_applicable for this CLI control-plane issue
- Replay result: not_applicable for this CLI control-plane issue

## Artifact Verification
- Primary proof surface:
  - Rust create-path regression tests
- Required artifacts present:
  - Yes; the code changes and create-path regression coverage are present in the worktree.
- Artifact schema/version checks:
  - Existing prompt-card schemas remained unchanged.
- Hash/byte-stability checks:
  - Not run; command and test proofs were sufficient for this control-plane behavior change.
- Missing/optional artifacts and rationale:
  - No replay bundle was required because this issue changes create-path validation rather than the runtime replay substrate.

## Decisions / Deviations
- Kept the create gate on doctor-ready only and did not include preflight/open-PR-wave blocking, because issue creation should prove structural readiness without turning into queue admission.
- Used a test-only post-bootstrap hook to prove the failure path end-to-end without widening the production create surface.
- `pr finish` validated the issue bundle but could not publish because ignored canonical `.adl` bundle paths and legacy tracked `.adl` residue still block the automated publication path on this repo, so publication was completed manually.

## Follow-ups / Deferred work
- The workflow conductor still routed the finished 1668 worktree back to `pr-run` because `open_pr_wave_only` outweighed the post-implementation state; that maturity gap should be fixed separately so finished issues hand off to `pr-finish` or janitor surfaces more naturally.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
