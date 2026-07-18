# Structured Output Record

Template: 1.0.0

Issue: 5464

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Replaced the stale nextest installer manifest with the immutable v2.82.10 revision that contains nextest 0.9.140 and disabled fallback on every hosted install.

## Artifacts

- .github/workflows/ci.yaml
- adl/tools/test_ci_runtime_contracts.sh

## Execution

- Pin all four nextest install steps to install-action v2.82.10 commit 50414676f9f5d50a65992c6dd2ed02641263226c
- Set fallback to none on every nextest install step
- Require the canonical installer, nextest version, fallback policy, and complete four-step inventory in the CI runtime contract

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "purpose": "Prove every hosted nextest install uses the supported immutable manifest, pins 0.9.140, and disables fallback",
    "outcome": "passed",
    "evidence_ref": "local:5464-nextest-install-contract-pass"
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
