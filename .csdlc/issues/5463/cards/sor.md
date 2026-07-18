# Structured Output Record

Template: 1.0.0

Issue: 5463

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Move occurrence accounting after canonical SHA validation so rejected fixtures cannot satisfy real inventory presence checks.

## Artifacts

- .github/workflows/aws-codefriend-build.yaml
- .github/workflows/aws-spot-remote-validation.yaml
- .github/workflows/ci.yaml
- .github/workflows/nightly-coverage-ratchet.yaml
- .github/workflows/v0871_milestone_closeout_gate.yaml
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_ci_path_policy.sh
- docs/tooling/GITHUB_ACTIONS_RUNTIME_PIN_INVENTORY.md
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_ci_runtime_contracts.sh

## Execution

- Upgrade actions/checkout to the immutable v7.0.0 Node 24 revision
- Upgrade actions/upload-artifact to the immutable v7.0.1 Node 24 revision
- Upgrade Swatinem/rust-cache to the immutable v2.9.1 Node 24 revision
- Scan both list-style and mapping-style uses syntax for canonical and deprecated revisions
- Retain the source-linked runtime pin inventory and explicit no-AWS proof boundary
- Parse matching single- and double-quoted uses scalars
- Fail closed on malformed quoted uses scalars
- Add negative fixtures for quoted list-style and mapping-style floating pins
- Count a tracked action only after its uses scalar exactly matches the canonical immutable SHA

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "purpose": "Prove canonical immutable Node 24 revisions and deprecated-revision absence across every workflow",
    "outcome": "passed",
    "evidence_ref": "local:5463-ci-runtime-contract-pass"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_path_policy.sh"
    ],
    "purpose": "Prove the action pin upgrade preserves CI path-policy behavior and fixture truth",
    "outcome": "passed",
    "evidence_ref": "local:5463-ci-path-policy-pass"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "purpose": "Prove quoted and unquoted tracked action uses cannot escape canonical immutable Node 24 enforcement",
    "outcome": "passed",
    "evidence_ref": "local:5463-quoted-uses-regression-pass"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "purpose": "Prove rejected negative fixtures cannot mask disappearance of a real canonical action occurrence",
    "outcome": "passed",
    "evidence_ref": "local:5463-occurrence-accounting-regression-pass"
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
