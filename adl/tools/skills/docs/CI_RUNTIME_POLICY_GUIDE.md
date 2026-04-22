# CI Runtime Policy Guide For Operational Skills

## Purpose

This guide explains how ADL operational skills should interpret PR validation
after the v0.90.3 changed-path CI policy introduced stable check names with
truthful skip behavior.

The source milestone policy is:

- `docs/milestones/v0.90.3/CI_RUNTIME_POLICY_v0.90.3.md`

## Core Rule

`adl-ci` and `adl-coverage` remain stable required PR check names, but a green
check does not always mean the same validation lane ran.

Skills must distinguish:

- `docs_only_path_policy_skip`: docs, planning, or non-runtime tooling changed;
  expensive Rust, demo smoke, and coverage phases were explicitly skipped
- `runtime_full_validation`: runtime, source, test, or demo-affecting surfaces
  changed; Rust validation, demo smoke where required, and coverage gates ran
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
- `demo_smoke_required`
- `fail_closed`
- `changed_count`
- `reason`

The `reason` field is the operator-facing explanation for why a lane ran or
was skipped.

## Skill Interpretation Rules

Use these rules when choosing validation, publishing PR truth, monitoring CI,
or assembling release evidence:

- A docs-only `adl-coverage` path-policy skip can be healthy for a PR, but it
  is not release coverage evidence.
- A runtime/source/test/demo-affecting PR should not skip Rust validation,
  demo smoke when required, or coverage gates.
- Rust source additions or heavy edits should run the coverage-impact preflight
  before publication. This is an authoring-time early warning, not a replacement
  for the full `adl-coverage` gate.
- `fail_closed=true` means full validation is required; it is not an approved
  skip.
- If the stable check result and changed files disagree, treat the PR as
  blocked or action-required until `pr-janitor` or a human resolves the
  discrepancy.
- Do not claim "full coverage passed" unless the coverage-required lane ran
  and produced coverage artifacts such as `coverage-summary.json`.
- Do not treat a green `adl-coverage` check as sufficient release evidence
  unless the evidence shows the full coverage lane ran.

## Examples

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
- `demo_smoke_required=true` when demo surfaces changed

Truthful interpretation:

- Rust fmt, clippy, tests, demo smoke where applicable, and coverage gates are
  expected.
- A skipped coverage lane is a blocker unless there is an explicit policy
  exception.

### Failed-Closed Classification

Observed:

- `fail_closed=true`

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
  validation from `main`, release ceremony, runtime PRs, or explicit coverage
  artifacts produced by a full coverage lane.
