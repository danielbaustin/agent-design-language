# v0-87-1-tools-enforce-github-issue-metadata-parity-with-canonical-adl-v0-87-1-issue-prompts

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1607
Run ID: issue-1607
Version: v0.87.1
Title: [v0.87.1][tools] Enforce GitHub issue metadata parity with canonical .adl v0.87.1 issue prompts
Branch: codex/1607-v0-87-1-tools-enforce-github-issue-metadata-parity-with-canonical-adl-v0-87-1-issue-prompts
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5 Codex
- Provider: OpenAI
- Start Time: 2026-04-11T19:55:00Z
- End Time: 2026-04-11T20:35:00Z

## Summary
Enforced GitHub issue metadata parity in the PR control plane by normalizing version-prefixed titles, repairing missing or stale version labels during `pr create` and `pr init`/`pr run`, rejecting duplicate local issue identities, and adding a milestone audit script for tracker metadata drift.

## Artifacts produced
- `adl/src/cli/pr_cmd.rs`
- `adl/src/cli/pr_cmd/github.rs`
- `adl/src/cli/pr_cmd_prompt.rs`
- `adl/src/cli/tests/pr_cmd_inline/basics.rs`
- `adl/src/cli/tests/pr_cmd_inline/repo_helpers.rs`
- `adl/tools/check_issue_metadata_parity.sh`
- `adl/tools/test_check_issue_metadata_parity.sh`

## Actions taken
- Replaced the bootstrap source prompt/STP for issue `#1607` with an authored, reviewable execution target before binding execution.
- Added title normalization for version-prefixed issue titles so manual/bootstrap drift is corrected to the canonical milestone title form.
- Added GitHub issue metadata parity enforcement that repairs missing labels, removes stale version labels, and aligns the GitHub issue title with the expected canonical title.
- Added duplicate local issue-identity detection so split prompt/task-bundle variants for the same issue number are rejected instead of silently selected.
- Added a bounded audit/check script for milestone metadata drift plus a focused shell test for the new audit surface.
- Added regression coverage for title normalization, duplicate local identities, and init-time repair of missing GitHub version metadata.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths are updated on the issue branch via PR 1620
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: managed issue worktree with committed branch push and open pull request; canonical `.adl` issue bundle was force-staged for publication
- Verification performed:
  - `git status --short`
    - verifies the branch contains only the intended tracked changes before publication.
  - `rg --files .adl/v0.87.1/bodies .adl/v0.87.1/tasks adl/src/cli adl/tools | rg '1607|check_issue_metadata_parity|pr_cmd.rs$|pr_cmd_prompt.rs$|github.rs$|repo_helpers.rs$|basics.rs$'`
    - verifies the expected proof surfaces exist in the worktree.
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
  - `cargo test --manifest-path adl/Cargo.toml normalize_issue_title_for_version_adds_or_replaces_prefix -- --nocapture`
    - verifies milestone title normalization.
  - `cargo test --manifest-path adl/Cargo.toml ensure_no_duplicate_issue_identities_rejects_duplicate_prompt_or_task_bundle -- --nocapture`
    - verifies duplicate local identity rejection.
  - `cargo test --manifest-path adl/Cargo.toml real_pr_init_repairs_missing_version_metadata_on_github_issue -- --nocapture`
    - verifies bootstrap/init repairs missing GitHub version metadata.
  - `bash adl/tools/test_check_issue_metadata_parity.sh`
    - verifies the new metadata audit/check surface detects drift and passes after correction.
  - `bash -n adl/tools/check_issue_metadata_parity.sh adl/tools/test_check_issue_metadata_parity.sh`
    - verifies shell syntax for the new audit scripts.
  - `cargo fmt --manifest-path adl/Cargo.toml --all -- --check`
    - verifies formatting compliance.
  - `git diff --check`
    - verifies no whitespace or patch-format regressions remain.
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
      - "cargo test --manifest-path adl/Cargo.toml normalize_issue_title_for_version_adds_or_replaces_prefix -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml ensure_no_duplicate_issue_identities_rejects_duplicate_prompt_or_task_bundle -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml real_pr_init_repairs_missing_version_metadata_on_github_issue -- --nocapture"
      - "bash adl/tools/test_check_issue_metadata_parity.sh"
      - "bash -n adl/tools/check_issue_metadata_parity.sh adl/tools/test_check_issue_metadata_parity.sh"
      - "cargo fmt --manifest-path adl/Cargo.toml --all -- --check"
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
- Determinism tests executed: title normalization unit coverage, duplicate-identity rejection coverage, and a stateful init fixture that deterministically repairs missing GitHub metadata
- Fixtures or scripts used: Rust inline tests under `adl/src/cli/tests/pr_cmd_inline/` plus `adl/tools/test_check_issue_metadata_parity.sh`
- Replay verification (same inputs -> same artifacts/order): confirmed for the targeted test fixtures; the same fake-GitHub metadata inputs produce the same normalized title, label repair, and duplicate-identity verdicts
- Ordering guarantees (sorting / tie-break rules used): duplicate local identities are sorted before rendering the rejection message, and the audit script scans canonical prompts in sorted order
- Artifact stability notes: the new audit/check surface is deterministic for identical local prompt sets and GitHub metadata responses

## Security / Privacy Checks
- Secret leakage scan performed: manual review of touched diffs and fixture scripts; no secrets or credential material were introduced
- Prompt / tool argument redaction verified: yes; the new scripts and tests operate only on issue titles, labels, and local prompt metadata
- Absolute path leakage check: passed via review of the final SOR and audit script outputs; only repository-relative paths are recorded in the issue artifacts
- Sandbox / policy invariants preserved: yes; the change only edits GitHub issue metadata through bounded `gh issue edit` operations and local deterministic audit logic

## Replay Artifacts
- Trace bundle path(s): not applicable; no ADL runtime trace bundle was produced for this control-plane tooling issue
- Run artifact root: not applicable; validation used repository-local tests and audit scripts only
- Replay command used for verification: reran the targeted Rust tests and `adl/tools/test_check_issue_metadata_parity.sh`
- Replay result: PASS

## Artifact Verification
- Primary proof surface: the touched control-plane sources plus `adl/tools/check_issue_metadata_parity.sh` and its test fixture
- Required artifacts present: yes; all named code, test, and audit-script surfaces are present in the issue worktree
- Artifact schema/version checks: the canonical source prompt/STP and completed SOR remain structurally valid for the v0.87.1 issue flow
- Hash/byte-stability checks: not run; the issue relies on deterministic targeted tests and audit-script validation rather than artifact hashing
- Missing/optional artifacts and rationale: no demo artifact is required because this is a metadata/control-plane correctness issue

## Decisions / Deviations
- Closed GitHub issue `#1604` separately as duplicate/stale because current main already contains the broader canonical ignored `.adl` publication fix from `#1592` / PR `#1599`.

## Follow-ups / Deferred work
- If future tracker hygiene needs broader retroactive repair, use the new audit/check surface to identify remaining drift and schedule bounded follow-on fixes rather than widening this issue.
