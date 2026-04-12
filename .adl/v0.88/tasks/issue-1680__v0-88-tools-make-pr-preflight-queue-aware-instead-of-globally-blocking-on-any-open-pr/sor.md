# v0-88-tools-make-pr-preflight-queue-aware-instead-of-globally-blocking-on-any-open-pr

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1680
Run ID: issue-1680
Version: v0.88
Title: [v0.88][tools] Make PR preflight queue-aware instead of globally blocking on any open PR
Branch: codex/1680-v0-88-tools-make-pr-preflight-queue-aware-instead-of-globally-blocking-on-any-open-pr
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5-codex
- Provider: OpenAI Codex desktop
- Start Time: 2026-04-12T19:20:00Z
- End Time: 2026-04-12T19:50:31Z

## Summary
Implemented queue-aware milestone preflight for PR workflow execution. The control plane now resolves a target queue from canonical issue metadata or truthful inference, blocks only on same-queue open PRs, and reports the queue in doctor/preflight output and guard messages. Added queue metadata emission for generated issue prompts, plus regression coverage for same-queue blocking, cross-queue allow, and missing/uninferrable queue failure.

## Artifacts produced
- `adl/src/cli/pr_cmd.rs`
- `adl/src/cli/pr_cmd/doctor.rs`
- `adl/src/cli/pr_cmd/github.rs`
- `adl/src/cli/pr_cmd_cards.rs`
- `adl/src/cli/pr_cmd_prompt.rs`
- `adl/src/cli/tests/pr_cmd_inline/basics.rs`
- `adl/src/cli/tests/pr_cmd_inline/lifecycle.rs`
- `adl/src/cli/tests/pr_cmd_inline/repo_helpers.rs`
- `adl/tools/pr.sh`
- `adl/tools/test_pr_start_worktree_safe.sh`
- updated local issue surfaces for `#1680` with explicit `queue: "tools"`

## Actions taken
- Added workflow-queue inference and prompt-resolution helpers in `pr_cmd_prompt.rs`.
- Added `queue:` emission to generated and mirrored issue prompt front matter.
- Changed `unresolved_milestone_pr_wave(...)` to filter open PR blockers by target queue instead of milestone-only breadth.
- Extended doctor/preflight output to report `TARGET_QUEUE`, `TARGET_QUEUE_SOURCE`, and per-PR queue data.
- Updated start/run guard messaging to describe same-queue blocking truthfully.
- Added focused regression tests for queue inference, same-queue blocking, cross-queue allowance, and missing queue handling.
- Dogfooded the new queue metadata in the `#1680` issue prompt/STP.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated:
- `adl/src/cli/pr_cmd.rs`
- `adl/src/cli/pr_cmd/doctor.rs`
- `adl/src/cli/pr_cmd/github.rs`
- `adl/src/cli/pr_cmd_cards.rs`
- `adl/src/cli/pr_cmd_prompt.rs`
- `adl/src/cli/tests/pr_cmd_inline/basics.rs`
- `adl/src/cli/tests/pr_cmd_inline/lifecycle.rs`
- `adl/src/cli/tests/pr_cmd_inline/repo_helpers.rs`
- `adl/tools/pr.sh`
- `adl/tools/test_pr_start_worktree_safe.sh`
- `.adl/v0.88/bodies/issue-1680-v0-88-tools-make-pr-preflight-queue-aware-instead-of-globally-blocking-on-any-open-pr.md`
- `.adl/v0.88/tasks/issue-1680__v0-88-tools-make-pr-preflight-queue-aware-instead-of-globally-blocking-on-any-open-pr/stp.md`
- Worktree-only paths remaining:
  - none; all required changes exist in the issue branch worktree and are tracked for PR publication, but they are not yet on `main`
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: direct edits in the issue worktree after `pr run` binding
- Verification performed:
  - `git status --short`
  - `git diff --check`
