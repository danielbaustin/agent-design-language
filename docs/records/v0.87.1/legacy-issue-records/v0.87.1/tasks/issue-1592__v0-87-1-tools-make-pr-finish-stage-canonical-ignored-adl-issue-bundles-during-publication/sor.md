# v0-87-1-tools-make-pr-finish-stage-canonical-ignored-adl-issue-bundles-during-publication

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1592
Run ID: issue-1592
Version: v0.87.1
Title: [v0.87.1][tools] Make pr finish stage canonical ignored .adl issue bundles during publication
Branch: codex/1592-v0-87-1-tools-make-pr-finish-stage-canonical-ignored-adl-issue-bundles-during-publication
Status: DONE

Execution:
- Actor: Codex
- Model: gpt-5-codex
- Provider: OpenAI Codex desktop
- Start Time: 2026-04-11T17:05:00Z
- End Time: 2026-04-11T17:22:59Z

## Summary
Fixed the Rust `pr finish` publication path so it stages the current issue's canonical `.adl` issue body and `stp.md` / `sip.md` / `sor.md` bundle files even when `.adl/` is gitignored. The finish flow now succeeds for both mixed tracked-plus-ignored publication and ignored-bundle-only publication.

## Artifacts produced
- Updated `adl/src/cli/pr_cmd.rs` to stage the selected publish paths plus the current issue's canonical bundle with a bounded force-add path.
- Updated `adl/src/cli/tests/pr_cmd_inline/finish.rs` with helper coverage for force-staging ignored bundle files, a mixed tracked-plus-ignored publication assertion, and a new ignored-bundle-only `real_pr finish` regression test.

## Actions taken
- Reviewed the finish lifecycle ordering to confirm the failure happened after canonical output sync but before publication staging.
- Changed `finish` to compute the current issue's canonical bundle paths, run the normal `git add` for selected tracked paths, and then force-add only the bounded current-issue canonical bundle artifacts.
- Moved the no-change check to happen after staging so ignored-bundle-only publication can succeed instead of falsely reporting "Nothing to PR."
- Added regression tests for both the mixed tracked-plus-ignored case and the ignored-bundle-only case.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `.adl/v0.87.1/tasks/issue-1592__v0-87-1-tools-make-pr-finish-stage-canonical-ignored-adl-issue-bundles-during-publication/sor.md`
- Worktree-only paths remaining: none
- Integration state: merged
- Verification scope: main_repo
- Integration method used: normalized the canonical root SOR directly on `main` after verifying the issue is already closed and linked to merged PR `#1599`
- Verification performed:
  - `gh issue view 1592 --json title,url,state,stateReason,closedByPullRequestsReferences`
    - verified the issue is closed and captured the final closure metadata used for this normalization pass
  - `gh pr view 1599 --json state,url`
    - verified the linked closing PR remains available as the final publication surface
  - `ls .adl/v0.87.1/tasks/issue-1592__v0-87-1-tools-make-pr-finish-stage-canonical-ignored-adl-issue-bundles-during-publication/sor.md`
    - verified the canonical root SOR path exists on the main repository path
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `cargo test --manifest-path adl/Cargo.toml finish_helper_paths_cover_nonempty_and_staged_checks -- --nocapture` verified the staging helper still handles tracked staging and now force-stages the bounded ignored bundle file path.
  - `cargo test --manifest-path adl/Cargo.toml real_pr_finish_creates_draft_pr_and_commits_branch_changes -- --nocapture` verified mixed tracked-plus-ignored publication now commits and publishes the canonical `.adl` issue bundle alongside tracked changes.
  - `cargo test --manifest-path adl/Cargo.toml real_pr_finish_publishes_ignored_canonical_bundle_when_no_tracked_changes_remain -- --nocapture` verified `pr finish` succeeds when the only publishable artifacts are the ignored canonical issue body and task-bundle files.
  - `git diff --check` verified the patch is whitespace-clean after formatting.
- Results: PASS

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
      - "cargo test --manifest-path adl/Cargo.toml finish_helper_paths_cover_nonempty_and_staged_checks -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml real_pr_finish_creates_draft_pr_and_commits_branch_changes -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml real_pr_finish_publishes_ignored_canonical_bundle_when_no_tracked_changes_remain -- --nocapture"
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
- Determinism tests executed: repeated targeted `finish` regressions covering tracked-plus-ignored staging and ignored-bundle-only publication.
- Fixtures or scripts used: the Rust `real_pr_finish_*` integration tests in `adl/src/cli/tests/pr_cmd_inline/finish.rs`.
- Replay verification (same inputs -> same artifacts/order): rerunning the targeted finish regressions with identical fixture inputs produced the same successful publication outcomes and the same canonical bundle paths in the commit tree.
- Ordering guarantees (sorting / tie-break rules used): the current issue's canonical publish paths are now collected into a stable sorted set before force-add, so the bounded ignored-path staging set is deterministic for the same issue inputs.
- Artifact stability notes: the tests prove stable publication of the same canonical issue body and task-bundle file set; temporary fixture directories vary by test run as expected, but the committed path set and success behavior remain stable.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual review of the staging helper and new test fixtures confirmed the change only stages canonical issue-bundle paths and does not serialize credentials or secret values.
- Prompt / tool argument redaction verified: the change operates on repository-relative issue-bundle paths and does not widen any prompt or tool-argument recording surfaces.
- Absolute path leakage check: no tracked output or validation record depends on absolute host paths; test fixtures use temporary directories internally but the committed behavior and recorded commands remain repository-relative.
- Sandbox / policy invariants preserved: the fix stays inside the local git publication path and does not broaden network or filesystem scope beyond bounded current-issue publication.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable; this issue is proven by repository-local Rust regression tests rather than a separate trace bundle.
- Run artifact root: temporary Rust test repositories created by the targeted `real_pr_finish_*` fixture tests.
- Replay command used for verification: `cargo test --manifest-path adl/Cargo.toml real_pr_finish_creates_draft_pr_and_commits_branch_changes -- --nocapture` and `cargo test --manifest-path adl/Cargo.toml real_pr_finish_publishes_ignored_canonical_bundle_when_no_tracked_changes_remain -- --nocapture`
- Replay result: PASS for both mixed tracked-plus-ignored and ignored-bundle-only publication.

## Artifact Verification
- Primary proof surface: `adl/src/cli/pr_cmd.rs` and `adl/src/cli/tests/pr_cmd_inline/finish.rs`
- Required artifacts present: yes; the bounded force-add logic and both regression proofs are present in the worktree.
- Artifact schema/version checks: no artifact schema changes were introduced; this issue changes finish-publication behavior only.
- Hash/byte-stability checks: not run separately; proof is behavioral and path-set based via the targeted Rust fixture tests.
- Missing/optional artifacts and rationale: no documentation changes were required for this issue because the behavior fix is internal to `finish` publication.

## Decisions / Deviations
- Fixed the underlying bounded publication bug first instead of changing flag/help text in this issue; operator-facing flag semantics remain available as a narrower follow-up.
- Kept the force-add scope limited to the current issue's canonical body/STP/SIP/SOR files rather than widening ignored-path publication generally.

## Follow-ups / Deferred work
- `#1593` remains the follow-up to decide whether `--allow-gitignore` should now be narrowed, documented differently, or otherwise made explicitly truthful after the bounded publication fix.
- `pr finish` still needs to publish this branch so the Main Repo Integration section can be normalized from `worktree_only` to the actual PR state.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
