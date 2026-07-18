# Structured Task Prompt

Template: 1.0.0

Issue: 5514

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Restore all valid ADL CSM selectors in the exact bridge partition without changing runtime behavior, coverage thresholds, Runtime v2, or AWS lanes.

## Deliverables

- Complete owning-workspace ADL CSM expression
- Regression assertion for every retained ADL selector
- Passing #5504 changed-source coverage proof

## Acceptance

1. AC-1: Exact canonical-expression matching remains fail closed
2. AC-2: Every valid ADL CSM selector in the canonical expression reaches the ADL workspace
3. AC-3: Runtime v3 auth, supervision, and topology selectors reach only adl-runtime
4. AC-4: The nonexistent adl::cli_smoke selector and Runtime v2 remain absent
5. AC-5: The #5504 changed-source coverage gate passes without lowering thresholds

## Dependencies

- Issue #5512 workspace partition merged
- PR #5504 run 29645093147 retained failure evidence

## Inputs

- adl/tools/run_pr_fast_coverage_lane.sh
- adl/tools/test_run_pr_fast_coverage_lane.sh
- GitHub run 29645093147 job 88081915666

## Non Goals

- Production runtime changes
- Runtime v2 source or tests
- Coverage threshold changes
- AWS execution
