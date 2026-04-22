# CI Runtime Policy - v0.90.3

## Status

Landed through tooling issue #2392.

This note is for all ADL execution sessions working through the v0.90.3
closeout tail. The goal is to keep PR checks truthful without forcing every
docs-heavy or planning-heavy PR to wait for full Rust coverage.

## Why This Changed

The previous PR workflow ran full `cargo test` and full `cargo llvm-cov` on
every pull request. That was appropriate when most PRs touched runtime code, but
it became a blocker as v0.90.3 entered documentation, demo-matrix, review, and
release-evidence work.

The new workflow keeps the check names stable while using changed-path policy
to decide whether expensive Rust phases are needed.

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
- run full coverage and coverage gates

Pushes to `main`:

- run full validation and full coverage

## Coverage Behavior

Coverage still runs for Rust/runtime changes. The workflow now avoids rerunning
the workspace just to print the text summary. It generates:

- `lcov.info`
- `coverage-summary.json`
- `coverage-summary.txt`

from the coverage data produced by one coverage run.

## Session Guidance

When working a docs-heavy closeout PR, do not panic if `adl-coverage` completes
quickly with a skip message. That is expected when the PR does not touch Rust
runtime or test surfaces.

When working a runtime PR, expect full CI cost. Runtime/source changes still
pay for full Rust validation and coverage.

When in doubt, check the `Classify changed paths` step in `adl-ci` or
`adl-coverage`. Its `reason` field explains why a lane did or did not run.

## Non-Claims

This policy does not remove release coverage gates. It only avoids paying for
full coverage on PRs that cannot affect Rust runtime behavior. Release
readiness, `main` pushes, and runtime/source changes still retain strong
validation.
