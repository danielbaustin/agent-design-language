# Structured Output Record

Template: 1.0.0

Issue: 5349

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented deterministic provider and governed-tool adapters; receipts are non-blocking audit evidence and never an execution gate.

## Artifacts

- adl-v2/crates/adl-adapters
- .csdlc/prepared/issues/5349

## Execution

- Added deterministic scripted mock adapter
- Added bounded permit-authorized HTTPS adapter
- Added verified governed-tool adapter
- Added explicit lossless compatibility adapter
- Added focused dependency, security, budget, and exact-behavior proof

## Validation

[
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5349/run_validation_lane.rb",
      "all"
    ],
    "purpose": "Prove dependency/source readiness, all adapter behavior, secret and authority boundaries, strict quality, and inventory at the implementation revision without consulting receipts",
    "outcome": "passed",
    "evidence_ref": "local FastWork matrix: 54 tests; source_lines=555; test_lines=761; largest_module=285; all required lanes passed"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
