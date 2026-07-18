# Structured Output Record

Template: 1.0.0

Issue: 5464

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Close alternate YAML-form escapes by coupling canonical named-block checks to exact whole-workflow nextest selection counts.

## Artifacts

- .github/workflows/ci.yaml
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_ci_runtime_contracts.sh

## Execution

- Pin all four nextest install steps to install-action v2.82.10 commit 50414676f9f5d50a65992c6dd2ed02641263226c
- Set fallback to none on every nextest install step
- Require the canonical installer, nextest version, fallback policy, and complete four-step inventory in the CI runtime contract
- Require exactly four nextest selections and exactly four 0.9.140 selections across the whole workflow
- Reject the alternate cargo-nextest alias
- Add negative fixtures for unnamed, quoted, inline-map, floating-installer, and fallback-enabled forms

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
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "purpose": "Prove unnamed, quoted, inline, floating-installer, and fallback-enabled nextest forms cannot escape enforcement",
    "outcome": "passed",
    "evidence_ref": "local:5464-nextest-bypass-regression-pass"
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
