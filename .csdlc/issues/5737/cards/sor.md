# Structured Output Record

Template: 1.0.0

Issue: 5737

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Claim scans now consider only claims matching active checkouts, defer terminal receipt checks until a real path overlap, and repair dormant typed projections during authority reacquisition.

## Artifacts

- .csdlc/evidence/5737/gate2.log
- .csdlc/evidence/5737/clippy.log

## Execution

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate2.rs

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "purpose": "Prove active claim collision safety, stale projection filtering, and authority reacquisition.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5737/gate2.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove the simplified claim implementation remains warning-free across all targets.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5737/clippy.log"
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
