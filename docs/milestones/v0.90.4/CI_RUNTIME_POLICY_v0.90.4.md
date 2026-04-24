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

## Current Cost Center

After the coverage split and slow-proof isolation landed, the ordinary PR path
still had one obvious long pole: the non-coverage `adl-ci` test step.

Observed on successful run `24903573520` for PR `#2517`:

- `adl-ci`: about `13m17s`
- `clippy`: about `66s`
- `test`: about `10m37s`
- `demo smoke`: about `43s`

That run is important because it shows the remaining bottleneck was not the
coverage lane. The ordinary `cargo nextest run` sweep itself was still too
broad for bounded PRs.

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
- run the bounded authoritative base coverage summary because the PR changes
  the rules or enforcement surfaces that govern coverage itself
- defer the proof-heavy authoritative slice and workspace-threshold gate to the
  push-to-main run so policy PRs do not pay the full proof tax twice

Pushes to `main` and nightly coverage:

- run full validation and full coverage
- avoid a second standalone full `cargo test` when the full coverage lane is
  already executing the Rust test suite

## Ordinary PR-Fast Test Runner

The ordinary non-coverage test step now runs through:

```bash
adl/tools/run_pr_fast_test_lane.sh
```

This runner is intentionally conservative:

- it computes the changed surface from the PR base/head SHAs
- it uses a focused `cargo nextest` expression only when every changed fast-lane
  surface maps to a bounded token set
- it fails closed to the full ordinary nextest sweep when the change is broad,
  ambiguous, or touches too many independently-filtered modules

Focused fast-lane cases currently include bounded slices such as:

- individual `runtime_v2` module files
- bounded `runtime_v2` family surfaces such as `runtime_v2/mod.rs`,
  `runtime_v2/tests.rs`, `runtime_v2/validators.rs`, and
  `runtime_v2/governed_episode/*`
- bounded CLI command files
- bounded CLI family surfaces such as `cli/mod.rs`, `cli/tests.rs`,
  `cli/identity_cmd/*`, `cli/tests/internal_commands/*`,
  `cli/tests/artifact_builders/*`, and `cli/tests/run_state/*`
- publication-control-plane docs that intentionally route to the `pr_cmd`
  validation slice

Fail-closed full-lane cases include:

- broad entry surfaces such as `adl/src/lib.rs`, `adl/src/main.rs`,
  `adl/src/runtime_v2/mod.rs`, and `adl/src/schema.rs`
- test-harness and integration surfaces under `adl/tests/`
- unmapped nested source paths
- PRs that would need more than four focused test tokens

The goal is not to guess. The goal is to use a smaller truthful lane when the
changed surface is obvious and bounded, and otherwise keep the prior full
ordinary lane.

## Coverage Behavior

Coverage-impact preflight still runs for Rust/runtime PR changes. The workflow
does not run a second full instrumented test universe for ordinary PRs.

The heavyweight `runtime_v2` proof-materialization tranche is intentionally
classified separately from always-on contract checks:

- default `adl-ci` runs `cargo test` without `slow-proof-tests`
- authoritative `cargo llvm-cov --workspace --all-features` lanes still execute
  that tranche

The authoritative lane is now split internally as well:

- `always_on_authoritative`
  runs the base coverage universe that must remain always-on for release
  governance
- `proof_heavy_authoritative`
  runs the bounded proof-heavy slice and opt-in feature tranches through a
  second targeted `cargo llvm-cov nextest` pass

Both phases accumulate into the same final coverage report on `main` and other
full-evidence events. For `pr_policy_surface` pull requests, the workflow runs
the always-on authoritative base summary plus the changed-source coverage gate,
then defers the proof-heavy phase and workspace-threshold gate to push-to-main.
The split is about runtime stability and explainability, not about dropping
proof surfaces.

That keeps ordinary PR validation fast without pretending those proof surfaces
no longer matter.

When `full_coverage_required=true`, full coverage generates:

- `lcov.info`
- `coverage-summary.json`
- `coverage-summary.txt`

from the coverage data produced by one coverage run.

In implementation terms, the workflow now routes this through:

```bash
adl/tools/run_authoritative_coverage_lane.sh
```

That runner is the machine-readable contract for which authoritative surfaces
stay always-on and which proof-heavy slices are executed in the targeted second
pass.

## Session Guidance

When working a normal runtime PR, expect Rust fmt, clippy, normal tests, demo
smoke when required, and coverage-impact preflight. Do not cite the PR-fast
coverage lane as full release coverage evidence.

When working a PR that changes coverage governance or coverage tooling, expect
the authoritative base coverage lane on the PR and the full proof-heavy lane on
push-to-main. That keeps governance changes reviewable without making every
policy PR pay the full proof-heavy workspace gate before merge.

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

This policy does not lower push-to-main coverage thresholds or weaken release
governance. It keeps ordinary bounded Rust PRs on the fast truthful path,
keeps policy-surface PRs on a bounded authoritative base lane, and preserves
the full proof-heavy/workspace-threshold coverage gate for `main`, nightly,
release, and other full-evidence events.
