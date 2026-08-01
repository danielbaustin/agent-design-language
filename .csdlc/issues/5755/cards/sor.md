# Structured Output Record

Template: 1.0.0

Issue: 5755

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented #5755 Runtime v3 protocol/control security fixes needed to unblock #5664 terminal closeout.

## Artifacts

- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/tests/control.rs
- adl-runtime-kernel/src/protocol_adapters.rs
- adl-runtime-kernel/tests/protocol_adapters.rs

## Execution

- Added route-level /v1/control request body limit and oversized-body negative test.
- Added explicit RustlsMutualTlsClient mode with bound client certificate identity.
- Rejected no-client-auth RustlsClient usage for protocol adapters.
- Updated protocol env construction to require client certificate and key files.
- Added mTLS positive proof with server-observed client certificate identity.

## Validation

[
  {
    "command": [
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-wp-5755-target",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "control",
      "--test",
      "protocol_adapters"
    ],
    "purpose": "Prove /v1/control body limit, protocol adapter no-client-auth rejection, and mTLS client identity positive path.",
    "outcome": "passed",
    "evidence_ref": "local terminal output: control 22 passed; protocol_adapters 11 passed"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Prove whitespace/diff hygiene for #5755 Runtime v3 security repair.",
    "outcome": "passed",
    "evidence_ref": "local terminal output: git diff --check produced no output"
  },
  {
    "command": [
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-wp-5755-target",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "control",
      "--test",
      "protocol_adapters"
    ],
    "purpose": "Final exact-diff proof after diagnostic fix: /v1/control body limit, mTLS boundary, no-client-auth rejection, and invalid mTLS key diagnostic regression.",
    "outcome": "passed",
    "evidence_ref": "local terminal output: control 22 passed; protocol_adapters 12 passed"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "purpose": "Prove Rust formatting for #5755 Runtime v3 security repair.",
    "outcome": "passed",
    "evidence_ref": "local terminal output: cargo fmt --check passed"
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
