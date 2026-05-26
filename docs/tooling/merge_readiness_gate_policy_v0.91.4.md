# v0.91.4 Merge-Readiness Gate Policy

## Purpose

Define the bounded validation-policy surface for `WP-07` so reviewers can see
when `pr finish` is allowed to use docs-only validation, when it may use the
focused local-CI-gated profile, and when it must fall back to broad Rust
validation.

## Validation Profiles

### `DocsOnly`

Use the docs-only validation profile when every changed tracked path is a docs
surface and no code/tooling behavior changed.

Current contract:

- paths under `docs/`
- root Markdown policy docs such as `AGENTS.md`

This profile proves:

- tracked docs remain patch-clean
- no broad Rust/tooling claim is being made

This profile does not prove:

- remote CI status
- branch-protection readiness
- merge permission

### `FocusedLocalCiGated`

Use the focused local-CI-gated profile when every changed tracked path is one
of the explicitly allowed workflow surfaces:

- `.github/workflows/ci.yaml`
- `adl/src/cli/pr_cmd.rs`
- `adl/src/cli/pr_cmd/`
- `adl/src/cli/tests/pr_cmd_inline/finish/`
- `adl/tools/check_coverage_impact.sh`
- `adl/tools/test_check_coverage_impact.sh`
- `adl/tools/ci_path_policy.sh`
- `adl/tools/test_ci_path_policy.sh`
- known validation-profile docs

This profile proves:

- the issue stayed inside the PR/doctor/gate policy surface
- the focused `cli::pr_cmd` Rust test slice still passes
- coverage-impact / CI-path policy helpers still pass when those helpers change

This profile does not prove:

- repository-wide Rust health
- remote GitHub check success
- merge truth by local test success alone

### `FullRust`

Use the full Rust profile for everything else. This is the safe fallback when a
change leaves the bounded PR-gate surface.

## Merge-Truth Boundary

Local focused validation is allowed to prove gate logic, validator behavior, and
card-truth blocking rules. It is not allowed to imply that a PR is merged,
reviewed, or green on GitHub unless that state is separately recorded from
GitHub truth.

This `WP-07` hardening lands the policy boundary and focused validation posture.
It does not introduce a new live GitHub-state reconciliation engine by itself.

Human review and merge authority remain required.
