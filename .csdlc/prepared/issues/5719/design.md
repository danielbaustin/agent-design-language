# #5719 Design

## Goal

Fix the CI path-policy selection that caused podcast/demo page-only PRs to schedule both runtime hosted coverage and workspace hosted coverage. The stable `adl-coverage-hosted` job remains the required aggregator/check; the expensive producer lanes should only run when the path policy proves they are required.

## Scope

- `adl/tools/ci_path_policy.sh`
- `adl/tools/test_ci_path_policy.sh`
- `.github/workflows/ci.yaml` only if the selector contract shows the workflow consumes the corrected output incorrectly
- issue-local C-SDLC records and publication evidence

## Approach

1. Reproduce the #5716-like path set in the existing path-policy contract tests.
2. Add the narrowest path classifier that treats podcast/demo page and launch-packet static surfaces as focused/static validation, not full hosted coverage.
3. Preserve full hosted coverage for Rust, runtime, provider, and tooling policy changes.
4. Run the existing CI path-policy contract tests and focused validation-manager checks.

## Non-Goals

- Do not remove the `adl-coverage-hosted` aggregator.
- Do not weaken full coverage for Rust/runtime/provider/tooling changes.
- Do not edit the podcast page or any active podcast PR in this issue.
