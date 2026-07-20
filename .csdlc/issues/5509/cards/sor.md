# Structured Output Record

Template: 1.0.0

Issue: 5509

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Route the closed Runtime v3/CSM bridge family through independent focused tests and composed coverage while retaining fail-closed behavior for every other mixed-crate change.

## Artifacts

- adl/tools/ci_path_policy.sh
- adl/tools/run_pr_fast_test_lane.sh
- adl/tools/test_ci_path_policy.sh
- adl/tools/test_run_pr_fast_coverage_lane.sh
- adl/tools/test_run_pr_fast_test_lane.sh

## Execution

- Add a mixed_focused two-crate test plan for the Runtime v3/CSM bridge family
- Execute ADL CSM tests and Runtime v3 tests in their owning crates
- Select focused composed coverage for the closed path family
- Preserve full-validation fallback when any unrelated Rust path is present
- Add positive, execution, coverage-composition, and negative routing contracts

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_pr_fast_test_lane.sh"
    ],
    "purpose": "Prove the bounded bridge runs focused tests in both owning crates and rejects unrelated mixed paths",
    "outcome": "passed",
    "evidence_ref": "local:5509-pr-fast-two-crate-contract-pass"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_pr_fast_coverage_lane.sh"
    ],
    "purpose": "Prove both crate coverage runs and summaries are composed without Runtime v2 selection",
    "outcome": "passed",
    "evidence_ref": "local:5509-focused-coverage-composition-pass"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_path_policy.sh"
    ],
    "purpose": "Prove the exact bridge family selects focused coverage and unrelated additions remain fail closed",
    "outcome": "passed",
    "evidence_ref": "local:5509-ci-path-policy-contract-pass"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
