# CI Runtime Budgets - v0.91.2

## Status

Follow-on observability surface for `#3134`.

This document adds the missing job/step-level view above the existing nextest
hotspot report. It is meant to answer "where did the PR wait go?" without
forcing an operator to read the full GitHub Actions log by hand.

## Source Of Truth

The current workflow file is:

- `.github/workflows/ci.yaml`

The issue text originally named `.github/workflows/ci.yml`; that is stale path
wording. The repo uses `.yaml`.

## Budget Model

Budgets are triage thresholds, not release gates. A budget miss should route
work; it should not silently fail or pass a release by itself.

| Surface | Default Budget | What It Means |
| --- | ---: | --- |
| `adl-ci` job | `600s` | Ordinary PR CI should normally finish quickly unless Rust proof is required. |
| `adl-coverage` job | `900s` | Coverage should be visible as the dominant cost when it is truly required. |
| `lane-selection` bucket | `60s` | Path policy and coverage-impact classification should stay cheap. |
| `setup-install-cache` bucket | `240s` | Toolchain, cache, linker, and setup overhead should not dominate ordinary PRs. |
| `tooling-contracts` bucket | `240s` | Shell/Python/skill contract checks should remain bounded. |
| `rust-test-execution` bucket | `600s` | Rust test work should point to the focused or slow-proof lane responsible. |
| `coverage-execution` bucket | `900s` | Coverage cost should be explicit and routed through coverage policy. |
| `reporting-upload` bucket | `120s` | Artifacts and summaries should not dominate runtime. |

## Operator Commands

Capture GitHub Actions job timing JSON:

```bash
gh run view <run-id> --json jobs > .adl/tmp/ci-jobs.json
```

Render the default Markdown report:

```bash
python3 adl/tools/summarize_ci_runtime.py .adl/tmp/ci-jobs.json
```

Render machine-readable output for follow-on analysis:

```bash
python3 adl/tools/summarize_ci_runtime.py .adl/tmp/ci-jobs.json --format json
```

Override a budget for a one-off analysis:

```bash
python3 adl/tools/summarize_ci_runtime.py .adl/tmp/ci-jobs.json \
  --job-budget adl-coverage=1200 \
  --category-budget coverage-execution=1200
```

## Runtime Buckets

The report groups steps into routing buckets:

- `lane-selection`: `ci_path_policy` and coverage-impact classification.
- `setup-install-cache`: Rust toolchain, cache, linker, `sccache`, and setup.
- `tooling-contracts`: shell, docs, guardrail, and skill contract checks.
- `rust-test-execution`: `fmt`, `clippy`, nextest, and doctest work.
- `coverage-execution`: `cargo llvm-cov`, coverage gates, and coverage reports.
- `reporting-upload`: stats, summaries, and uploaded artifacts.
- `skipped-policy`: explicit policy skips/deferred proof messages.
- `other`: uncategorized steps that may need a script update if they become
  common.

## Representative Before State

The captured `#3032` nextest log remains the slow-cycle before-state for the
runtime/test recovery lane:

- declared run: `1,882` tests across `28` binaries
- completed timing rows parsed: `1,881`
- slow threshold markers: `77`
- total parsed completed runtime: `7364.644s`
- dominant cluster: `runtime-v2-contract-registry/accessors`
- dominant cluster total: `1860.104s`

That test-body hotspot is already summarized by:

```bash
python3 adl/tools/summarize_nextest_timings.py .adl/docs/TBD/test-logs.txt --top 12 --min-seconds 45
```

The CI runtime budget report complements that test-body report by showing
whether future PR delay comes from setup, lane selection, test execution,
coverage execution, or reporting.

## Representative Fast Docs PR State

PR `#3147` was a recent docs-only publication-backlog update. Its GitHub checks
completed quickly:

| Check | Started | Completed | Approx Runtime |
| --- | --- | --- | ---: |
| `adl-ci` | `2026-05-20T16:11:27Z` | `2026-05-20T16:11:39Z` | `12s` |
| `adl-coverage` | `2026-05-20T16:11:43Z` | `2026-05-20T16:11:52Z` | `9s` |

That is the desired shape for docs-only changes: path policy should skip Rust
and coverage work while still leaving green check evidence.

## Routing Rules

- If `lane-selection` is over budget, inspect `adl/tools/ci_path_policy.sh`,
  `adl/tools/check_coverage_impact.sh`, and their contract tests.
- If `setup-install-cache` dominates, inspect toolchain install, cache restore,
  linker setup, and `sccache` stats before blaming tests.
- If `rust-test-execution` dominates, run
  `adl/tools/summarize_nextest_timings.py` against the captured nextest log and
  route to test refactor, selector policy, or slow-proof consolidation.
- If `coverage-execution` dominates, inspect whether the PR truly needed full
  coverage or should have used changed-source coverage impact.
- If `tooling-contracts` dominates, split or focus the expensive shell/Python
  contract check rather than widening Rust test policy.
- If `reporting-upload` dominates, inspect artifact size and upload scope.

## Validation

Focused validation for this observability surface:

```bash
python3 -m py_compile adl/tools/summarize_ci_runtime.py
bash adl/tools/test_summarize_ci_runtime.sh
```

Do not run a full Rust or coverage suite merely to prove this parser. Use
captured GitHub Actions JSON, a fixture, or an existing slow-run artifact unless
the issue being diagnosed independently requires a real rerun.

## Non-Claims

- This surface does not fix slow tests by itself.
- This surface does not replace nextest hotspot diagnosis.
- This surface does not replace coverage gates, release proof, or human review.
- These budgets are routing thresholds, not final quality gates.
