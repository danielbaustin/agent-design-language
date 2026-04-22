# CI Runtime Policy - v0.90.3

## Status

Landed through tooling issue #2392.

This note is for all ADL execution sessions working through the v0.90.3
closeout tail. The goal is to keep PR checks truthful without forcing every
docs-heavy or planning-heavy PR to wait for full Rust coverage, and without
making runtime PRs run the same full Rust test universe twice.

## Why This Changed

The previous PR workflow ran full `cargo test` and full `cargo llvm-cov` on
every pull request. That was appropriate when most PRs touched runtime code, but
it became a blocker as v0.90.3 entered documentation, demo-matrix, review, and
release-evidence work.

The new workflow keeps the check names stable while using changed-path policy
to decide whether expensive Rust phases are needed. For normal runtime PRs, the
workflow now runs one full normal Rust test pass plus a fast changed-source
coverage-impact preflight. Full instrumented coverage remains the authoritative
main/nightly/release evidence lane instead of duplicating every PR test run.

## Stable Check Names

The required PR checks remain:

- `adl-ci`
- `adl-coverage`

These jobs still appear on docs-only PRs. When expensive validation is not
needed, the job records an explicit skip message instead of disappearing.

## Path Policy

The shared classifier is:

```bash
adl/tools/ci_path_policy.sh
```

For pull requests, it compares the PR base and head SHAs and emits:

- `rust_required`
- `coverage_required`
- `full_coverage_required`
- `demo_smoke_required`
- `fail_closed`
- `changed_count`
- `reason`

For non-PR events, including pushes to `main`, it runs full validation.

If the diff cannot be determined, the policy fails closed and runs full
validation.

## What Runs

Docs/planning/tools-only PRs:

- run tooling sanity checks
- run issue-linkage and legacy-reference guardrails
- run operational-skill contract checks
- run docs command checks
- skip Rust fmt, clippy, full tests, demo smoke, and full coverage with an
  explicit path-policy message

Runtime/source PRs:

- run tooling sanity checks
- run guardrails and contract checks
- run Rust fmt, clippy, and full tests
- run demo smoke
- run the changed-source coverage-impact preflight in the stable
  `adl-coverage` check
- defer full instrumented coverage to main/nightly/release evidence unless the
  path classifier fails closed

Pushes to `main`:

- run full validation and full coverage
- avoid a second standalone full `cargo test` when the full coverage lane is
  already executing the Rust test suite

## Coverage Behavior

Coverage-impact preflight still runs for Rust/runtime PR changes. The workflow
does not run a second full instrumented test universe for ordinary PRs.

When `full_coverage_required=true`, full coverage generates:

- `lcov.info`
- `coverage-summary.json`
- `coverage-summary.txt`

from the coverage data produced by one coverage run.

## Session Guidance

When working a docs-heavy closeout PR, do not panic if `adl-coverage` completes
quickly with a skip message. That is expected when the PR does not touch Rust
runtime or test surfaces.

When working a runtime PR, expect Rust fmt, clippy, normal tests, demo smoke
when required, and coverage-impact preflight. Do not cite the PR-fast coverage
lane as full release coverage evidence.

When working a main, nightly, release, or fail-closed event, expect full
coverage. In those lanes, standalone `cargo test` may be skipped because
`cargo llvm-cov` is the authoritative full test execution. A lightweight
`cargo test --doc` check may still run to preserve doc-test coverage without
duplicating the whole suite.

When in doubt, check the `Classify changed paths` step in `adl-ci` or
`adl-coverage`. Its `reason` field explains why a lane did or did not run.

## Non-Claims

This policy does not remove release coverage gates. It avoids duplicate full
test execution on PRs while preserving release coverage gates on main, nightly,
release, and fail-closed full-validation surfaces. Runtime/source PRs still
retain strong validation through normal Rust tests plus coverage-impact
preflight; they simply no longer run the whole test suite a second time under
coverage instrumentation before merge.
