# Structured Output Record

Template: 1.0.0

Issue: 5427

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Added typed identity-version semantic operation and cross-issue repair route.

## Artifacts

- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/bin/csdlc-edit.rs
- csdlc-v2/tests/card_identity.rs

## Execution

- Update all six card identity versions atomically through typed operation
- Reject malformed identity versions before mutation
- Repair #5353 card identities from v0.91.8 to v0.91.7
- Add focused round-trip and rejection tests

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
    "purpose": "Prove identity round-trip, malformed rejection, full v2 regression, and repair-compatible projections",
    "outcome": "passed",
    "evidence_ref": "local:5427-card-identity-and-csdlc-v2-suite"
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
