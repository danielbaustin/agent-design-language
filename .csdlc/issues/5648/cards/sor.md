# Structured Output Record

Template: 1.0.0

Issue: 5648

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Added typed operator-authorized active-claim revoke with CAS, expiry guard, audit, schema, CLI route, and focused tests.

## Artifacts

- .csdlc/evidence/5648/local

## Execution

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/bin/csdlc-bind.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/tests/gate2.rs

## Validation

[
  {
    "command": [
      "csdlc-validate",
      "--request",
      ".csdlc/prepared/issues/5648/validate-local.json"
    ],
    "purpose": "Prove operator CAS revoke, stale guards, formatting, and strict targeted lint",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5648/local contains a local_pass three-lane proof: claim-revoke focused tests, cargo fmt --check, and strict targeted Clippy all passed."
  },
  {
    "command": [
      "csdlc-validate",
      "--request",
      ".csdlc/prepared/issues/5648/validate-local.json"
    ],
    "purpose": "Prove operator CAS revoke, stale guards, formatting, and strict targeted lint",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5648/local contains a local_pass three-lane proof: claim-revoke focused tests, cargo fmt --check, and strict targeted Clippy all passed."
  },
  {
    "command": [
      "csdlc-validate",
      "--request",
      ".csdlc/prepared/issues/5648/validate-local.json"
    ],
    "purpose": "Prove operator CAS revoke, stale guards, formatting, and strict targeted lint",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5648/local contains a local_pass three-lane proof: claim-revoke focused tests, cargo fmt --check, and strict targeted Clippy all passed."
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
