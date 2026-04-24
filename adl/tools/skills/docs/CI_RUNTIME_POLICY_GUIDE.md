# CI Runtime Policy Guide For Operational Skills

## Purpose

This guide explains how ADL operational skills should interpret PR validation
after the v0.90.4 changed-path CI policy introduced stable check names, explicit
full-coverage policy surfaces, PR-fast Rust validation, and truthful
skip/defer behavior.

The source milestone policy is:

- `docs/milestones/v0.90.4/CI_RUNTIME_POLICY_v0.90.4.md`
- `docs/milestones/v0.90.4/RUST_VALIDATION_ACCELERATION_v0.90.4.md`

## Core Rule

`adl-ci` and `adl-coverage` remain stable required PR check names, but a green
check does not always mean the same validation lane ran.

Skills must distinguish:

- `docs_only_path_policy_skip`: docs, planning, or non-runtime tooling changed;
  expensive Rust, demo smoke, and coverage phases were explicitly skipped
- `runtime_full_validation`: runtime, source, test, or demo-affecting surfaces
  changed on a full-evidence event; Rust validation, demo smoke where required,
  and full coverage gates ran
- `runtime_pr_fast_validation`: runtime, source, test, or demo-affecting
  surfaces changed on a pull request; Rust fmt, clippy, normal tests, demo
  smoke where required, and coverage-impact preflight ran, while full
  instrumented coverage was deferred to avoid a duplicate full test universe
- `failed_closed_full_validation`: changed-path classification failed or was
  ambiguous; CI must require full validation instead of granting a waiver
- `release_or_main_full_validation`: pushes to `main` and release evidence gates
  require full validation regardless of PR-level skip behavior

## Path-Policy Evidence

When a skill needs to interpret a stable check, inspect the
`Classify changed paths` step in `adl-ci` or `adl-coverage`.

The classifier is:

```bash
adl/tools/ci_path_policy.sh
```

It emits:

- `rust_required`
- `coverage_required`
- `full_coverage_required`
- `demo_smoke_required`
- `fail_closed`
- `coverage_lane`
- `coverage_authority`
- `changed_count`
- `reason`

The stable interpretation order is:

1. `coverage_lane`
2. `coverage_authority`
3. `reason`

`reason` remains the human-facing explanation, while `coverage_lane` and
`coverage_authority` make the event/risk split machine-readable.

## Skill Interpretation Rules

Use these rules when choosing validation, publishing PR truth, monitoring CI,
or assembling release evidence:

- A docs-only `adl-coverage` path-policy skip can be healthy for a PR, but it
  is not release coverage evidence.
- A runtime/source/test/demo-affecting PR should not skip Rust validation,
  demo smoke when required, or the PR coverage-impact preflight.
- Ordinary Rust source additions and edits should run the coverage-impact
  preflight before publication. On normal PRs, this is the fast
  `adl-coverage` lane rather than a second full instrumented test run.
- PRs that change explicit coverage-governance surfaces should run full
  authoritative coverage behavior even when they are otherwise tooling-focused.
  The trigger is policy-surface based, not size- or novelty-based.
- When `full_coverage_required=true`, the JSON summary and changed-source
  impact gate should execute before LCOV artifact generation. A coverage
  failure should fail at the first reviewable policy gate instead of spending
  extra time producing upload artifacts for a known-bad run.
- `fail_closed=true` means full validation is required; it is not an approved
  skip.
- If the stable check result and changed files disagree, treat the PR as
  blocked or action-required until `pr-janitor` or a human resolves the
  discrepancy.
- Do not claim "full coverage passed" unless the full coverage-required lane ran
  and produced coverage artifacts such as `coverage-summary.json`.
- Do not treat a green `adl-coverage` check as sufficient release evidence
  unless the evidence shows the full coverage lane ran.

## Local Validation Selection Before PR

The CI path-policy classifier does not mean every issue should run the full
local Rust cycle before publication.

Skills should classify the changed surface first:

- `docs-only`
- `milestone-package-truth`
- `workflow-docs`
- `tooling-focused`
- `rust-focused`
- `demo-focused`
- `review-remediation`
- `release-tail`

Expected local-validation posture:

- `docs-only`, `milestone-package-truth`, and most `workflow-docs` work use
  docs, path, contract, Markdown, and guardrail checks rather than local Rust
  fmt, clippy, tests, or coverage
- `tooling-focused` work uses the smallest shell, unit, or contract check that
  proves the changed behavior
- `rust-focused` work uses targeted Rust validation and widens only when the
  changed module or contract boundary is broad
