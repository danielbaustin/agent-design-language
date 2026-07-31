# Structured Output Record

Template: 1.0.0

Issue: 5664

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the disjoint Provider, ACIP, A2A, and Cloud Bridge protocol adapter slice with authenticated request and response frames, atomic replay rejection, and fail-closed protocol production builder behavior.

## Artifacts

- adl-runtime-kernel/Cargo.toml
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/src/protocol_adapters.rs
- adl-runtime-kernel/tests/protocol_adapters.rs

## Execution

- Added real Tokio/Rustls protocol adapters for Provider, ACIP, A2A, and Cloud Bridge.
- MAC-bound protocol responses to the original request frame and added tamper regression coverage.
- Made replay reservation atomic for concurrent direct adapter execution while preserving bounded retry behavior.
- Made the protocol production builder return no partial executors when required protocol configuration is missing.

## Validation

[
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-5664-protocol-clippy-target",
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "--bin",
      "adl-runtime-kernel",
      "--test",
      "protocol_adapters",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove the touched runtime kernel lib, runtime binary, and protocol adapter test compile warning-free.",
    "outcome": "passed",
    "evidence_ref": "protocol-adapter-clippy.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-5664-protocol-target",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "protocol_adapters"
    ],
    "purpose": "Prove Provider, ACIP, A2A, and Cloud Bridge protocol behavior including Rustls, response MAC tamper rejection, concurrent replay rejection, timeout, retry, shutdown, and fail-closed config handling.",
    "outcome": "passed",
    "evidence_ref": "protocol-adapter-tests.log"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
