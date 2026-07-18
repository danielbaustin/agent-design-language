# Structured Output Record

Template: 1.0.0

Issue: 5487

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Closed-out design repair now performs authority/CAS/hash checks, markdown AST validation, journaled projection and receipt replacement, and typed reconciliation materializes repaired #5467 artifacts.

## Artifacts

- csdlc-v2/src/store.rs
- csdlc-v2/src/bin/csdlc-closeout.rs
- .csdlc/issues/5467/retained/design.md
- .csdlc/issues/5467/retained/diagram.mmd

## Execution

- Add TerminalDesignRepairRequest and csdlc-closeout repair-design.
- Atomically update terminal record, SPP/VPP digests, receipt, authored artifacts, and audit.
- Repair #5467 retained design and diagram with the approved SSM/EBS/IAM scope.

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
    "purpose": "Prove terminal transaction recovery and lifecycle invariants; clippy and diff checks also pass.",
    "outcome": "passed",
    "evidence_ref": "gate7_lifecycle: 6 tests passed; cargo clippy --all-targets -D warnings; git diff --check"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate7_terminal_design_repair_5487"
    ],
    "purpose": "Verify the typed repair request covers committed #5467 design and diagram artifacts with stable digests and Mermaid shape.",
    "outcome": "passed",
    "evidence_ref": "gate7_terminal_design_repair_5487: 1 test passed"
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
