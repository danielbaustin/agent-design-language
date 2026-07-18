# Structured Output Record

Template: 1.0.0

Issue: 5464

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Validate all install-action steps before classifying normalized nextest and cargo-nextest tool values.

## Artifacts

- .github/workflows/ci.yaml
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_ci_runtime_contracts.sh

## Execution

- Pin all four nextest install steps to install-action v2.82.10 commit 50414676f9f5d50a65992c6dd2ed02641263226c
- Set fallback to none on every nextest install step
- Require the canonical installer, nextest version, fallback policy, and complete four-step inventory in the CI runtime contract
- Require exactly four nextest selections and exactly four 0.9.140 selections across the whole workflow
- Reject the alternate cargo-nextest alias
- Add negative fixtures for unnamed, quoted, inline-map, floating-installer, and fallback-enabled forms
- Require each install-action use to occupy its own named step
- Normalize block-style tool scalars and classify versioned and unversioned nextest aliases
- Add negative fixtures for unversioned nextest and cargo-nextest aliases

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
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "purpose": "Prove every install-action step is inventoried and both unversioned nextest aliases fail closed",
    "outcome": "passed",
    "evidence_ref": "local:5464-unversioned-alias-regression-pass"
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
