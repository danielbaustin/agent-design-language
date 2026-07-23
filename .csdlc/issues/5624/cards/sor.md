# Structured Output Record

Template: 1.0.0

Issue: 5624

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Resolve terminal worktree identity canonically while preserving exact branch, topology, cleanliness, and receipt invariants.

## Artifacts

- csdlc-v2/src/readiness.rs
- csdlc-v2/tests/gate7.rs
- csdlc-v2/tests/gate7_lifecycle.rs
- .csdlc/evidence/5624/validation-380a9eab1.json

## Execution

- Resolve literal dot to the canonical current issue worktree
- Resolve clean repository-relative and absolute terminal worktree paths exactly
- Reject malformed, missing, suffix-collision, wrong-checkout, wrong-branch, and dirty targets
- Preserve terminal receipt bytes during validation

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/5624/run_focused_validation.sh"
    ],
    "purpose": "Prove canonical prune topology, receipt immutability, the command-level issue-local worktree path, all C-SDLC v2 targets, formatting, and strict Clippy.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5624/validation-380a9eab1.json"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/5624/run_focused_validation.sh"
    ],
    "purpose": "Prove the external Cargo target contract, Gate 10A provenance, guarded prune behavior, complete C-SDLC v2 suite, formatting, and strict Clippy in one FastWork run.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5624/validation-a2c79708e.json"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_cargo_validation.sh",
      "and",
      "cargo",
      "test",
      "--test",
      "gate2",
      "and",
      "fmt",
      "and",
      "strict-clippy"
    ],
    "purpose": "Prove deterministic validation-only Git identity, all affected Gate2 fixture commits, formatting, and strict Clippy.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5624/validation-git-identity.json"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
