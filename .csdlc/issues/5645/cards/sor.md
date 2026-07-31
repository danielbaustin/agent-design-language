# Structured Output Record

Template: 1.0.0

Issue: 5645

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Added typed csdlc-merge request/result contracts, canonical merge_ready and exact-head validation, Octocrab merge invocation, merge-SHA output, and focused fail-closed tests.

## Artifacts

- csdlc-v2/src/merge.rs
- csdlc-v2/src/bin/csdlc-merge.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/Cargo.toml

## Execution

- Added csdlc-v2 merge module and public schemas.
- Added csdlc-merge binary using the shared token resolver and Octocrab.
- Kept csdlc-publish non-merging.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "merge"
    ],
    "purpose": "Prove typed merge request/result schemas, exact-head drift rejection, required-check gating, and successful green gate classification.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5645/merge-command.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "merge"
    ],
    "purpose": "Prove typed merge request/result schemas, exact-head drift rejection, required-check gating, and successful green gate classification.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5645/merge-command.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "merge"
    ],
    "purpose": "Prove typed merge request/result schemas, exact-head drift rejection, required-check gating, and successful green gate classification.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5645/merge-command.log"
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
