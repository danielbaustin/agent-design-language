# Structured Output Record

Template: 1.0.0

Issue: 5727

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented typed safe reacquisition of deliberately released or expired C-SDLC v2 claims without lifecycle rewind.

## Artifacts

- .csdlc/prepared/issues/5727/design.md
- .csdlc/prepared/issues/5727/diagram.mmd
- .csdlc/evidence/5727

## Execution

- Added a typed csdlc-bind reacquisition request and result contract with CAS, binding, lease, and live-overlap guards.
- Made dormant nonterminal records readable and doctor-classifiable while preserving writer-claim enforcement for mutations.
- Added command-level, released-claim, expired-claim, stale-state, binding, overlap, schema, and real #5354 acceptance proof.

## Validation

[
  {
    "command": [
      "/usr/bin/env",
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
    "purpose": "Prove the changed Rust surfaces are warning-free across all targets.",
    "outcome": "passed",
    "evidence_ref": "clippy.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all",
      "--",
      "--check"
    ],
    "purpose": "Prove Rust formatting is canonical.",
    "outcome": "passed",
    "evidence_ref": "fmt.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "purpose": "Prove typed lifecycle, reacquisition, schema, CAS, collision, and CLI behavior.",
    "outcome": "passed",
    "evidence_ref": "gate2.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate7_lifecycle"
    ],
    "purpose": "Prove lifecycle integration and worktree binding behavior remain intact.",
    "outcome": "passed",
    "evidence_ref": "gate7.log"
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
