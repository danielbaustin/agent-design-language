# Structured Output Record

Template: 1.0.0

Issue: 5602

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Use cargo-llvm-cov clean and show-env, run partitioned cargo nextest directly with run-scoped instrumentation targets, and render one explicit combined report per workspace.

## Artifacts

- adl/tools/run_authoritative_coverage_lane.sh
- adl/tools/test_run_authoritative_coverage_lane.sh
- adl/tools/run_authoritative_coverage_lane.sh
- adl/tools/test_run_authoritative_coverage_lane.sh
- adl/tools/test_ci_runtime_contracts.sh

## Execution

- Collect workspace coverage profiles without partition-local reports
- Collect companion Runtime profiles without partition-local reports
- Strengthen exact command-shape regression expectations
- Replace the incompatible no-clean plus no-report wrapper invocation with the documented external-test workflow
- Isolate CARGO_TARGET_DIR and CARGO_LLVM_COV_TARGET_DIR per coverage run
- Update the authoritative and CI runtime contracts to prove show-env instrumentation is consumed

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_authoritative_coverage_lane.sh"
    ],
    "purpose": "Prove profile-only partition collection, explicit combined reporting, failure propagation, concurrency isolation, and unchanged gates; FastWork plan separately resolved under /Volumes/FastWork/adl-5602-coverage.",
    "outcome": "passed",
    "evidence_ref": "local:5602-authoritative-coverage-contract-pass"
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
