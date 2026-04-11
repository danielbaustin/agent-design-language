# v0-87-1-tools-auto-attach-pr-janitor-on-pr-open

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1595
Run ID: issue-1595
Version: v0.87.1
Title: [v0.87.1][tools] Auto-attach PR janitor on PR open
Branch: codex/1595-v0-87-1-tools-auto-attach-pr-janitor-on-pr-open
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5
- Provider: openai
- Start Time: 2026-04-11T16:45:00Z
- End Time: 2026-04-11T17:52:47Z

## Summary
Added a repo-native janitor auto-attach hook to the finish path so a concrete `pr-janitor` run is launched as soon as PR publication succeeds, with explicit blocking behavior if attachment fails.

## Artifacts produced
- `adl/src/cli/pr_cmd.rs`
- `adl/src/cli/pr_cmd/github.rs`
- `adl/src/cli/tests/pr_cmd_inline/finish.rs`
- `adl/src/cli/tests/pr_cmd_inline/mod.rs`
- `adl/tools/attach_pr_janitor.sh`
- `adl/tools/skills/pr-finish/SKILL.md`
- `adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`

## Actions taken
- Added `attach_pr_janitor(...)` to the Rust GitHub helper layer and invoked it from `real_pr_finish(...)` after PR publication / ready-state handling.
- Added `adl/tools/attach_pr_janitor.sh` as the concrete launcher that installs the operational skills, writes a validated janitor payload, and starts one bounded Codex janitor pass in the background.
- Hardened finish-path tests to verify successful janitor attachment and explicit failure when auto-attach cannot start.
- Updated the finish/janitor docs to reflect that repo-native finish now auto-attaches the in-flight PR janitor hook.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `.adl/v0.87.1/tasks/issue-1595__v0-87-1-tools-auto-attach-pr-janitor-on-pr-open/sor.md`
- Worktree-only paths remaining: none
- Integration state: merged
- Verification scope: main_repo
- Integration method used: normalized the canonical root SOR directly on `main` after verifying the issue is already closed and linked to merged PR `#1603`
- Verification performed:
  - `gh issue view 1595 --json title,url,state,stateReason,closedByPullRequestsReferences`
    - verified the issue is closed and captured the final closure metadata used for this normalization pass
  - `gh pr view 1603 --json state,url`
    - verified the linked closing PR remains available as the final publication surface
  - `ls .adl/v0.87.1/tasks/issue-1595__v0-87-1-tools-auto-attach-pr-janitor-on-pr-open/sor.md`
    - verified the canonical root SOR path exists on the main repository path
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `cargo test --manifest-path adl/Cargo.toml real_pr_finish -- --nocapture`
    - verified the finish path still creates/updates PRs correctly, auto-attaches janitor on success, and fails explicitly when janitor attachment cannot start.
  - `git diff --check`
    - verified the final diff is clean and publication-safe.
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
      - "cargo test --manifest-path adl/Cargo.toml real_pr_finish -- --nocapture"
      - "git diff --check"
  determinism:
    status: PASS
    replay_verified: not_applicable
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
- Determinism tests executed: repeated finish-path fixture tests through `real_pr_finish`.
- Fixtures or scripts used: Rust inline `pr_cmd` finish fixtures plus the deterministic janitor launcher command contract.
- Replay verification (same inputs -> same artifacts/order): not applicable for background janitor process startup beyond the bounded fixture assertions.
- Ordering guarantees (sorting / tie-break rules used): finish publishes the PR before janitor attach and blocks on attach failure, preserving stable lifecycle ordering.
- Artifact stability notes: the janitor launcher writes one deterministic payload/log location rooted under `.adl/logs/pr-janitor/issue-<n>/`.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual inspection of the new launcher payload/log path handling plus test review for secret-free arguments.
- Prompt / tool argument redaction verified: yes; the launcher passes only issue/branch/PR metadata and expected checks/policy fields.
- Absolute path leakage check: output record uses repository-relative paths only.
- Sandbox / policy invariants preserved: yes; the launcher uses repo-local skill install plus `codex exec --sandbox workspace-write`.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable for this tooling issue.
- Run artifact root: `.adl/logs/pr-janitor/issue-<n>/` when janitor auto-attach runs.
- Replay command used for verification: `cargo test --manifest-path adl/Cargo.toml real_pr_finish -- --nocapture`
- Replay result: PASS for bounded finish-path fixtures.

## Artifact Verification
- Primary proof surface: `adl/tools/attach_pr_janitor.sh` plus the finish-path Rust tests.
- Required artifacts present: true
- Artifact schema/version checks: existing finish-path contracts and skill docs remained structurally valid.
- Hash/byte-stability checks: not performed; this change is validated by deterministic fixture behavior rather than artifact hashing.
- Missing/optional artifacts and rationale: no demo artifact or runtime trace bundle is required for this workflow-tooling issue.

## Decisions / Deviations
- Implemented the first concrete janitor attachment behavior as a repo-native launcher hook rather than pretending the Rust CLI can directly spawn desktop-only subagents.

## Follow-ups / Deferred work
- `#1596` and `#1597` continue the same milestone-compression wave with automatic closeout and finish-time milestone-doc drift checks.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
