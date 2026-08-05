# Structured Output Record

Template: 1.0.0

Issue: 5657

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Reject degraded production adapters before readiness, remove TLS private-key material from continuity identity, and keep the runtime as an Axum JSON API for the separate Observatory app.

## Artifacts

- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/operations.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/tests/assembly.rs

## Execution

- Added typed production adapter readiness validation.
- Made the serve binary fail closed before listener readiness when required adapters are degraded or missing.
- Removed TLS private-key hash from continuity identity projection.
- Kept the runtime API-only; the separate Observatory app consumes the authenticated feed.
- Added focused production-readiness regression proof.

## Validation

[
  {
    "command": [
      "/usr/bin/env",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--target-dir",
      "/Volumes/FastWork/adl-wp-5657/target",
      "--test",
      "assembly",
      "--test",
      "observatory"
    ],
    "purpose": "Run assembly, Observatory WebSocket, and formatting-compatible Rust tests against the exact implementation.",
    "outcome": "passed",
    "evidence_ref": "runtime-v3-launch-focused.log"
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
