# Structured Output Record

Template: 1.0.0

Issue: 5518

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Make the terminal journal durable before the first reconciled projection swap.

## Artifacts

- csdlc-v2/src/model.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/bin/csdlc-closeout.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/lib.rs
- .csdlc/issues/5516
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate7_lifecycle.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate7_lifecycle.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate7_lifecycle.rs

## Execution

- Add TerminalPlanStepRepairRequest to the public typed schema
- Add csdlc-closeout repair-plan-step command
- Require distinct active authority and closed-out claim-free target with exact receipt CAS
- Permit only pending or in-progress SPP step completion
- Regenerate projections, audit, record digest, and receipt atomically with rollback
- Repair #5516 SPP S3 from pending to completed
- Require the authority claim to own the exact .csdlc/issues/<target> path
- Hold a git-common-dir terminal-repairs lock across repair CAS, projection commit, and receipt replacement
- Share that lock with identity and terminal design repair operations
- Add Store-level tests for scope rejection, injected rollback, success parity, stale target, stale receipt, and nonterminal target rejection
- Acquire the shared terminal repair lock before the issue-local lock in retain_terminal_receipt
- Acquire the shared terminal repair lock before the issue-local lock in reconcile_terminal
- Recover common terminal journals only from entrypoints holding the git-common-dir terminal lock
- Make ordinary lifecycle mutations return reconciliation_required when a terminal journal is present
- Verify ordinary edits cannot alter the journal or receipt at any durable interruption boundary
- Remove the redundant pre-journal projection commit from terminal reconciliation
- Assert the after-journal interruption leaves the original projection and receipt unchanged

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "terminal_plan_repair"
    ],
    "purpose": "Prove incomplete authority rejection and forward-only pending or in-progress to completed status semantics",
    "outcome": "passed",
    "evidence_ref": "local FastWork: 2 focused terminal plan repair tests passed"
  },
  {
    "command": [
      "csdlc-closeout",
      "repair-plan-step",
      "--request",
      ".csdlc/prepared/issues/5518/repair-5516-plan-step-rollback-test.json"
    ],
    "purpose": "Prove post-projection interruption rolls back both target and receipt, then prove successful #5516 S3 repair and doctor parity",
    "outcome": "passed",
    "evidence_ref": "local: injected exit 69 restored #5516 generation 18 and receipt c1e5a632; successful repair produced generation 19 digest 317cf0e0, S3 completed in card and receipt, doctor pass"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove warning-free typed terminal repair code across all targets",
    "outcome": "passed",
    "evidence_ref": "local FastWork: all C-SDLC v2 targets passed strict Clippy"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "terminal_plan_repair"
    ],
    "purpose": "Prove authority scope, forward-only semantics, atomic rollback, success parity, stale CAS rejection, phase rejection, and warning-free code",
    "outcome": "passed",
    "evidence_ref": "local FastWork: 3 unit tests plus Store-level gate7 and gate9 integration coverage passed; all-target strict Clippy passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "terminal_plan_repair"
    ],
    "purpose": "Prove the reviewed shared-lock repair preserves terminal plan repair behavior and warning-free code",
    "outcome": "passed",
    "evidence_ref": "local FastWork: focused unit, gate7, gate9, exact integration, strict all-target Clippy, and git diff check passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "terminal_projection_and_receipt_recover_at_each_durable_boundary"
    ],
    "purpose": "Prove uncoordinated lifecycle operations cannot mutate shared terminal recovery state",
    "outcome": "passed",
    "evidence_ref": "local FastWork: all six durability boundaries, focused terminal repair tests, strict all-target Clippy, cargo fmt check, and git diff check passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "terminal_projection_and_receipt_recover_at_each_durable_boundary"
    ],
    "purpose": "Prove the original projection and receipt remain intact when interrupted immediately after journal durability",
    "outcome": "passed",
    "evidence_ref": "local FastWork: six-boundary durability test, focused terminal repair tests, strict all-target Clippy, cargo fmt check, and git diff check passed"
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
