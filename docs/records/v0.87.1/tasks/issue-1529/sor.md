# v0-87-1-tools-make-pr-init-and-create-seed-pre-run-sips-without-branch-or-worktree-assumptions

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1529
Run ID: issue-1529
Version: v0.87.1
Title: [v0.87.1][tools] Make pr init/create seed pre-run SIPs without branch/worktree assumptions
Branch: codex/1529-v0-87-1-tools-make-pr-init-and-create-seed-pre-run-sips-without-branch-or-worktree-assumptions
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5
- Provider: openai
- Start Time: 2026-04-10T01:00:00Z
- End Time: 2026-04-10T01:44:03Z

## Summary

Fixed the `pr init`/`pr create` card bootstrap path so pre-run task bundles no longer imply that a branch or worktree already exists. The generic SIP template now defaults to truthful pre-run language, root init/create bundles stamp `Branch: not bound yet`, and run-bound `pr run`/legacy `pr start` paths still rewrite generated SIPs to execution wording when a concrete branch is bound.

## Artifacts produced
- Updated pre-run SIP template: `adl/templates/cards/input_card_template.md`
- Updated Rust PR lifecycle generation and validation code: `adl/src/cli/pr_cmd.rs`, `adl/src/cli/pr_cmd_cards.rs`, `adl/src/cli/tooling_cmd/structured_prompt.rs`
- Updated shell compatibility card generation: `adl/tools/pr.sh`
- Updated lifecycle and validator tests: `adl/src/cli/tests/pr_cmd_inline/lifecycle.rs`, `adl/src/cli/tooling_cmd/tests.rs`, `adl/tools/test_pr_init.sh`, `adl/tools/test_pr_issue_version_inference.sh`
- Updated SIP schema description to reference the current `pr run` execution binding flow: `adl/schemas/structured_implementation_prompt.contract.yaml`

## Actions taken
- Changed root task-bundle bootstrap to use `Branch: not bound yet` for init/create instead of precomputing a future `codex/...` branch.
- Made the input-card template pre-run by default and added a lifecycle rewrite in Rust and shell generation so concrete branches still get run-bound execution language.
- Allowed bootstrap SOR validation to accept `Branch: not bound yet`, matching existing bootstrap SIP validation behavior.
- Tightened shell and Rust regression coverage for unbound init cards, run-bound start cards, pre-run doctor readiness, and bootstrap validator behavior.
- Adjusted the version-inference shell fixture to use authored source prompts before start, keeping that test focused on version/path inference rather than bootstrap-stub rejection.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths from this issue are present on main via merged PR #1532.
- Worktree-only paths remaining: none for required tracked artifacts; issue branch changes have merged to main via PR #1532.
- Integration state: merged
- Verification scope: worktree
- Integration method used: issue branch/worktree changes were published and merged via PR #1532.
- Verification performed:
  - `git status --short`
  - `git diff --name-only`
  - targeted Rust and shell validation commands listed below
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `bash -n adl/tools/pr.sh` verified the updated shell lifecycle helpers remain syntactically valid.
  - `cargo fmt --manifest-path adl/Cargo.toml --check` verified Rust formatting after lifecycle and validator edits.
  - `cargo test --manifest-path adl/Cargo.toml structured_prompt -- --nocapture` verified structured prompt validator behavior, including bootstrap `not bound yet` SIP/SOR handling.
  - `bash adl/tools/test_pr_init.sh` verified init-created SIPs stay pre-run/unbound and do not claim a branch/worktree exists.
  - `bash adl/tools/test_pr_issue_version_inference.sh` verified init remains pre-run while start creates run-bound worktree cards for inferred versions.
  - `cargo test --manifest-path adl/Cargo.toml pr_cmd -- --nocapture` verified the PR command lifecycle suite across init/create/doctor/start/ready/finish behavior.
  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase completed --input .adl/v0.87.1/tasks/issue-1529__v0-87-1-tools-make-pr-init-and-create-seed-pre-run-sips-without-branch-or-worktree-assumptions/sor.md` verified this output record satisfies the completed SOR contract.
  - `git diff --check` verified the tracked diff has no whitespace errors.
  - `rg -n "<local-host-path-patterns>" <changed paths and SOR> || true` verified the final changed surfaces and SOR do not contain accidental local absolute path leakage.
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check` verified formatting through the repo-native finish flow.
  - `cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings` verified the implementation against warning-deny lint policy through the repo-native finish flow.
  - `cargo test --manifest-path adl/Cargo.toml` verified the full Rust test suite through the repo-native finish flow.