- `demo-focused` work runs the named demo or smallest proving proof surface
- `review-remediation` work reruns the narrow validation that proves the named
  finding is fixed
- `release-tail` work uses tracker, gap, closeout, review-truth, path, and
  evidence checks unless tracked code changed

This is the main compression rule:

- do not use the full local test cycle as the default for every issue
- do use the smallest truthful local validation that proves the changed surface
- widen only when the issue, ambiguity, or failed checks justify it

When recording PR or output-card truth, say both:

- what validation class ran
- what larger validation did not run and why that was acceptable

## Examples

## Build Acceleration Posture

Current CI posture also includes bounded build acceleration:

- `sccache` is installed in Rust lanes and wired through `RUSTC_WRAPPER`
- `~/.cache/sccache` is persisted through the existing Rust cache action
- `lld` is installed and asserted on GitHub-hosted Linux runners before Rust
  acceleration is configured
- CI logs emit `sccache --show-stats` so operators can tell whether compiler
  output reuse is actually happening

Skills should treat this as throughput infrastructure, not proof by itself:

- a green run still needs the correct validation lane
- cache hits are supporting evidence about efficiency, not correctness
- missing `lld` after the install step is a CI failure because the workflow is
  explicitly claiming linker acceleration on the hosted runner path

### Docs-Only PR

Observed:

- `adl-ci`: pass
- `adl-coverage`: pass with explicit path-policy skip
- `reason`: changed paths are docs/planning/non-runtime tooling only

Truthful interpretation:

- PR CI is healthy for the changed surface.
- Rust coverage did not run and must not be cited as release coverage.

### Runtime PR

Observed:

- `rust_required=true`
- `coverage_required=true`
- `full_coverage_required=false`
- `coverage_lane=pr_fast`
- `coverage_authority=pr_changed_surface`
- `demo_smoke_required=true` when demo surfaces changed

Truthful interpretation:

- Rust fmt, clippy, doc tests via `cargo test --doc`, demo smoke where
  applicable, and the fast coverage-impact preflight are expected.
- The non-coverage `test` step now runs through
  `adl/tools/run_pr_fast_test_lane.sh`. For bounded PRs, that runner may use a
  focused `cargo nextest` expression. For broad or ambiguous PRs, it fails
  closed to the full ordinary nextest sweep.
- The ordinary PR lane must not re-enable heavyweight opt-in features such as
  `slow-proof-tests`; those stay reserved for dedicated heavy proof or
  authoritative coverage lanes.
- Full instrumented coverage is intentionally deferred for the PR to avoid
  running all tests twice.
- The PR does not itself provide full release coverage evidence.

### Full-Evidence Runtime Event Or Policy-Surface PR

Observed:

- `rust_required=true`
- `coverage_required=true`
- `full_coverage_required=true`
- `coverage_lane=authoritative_full`
- `coverage_authority=push_main`, `pr_policy_surface`, or another
  authoritative trigger

Truthful interpretation:

- Standalone `cargo test` may be skipped because the full `cargo llvm-cov`
  lane is the authoritative full test execution for that event.
- A lightweight `cargo test --doc` check may still run to preserve doc-test
  coverage without duplicating the whole suite.
- Full coverage artifacts and policy gates are expected.
- This lane can be cited as full coverage evidence when it produces
  `coverage-summary.json`, `coverage-summary.txt`, and `lcov.info`.

For a policy-surface PR, `rust_required` and `demo_smoke_required` may stay
false if the changed paths are tooling-only, but `full_coverage_required=true`
still means the authoritative base coverage lane must run because the PR is
modifying coverage governance itself.

The authoritative coverage implementation is now explicit:

```bash
adl/tools/run_authoritative_coverage_lane.sh
```

That runner performs one pass per event:

- `full_authoritative_all_features` on `main` and other full-evidence events
- `bounded_policy_surface_pr` on policy-surface pull requests

and then emits one coverage summary report. Skills should treat
`bounded_policy_surface_pr` as bounded PR governance validation, not as full
release evidence.

### Failed-Closed Classification

Observed:

- `fail_closed=true`
- `coverage_lane=authoritative_full`
- `coverage_authority=fail_closed`

Truthful interpretation:

- CI should require full validation.
- Skills must not waive validation on the grounds that the path classifier was
  uncertain.

### Release Evidence

Observed:

- Several docs-only PRs have green `adl-coverage` checks with skip messages.

Truthful interpretation:

- Those checks support PR-level readiness for docs-only changes.
- They do not satisfy release coverage gates. Release evidence needs full
  validation from `main`, release ceremony, fail-closed full-validation events,
  or explicit coverage artifacts produced by a full coverage lane.
