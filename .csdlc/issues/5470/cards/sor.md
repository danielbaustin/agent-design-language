# Structured Output Record

Template: 1.0.0

Issue: 5470

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Terminal reconciliation now journals projection and receipt updates across durable write and rename boundaries.

## Artifacts

- csdlc-v2/src/store.rs
- csdlc-v2/src/publication.rs
- csdlc-v2/tests/gate7_lifecycle.rs

## Execution

- Add a shared terminal transaction journal keyed by issue and record digests.
- Synchronize receipt temp files and parent directories before success.
- Recover interrupted projection/receipt pairs deterministically.

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
      "gate7_lifecycle"
    ],
    "purpose": "Prove projection/receipt recovery at every durable boundary and preserve terminal lifecycle contracts.",
    "outcome": "passed",
    "evidence_ref": "gate7_lifecycle: 6 tests passed; clippy and diff-check passed"
  }
]

## Integration

pr_open

## Publication

Publication: draft

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