- Results:
  - PASS after fixing the discovered bootstrap SOR validator gap.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "bash -n adl/tools/pr.sh"
      - "cargo fmt --manifest-path adl/Cargo.toml --check"
      - "cargo test --manifest-path adl/Cargo.toml structured_prompt -- --nocapture"
      - "bash adl/tools/test_pr_init.sh"
      - "bash adl/tools/test_pr_issue_version_inference.sh"
      - "cargo test --manifest-path adl/Cargo.toml pr_cmd -- --nocapture"
      - "bash adl/tools/validate_structured_prompt.sh --type sor --phase completed --input .adl/v0.87.1/tasks/issue-1529__v0-87-1-tools-make-pr-init-and-create-seed-pre-run-sips-without-branch-or-worktree-assumptions/sor.md"
      - "git diff --check"
      - "rg -n \"<local-host-path-patterns>\" <changed paths and SOR> || true"
      - "cargo fmt --manifest-path adl/Cargo.toml --all --check"
      - "cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings"
      - "cargo test --manifest-path adl/Cargo.toml"
  determinism:
    status: PARTIAL
    replay_verified: false
    ordering_guarantees_verified: true
    notes: deterministic lifecycle/card generation paths are covered by repeatable unit and shell fixtures; no separate replay artifact was generated for this tooling-only fix
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
      approved: true
```

## Determinism Evidence
- Determinism tests executed: `cargo test --manifest-path adl/Cargo.toml pr_cmd -- --nocapture`, `cargo test --manifest-path adl/Cargo.toml structured_prompt -- --nocapture`, `cargo test --manifest-path adl/Cargo.toml`, `bash adl/tools/test_pr_init.sh`, and `bash adl/tools/test_pr_issue_version_inference.sh`.
- Fixtures or scripts used: Rust inline lifecycle fixtures and shell temp-repo fixtures for init/create/start behavior.
- Replay verification (same inputs -> same artifacts/order): not separately replayed; this is a tooling lifecycle fix without a replay artifact requirement.
- Ordering guarantees (sorting / tie-break rules used): not changed; tests verify stable generated card fields and stable lifecycle classification for identical fixture state.
- Artifact stability notes: generated cards now encode lifecycle state from the explicit branch binding input instead of inferring a future worktree during pre-run bootstrap.

## Security / Privacy Checks
- Secret leakage scan performed: no secrets or credential surfaces were added; changes are limited to templates, lifecycle generation, validators, and tests.
- Prompt / tool argument redaction verified: no prompt/tool-argument capture or trace emission behavior was changed.
- Absolute path leakage check: final recorded artifact references are repository-relative.
- Sandbox / policy invariants preserved: no destructive git commands were used; edits stayed in the issue worktree and did not modify `main`.

## Replay Artifacts
- Trace bundle path(s): not applicable; no trace-producing runtime or demo was executed for this tooling lifecycle fix.
- Run artifact root: not applicable; validation used Rust and shell test fixtures only.
- Replay command used for verification: not applicable; no replay artifact was required.
- Replay result: not applicable; deterministic fixture coverage is recorded above.

## Artifact Verification
- Primary proof surface: generated card lifecycle behavior covered by `cargo test --manifest-path adl/Cargo.toml pr_cmd -- --nocapture` and the two shell tests.
- Required artifacts present: yes; all changed implementation, template, validator, schema, and test paths are present in the branch.
- Artifact schema/version checks: bootstrap SOR/SIP validation accepts `not bound yet` only in bootstrap phase; completed/non-bootstrap records still require concrete `codex/...` branches.
- Hash/byte-stability checks: not run; not required for this tooling lifecycle fix.
- Missing/optional artifacts and rationale: no demo or replay artifact was required because the issue is a PR-card lifecycle tooling defect.

## Decisions / Deviations
- Kept pre-run `not bound yet` acceptance phase-scoped to bootstrap validation so completed SORs and run-bound SIPs still require concrete branch truth.
- Preserved run-bound SIP wording for concrete branches through generator-side lifecycle rewrites rather than creating a second template.
- Recorded the initial validator gap as fixed and reran the affected validation to PASS before finish.

## Follow-ups / Deferred work
- None.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
