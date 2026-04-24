# CI Runtime Policy - v0.90.4

## Status

Landed through tooling issue #2463.

This note is for ADL execution sessions working through the active v0.90.4
issue wave. The goal is to keep PR checks truthful without making ordinary Rust
work wait on full-workspace `cargo llvm-cov` unless the change touches explicit
coverage-governance surfaces or lands on an authoritative full-evidence event.

## Why This Changed

The previous PR coverage posture still allowed bounded runtime work to fall into
the full `cargo llvm-cov --workspace --all-features` lane too often. In
practice, that meant routine WPs could spend 25 to 35 minutes waiting on a
coverage job that was meant for release governance, not every issue branch.

The v0.90.4 policy keeps the stable check names, keeps PR validation truthful,
and makes the full-coverage trigger explicit and reviewable:

- ordinary runtime/test PRs run the fast PR validation path
- coverage-policy-sensitive PRs still fail closed into the full coverage lane
- pushes to `main`, nightly automation, and other non-PR events still run the
  authoritative full coverage workflow

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
- `coverage_lane`
- `coverage_authority`
- `changed_count`
- `reason`

For non-PR events, including pushes to `main`, it runs full validation.

Interpretation order:

1. `coverage_lane`
2. `coverage_authority`
3. `reason`

`reason` remains the human-facing explanation, while `coverage_lane` and
`coverage_authority` make the authoritative-vs-fast coverage split explicit for
skills and operators.

If the diff cannot be determined, the policy fails closed and runs full
validation.

## Explicit Full-Coverage Triggers

PRs require the full coverage lane only when they change explicit
coverage-governance surfaces:

- `.github/workflows/ci.yaml`
- `.github/workflows/nightly-coverage-ratchet.yaml`
- `adl/tools/ci_path_policy.sh`
- `adl/tools/check_coverage_impact.sh`
- `adl/tools/enforce_coverage_gates.sh`

This is intentional. The rule is reviewable, explainable, and easy to audit. We
do not silently escalate ordinary runtime work into full coverage because a file
is large, newly added, or otherwise looks "risky" by heuristic alone.

## What Runs

Docs/planning/tools-only PRs:

- run tooling sanity checks
- run issue-linkage and legacy-reference guardrails
- run operational-skill contract checks
- run docs command checks
- skip Rust fmt, clippy, full tests, demo smoke, and full coverage with an
  explicit path-policy message

Ordinary runtime/source PRs:

- run tooling sanity checks
- run guardrails and contract checks
- run Rust fmt, clippy, and full tests
- keep heavyweight `runtime_v2` proof-materialization tests and the explicit
  CLI proof-smoke stdout test behind the `slow-proof-tests` feature so the
  default `cargo test` lane stays bounded
- run demo smoke when required
- run the changed-source coverage-impact preflight in the stable
  `adl-coverage` check
- defer full instrumented coverage to main, nightly, and explicit
  coverage-policy-sensitive PRs

Coverage-policy-sensitive PRs:

- run the same base validation as other relevant PRs
- run full coverage because the PR changes the rules or enforcement surfaces
  that govern coverage itself

Pushes to `main` and nightly coverage:

- run full validation and full coverage
- avoid a second standalone full `cargo test` when the full coverage lane is
  already executing the Rust test suite

## Coverage Behavior

Coverage-impact preflight still runs for Rust/runtime PR changes. The workflow
does not run a second full instrumented test universe for ordinary PRs.

The heavyweight `runtime_v2` proof-materialization tranche is intentionally
classified separately from always-on contract checks:

- default `adl-ci` runs `cargo test` without `slow-proof-tests`
- authoritative `cargo llvm-cov --workspace --all-features` lanes still execute
  that tranche

That keeps ordinary PR validation fast without pretending those proof surfaces
no longer matter.

When `full_coverage_required=true`, full coverage generates:

- `lcov.info`
- `coverage-summary.json`
- `coverage-summary.txt`

from the coverage data produced by one coverage run.

## Session Guidance

When working a normal runtime PR, expect Rust fmt, clippy, normal tests, demo
smoke when required, and coverage-impact preflight. Do not cite the PR-fast
coverage lane as full release coverage evidence.

When working a PR that changes coverage governance or coverage tooling, expect
the full coverage lane. That slower path is intentional because the PR is
changing how coverage trust is enforced.

When working a `main`, nightly, release, or fail-closed event, expect full
coverage. In those lanes, standalone `cargo test` may be skipped because
`cargo llvm-cov` is the authoritative full test execution. A lightweight
`cargo test --doc` check may still run to preserve doc-test coverage without
duplicating the whole suite.

When in doubt, check the `Classify changed paths` step in `adl-ci` or
`adl-coverage`. Read `coverage_lane` first, `coverage_authority` second, and
then use `reason` as the human-readable explanation for why a lane did or did
not run.

## Non-Claims

This policy does not lower coverage thresholds or weaken release governance. It
keeps ordinary bounded Rust PRs on the fast truthful path while preserving full
coverage for `main`, nightly, release, fail-closed events, and PRs that modify
coverage governance itself.
