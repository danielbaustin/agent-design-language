# Structured Output Record

Template: 1.0.0

Issue: 5527

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Typed terminal repair now replaces one exact stale SOR artifact reference with a receipt-authenticated retained artifact and refreshes record and receipt atomically.

## Artifacts

- csdlc-v2/src/model.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/bin/csdlc-closeout.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/tests/gate7_lifecycle.rs
- .csdlc/issues/5390

## Execution

- Added TerminalSorArtifactRepairRequest to the public typed schema.
- Added csdlc-closeout repair-sor-artifact with authority claim, target, receipt, path, and byte-digest guards.
- Added rollback-safe terminal transaction handling and exact/nonduplicating SOR replacement.
- Reconciled and repaired #5390 from the deleted diagram path to retained/diagram.mmd.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--test",
      "gate7_lifecycle"
    ],
    "purpose": "Prove terminal reconciliation, receipt parity, authority scoping, exact SOR replacement, and rollback behavior.",
    "outcome": "passed",
    "evidence_ref": "12 passed, 0 failed, including terminal_sor_artifact_repair_is_scoped_atomic_and_receipt_bound"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject compiler or Clippy warnings across the complete C-SDLC v2 target surface.",
    "outcome": "passed",
    "evidence_ref": "Finished dev profile successfully with warnings denied"
  },
  {
    "command": [
      "csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5390"
    ],
    "purpose": "Prove #5390 remains closed_out with coherent generated cards after typed repair.",
    "outcome": "passed",
    "evidence_ref": "doctor status pass at generation 41; SOR uses retained/diagram.mmd; record and receipt digests agree"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--all",
      "--",
      "--check"
    ],
    "purpose": "Prove canonical Rust formatting and clean patch hygiene.",
    "outcome": "passed",
    "evidence_ref": "cargo fmt --check and git diff --check passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--test",
      "gate7_lifecycle",
      "terminal_sor_artifact_repair_is_scoped_atomic_and_receipt_bound",
      "--",
      "--exact"
    ],
    "purpose": "Prove the exact declared atomic SOR artifact repair and rollback behavior.",
    "outcome": "passed",
    "evidence_ref": "1 passed, 0 failed, 11 filtered out"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate7_lifecycle",
      "terminal_sor_artifact_repair_is_scoped_atomic_and_receipt_bound",
      "--",
      "--exact"
    ],
    "purpose": "Prove the exact declared root-relative atomic SOR artifact repair and rollback lane.",
    "outcome": "passed",
    "evidence_ref": "Executed literally from the issue worktree root: 1 passed, 0 failed, 11 filtered out"
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
