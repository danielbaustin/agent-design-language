# Structured Output Record

Template: 1.0.0

Issue: 5353

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Repaired issue-local initialization atomicity and complete design/diagram digest refresh.

## Artifacts

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate2.rs

## Execution

- Preflight bootstrap validation and reject reserved control paths before mutation
- Preserve issue-local design and diagram files across atomic record swaps
- Refresh SPP and VPP design and diagram digests during approval

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "purpose": "Run complete C-SDLC v2 regression suite",
    "outcome": "passed",
    "evidence_ref": "local:cargo-test-csdlc-v2"
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
