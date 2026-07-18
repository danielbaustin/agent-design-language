# Structured Output Record

Template: 1.0.0

Issue: 5514

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Expand the exact Runtime v3/CSM bridge partition to every valid ADL CSM family while retaining Runtime v3-only companion coverage.

## Artifacts

- adl/tools/run_pr_fast_coverage_lane.sh
- adl/tools/test_run_pr_fast_coverage_lane.sh

## Execution

- Retain all canonical ADL library CSM selectors
- Route csmctl and csm_service tests through the canonical ADL CLI binary
- Keep the nonexistent cli_smoke selector out of ADL coverage
- Assert every retained selector in the command-level regression

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_pr_fast_coverage_lane.sh"
    ],
    "purpose": "Prove exact partition completeness, foreign-selector rejection, near-match behavior, and summary composition",
    "outcome": "passed",
    "evidence_ref": "local:5514-pr-fast-coverage-contract-pass"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