- Result: FAIL
  Explanation: the worktree contains the full required artifact set and is ready for `pr-finish`, but nothing has been integrated into `main` yet, so main-repo integration is not complete at `pr-run` time

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
  - `cargo build --manifest-path adl/Cargo.toml --bin adl` to rebuild the CLI against the queue-aware guard changes
  - `cargo test --manifest-path adl/Cargo.toml infer_workflow_queue_prefers_explicit_signals_and_tags -- --nocapture` to prove queue inference behavior
  - `cargo test --manifest-path adl/Cargo.toml real_pr_preflight_allows_cross_queue_open_prs -- --nocapture` to prove cross-queue preflight allow behavior
  - `cargo test --manifest-path adl/Cargo.toml resolve_issue_prompt_workflow_queue_rejects_missing_and_uninferrable_queue -- --nocapture` to prove truthful failure on missing/uninferrable queue metadata
  - `bash adl/tools/test_pr_start_worktree_safe.sh` to prove the shell wrapper blocks same-queue work and allows cross-queue preflight
  - `adl/tools/pr.sh doctor 1680 --version v0.88 --slug v0-88-tools-make-pr-preflight-queue-aware-instead-of-globally-blocking-on-any-open-pr` to inspect live queue-aware doctor output for the issue itself
  - `git diff --check` to verify patch hygiene
- Results:
  - all listed validations passed
  - live doctor output now reports the issue as `TARGET_QUEUE=tools` and only blocks on a same-queue open PR (`#1682`), which matches the intended policy

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
      - "cargo build --manifest-path adl/Cargo.toml --bin adl"
      - "cargo test --manifest-path adl/Cargo.toml infer_workflow_queue_prefers_explicit_signals_and_tags -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml real_pr_preflight_allows_cross_queue_open_prs -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml resolve_issue_prompt_workflow_queue_rejects_missing_and_uninferrable_queue -- --nocapture"
      - "bash adl/tools/test_pr_start_worktree_safe.sh"
      - "adl/tools/pr.sh doctor 1680 --version v0.88 --slug v0-88-tools-make-pr-preflight-queue-aware-instead-of-globally-blocking-on-any-open-pr"
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
      present: true
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: queue inference helper tests and doctor/preflight regression tests were rerun against stable fake `gh pr list` payloads.
- Fixtures or scripts used: Rust inline lifecycle/repo-helper tests plus `adl/tools/test_pr_start_worktree_safe.sh`.
- Replay verification (same inputs -> same artifacts/order): repeated fake-GitHub inputs produced stable same-queue block vs cross-queue pass outcomes and stable doctor text output.
- Ordering guarantees (sorting / tie-break rules used): blocker selection still relies on deterministic `gh pr list` parsing plus explicit queue classification; no nondeterministic ordering logic was introduced.
- Artifact stability notes: output now includes explicit queue fields, reducing ambiguity rather than adding hidden branching.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: review of changed command output and recorded card content; no secrets or tokens were introduced.
- Prompt / tool argument redaction verified: the change only adds queue metadata and blocker explanations; no prompt/tool-arg surfaces were widened.
- Absolute path leakage check: `git diff --check` passed, and recorded commands in this card use repository-relative paths.
- Sandbox / policy invariants preserved: yes; the change stays within local control-plane logic and test fixtures.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable; this issue changes control-plane gating logic rather than ADL trace artifacts.
- Run artifact root: not applicable.
- Replay command used for verification: targeted Rust tests and the shell wrapper regression script listed in Validation.
- Replay result: pass; deterministic fake-GitHub fixtures reproduced the expected queue-aware outcomes.

## Artifact Verification
- Primary proof surface: queue-aware doctor/start behavior in the Rust lifecycle tests and `adl/tools/test_pr_start_worktree_safe.sh`.
- Required artifacts present: yes; code, tests, help text, and the issue-local queue metadata changes are present in the worktree.
- Artifact schema/version checks: front matter now supports `queue:`; older prompts remain supported through truthful inference when `queue:` is absent.
- Hash/byte-stability checks: not applicable beyond deterministic test expectations; no binary artifact contract changed.
- Missing/optional artifacts and rationale: no standalone demo was required for this tools issue.

## Decisions / Deviations
- Proceeded with `pr run ... --allow-open-pr-wave` because the issue existed specifically to repair the overly coarse open-PR guard, and the old guard was self-blocking the fix.
- Kept the first implementation intentionally narrow: same-queue blocking is solved; protected-surface overlap and release-tail stricter policy remain future follow-on work.
- Target queue resolution is explicit when `queue:` exists and inferred when older prompts omit it; missing and uninferrable queue metadata now fails truthfully.

## Follow-ups / Deferred work
- `workflow-conductor` remains useful as a lightweight router, but today it still depends on a hand-built payload rather than collecting repo state itself. That is a separate follow-on from this queue-aware preflight change.
- Queue-aware blocking does not yet include protected-surface conflict detection across different queues.
- Release-tail stricter concurrency policy is still deferred to later control-plane work.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
