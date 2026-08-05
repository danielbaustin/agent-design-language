# Structured Output Record

Template: 1.0.0

Issue: 5663

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Resolved the two remaining consolidated review findings after the five real Runtime v3 adapter fixes. Duplicate in-flight callers now wait on the owner result through a cancellation-aware watch channel and cannot become synthetic executors; each waiter can cancel independently without poisoning the owner result. writer.lock acquisition now publishes owner.json through a pending lock directory and recovers partial locks without owner metadata. Current touched source/test physical LoC is before 3796, after 3791, net -5.

## Artifacts

- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/governed_operations.rs
- adl-runtime-kernel/tests/assembly.rs
- adl-runtime-kernel/tests/operations.rs
- .csdlc/evidence/5663
- .csdlc/prepared/issues/5663/validate-after-review.json
- .csdlc/evidence/5663/runtime-v3-local-adapters-assembly.log
- .csdlc/evidence/5663/runtime-v3-local-adapters-clippy.log
- .csdlc/evidence/5663/runtime-v3-local-adapters-loc.log
- .csdlc/prepared/issues/5663/replace-validation-lanes-after-review.json
- .csdlc/prepared/issues/5663/validate-after-review.json
- .csdlc/evidence/5663/runtime-v3-local-adapters-loc.log
- .csdlc/prepared/issues/5663/replace-validation-lanes-after-review.json
- .csdlc/prepared/issues/5663/validate-after-review.json
- .csdlc/evidence/5663/runtime-v3-local-adapters-loc.log
- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/operations.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/governed_operations.rs
- adl-runtime-kernel/tests/assembly.rs
- .csdlc/evidence/5663/runtime-v3-local-adapters-assembly.log
- .csdlc/evidence/5663/runtime-v3-local-adapters-governed.log
- .csdlc/evidence/5663/runtime-v3-local-adapters-clippy.log
- .csdlc/evidence/5663/runtime-v3-local-adapters-loc.log
- .csdlc/evidence/5663/runtime-v3-local-adapters-diff-check.log
- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/operations.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/governed_operations.rs
- adl-runtime-kernel/tests/assembly.rs
- .csdlc/prepared/issues/5663/validate-real-fixes.json
- .csdlc/evidence/5663/runtime-v3-local-adapters-assembly.log
- .csdlc/evidence/5663/runtime-v3-local-adapters-governed.log
- .csdlc/evidence/5663/runtime-v3-local-adapters-clippy.log
- .csdlc/evidence/5663/runtime-v3-local-adapters-loc.log
- .csdlc/evidence/5663/runtime-v3-local-adapters-diff-check.log
- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/operations.rs
- adl-runtime-kernel/tests/assembly.rs
- .csdlc/prepared/issues/5663/validate-real-fixes.json
- .csdlc/evidence/5663/runtime-v3-local-adapters-assembly.log
- .csdlc/evidence/5663/runtime-v3-local-adapters-governed.log
- .csdlc/evidence/5663/runtime-v3-local-adapters-clippy.log
- .csdlc/evidence/5663/runtime-v3-local-adapters-loc.log
- .csdlc/evidence/5663/runtime-v3-local-adapters-diff-check.log

## Execution

- Replaced production in-process adapter receipts with bounded local Agent, Shepherd, Scheduler, Chronosense, CheckpointStore, and Lifelog behavior.
- Kept external Provider, ACIP, A2A, and Cloud Bridge transports fail-closed until their separate adapter scope binds real transports.
- Consolidated governed Runtime v3 assembly filler bindings onto the production executor map and deleted the obsolete local echo executor.
- Removed superseded fixture-only topology tests now covered by real production assembly and focused operation/governed-operation tests.
- Measured claimed source and test paths at 2481 physical lines before and 2461 after, net -20.
- Made the default local Runtime v3 adapter state directory stable for the same runtime working directory instead of process/id scoped.
- Added isolated assembly proof for restart restore, timeout, cancellation, duplicate idempotency, malformed request, missing checkpoint, and corrupt checkpoint boundaries.
- Superseded pre-review validation with FastWork target-dir lanes and corrected the net physical LoC measurement to -10.
- Integrated default fresh-executor CheckpointStore restore proof into the production adapter assembly test to avoid parallel default-state cleanup races.
- Updated final typed VPP and PVF evidence to before 2481, after 2449, net -32.
- Marked earlier non-FastWork and intermediate LoC SOR rows as superseded pre-final lifecycle history.
- Integrated default fresh-executor CheckpointStore restore proof into the production adapter assembly test to avoid parallel default-state cleanup races.
- Moved external transport refusal ahead of local timeout/cancel branches and covered timeout/cancel payloads for Provider, ACIP, A2A, and Cloud Bridge.
- Updated final typed VPP and PVF evidence to before 2481, after 2453, net -28.
- Marked earlier non-FastWork and intermediate LoC SOR rows as superseded pre-final lifecycle history.
- Replaced receipt/text-trigger local Agent behavior with typed bounded blake3 and cancellation-aware sleep work, plus canonical ingress proof for real Agent dispatch.
- Replaced Scheduler saturation-by-text behavior with a typed local schedule command that retires each completed job and accepts repeated sequential work beyond four requests.
- Replaced metadata-only checkpointing with state_hex byte persistence, atomic checkpoint writes, restore-time principal verification, and payload hash integrity verification.
- Replaced payload-text cancellation triggers with CancellationToken propagation through OperationalAdapter and in-process Agent execution.
- Removed cwd/temp fallback state creation from production local adapters; callers must provide an explicit absolute state root guarded by writer.lock unique-writer behavior.
- Deleted the superseded fixture-only operations test target and folded real adapter proof into assembly/governed end-to-end tests.
- Skipped completed-idempotency caching for AdmissionClosed cancellation results and added same-idempotency retry proof.
- Replaced the plain writer.lock file with an owner.json directory lock, live-pid stale detection, atomic stale-lock rename recovery, and ownership-checked drop cleanup.
- Changed build_production_operation_executors to return io::Result and updated binary/governed/test callers to handle invalid configured state roots without panic.
- Trimmed duplicate assembly tests while preserving real end-to-end proofs for the five required behaviors and review blockers.
- Replaced duplicate in-flight OnceCell get_or_init waiting with owner-only execution plus watch-based result notification, so duplicate callers honor their own CancellationToken and never initialize work.
- Notified duplicate waiters on owner-side governed admission errors before removing the in-flight record.
- Changed writer-lock acquisition to write owner.json inside a pending directory before publishing writer.lock, and to recover missing or invalid owner metadata as stale partial locks.
- Kept ownership-checked writer lock release, live-pid stale detection, and explicit configured absolute state roots.
- Trimmed the proof harness while retaining end-to-end coverage for real execution, scheduler reuse, checkpoint restore identity/integrity, live cancellation, and safe storage locking.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "assembly"
    ],
    "purpose": "Run the focused assembly test surface for local Runtime v3 adapter behavior.",
    "outcome": "passed",
    "evidence_ref": "runtime-v3-local-adapters-assembly.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict all-target Clippy for the Runtime v3 kernel crate.",
    "outcome": "passed",
    "evidence_ref": "runtime-v3-local-adapters-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "governed_operations"
    ],
    "purpose": "Run the focused governed operations test surface after production executor consolidation.",
    "outcome": "passed",
    "evidence_ref": "runtime-v3-local-adapters-governed.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--numstat",
      "--",
      "adl-runtime-kernel/src/assembly.rs",
      "adl-runtime-kernel/src/governed_operations.rs",
      "adl-runtime-kernel/tests/assembly.rs",
      "adl-runtime-kernel/tests/operations.rs"
    ],
    "purpose": "Record the exact numstat delta for the claimed source and test paths.",
    "outcome": "passed",
    "evidence_ref": "runtime-v3-local-adapters-loc.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "operations"
    ],
    "purpose": "Run the focused operation executor tests after duplicate fixture topology removal.",
    "outcome": "passed",
    "evidence_ref": "runtime-v3-local-adapters-operations.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--target-dir",
      "/Volumes/FastWork/adl-wp-5663/target",
      "--test",
      "assembly"
    ],
    "purpose": "Prove production assembly uses real bounded local adapters, restart-stable checkpoints, failure boundaries, and fail-closed external transports after review fixes.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5663/runtime-v3-local-adapters-assembly.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--target-dir",
      "/Volumes/FastWork/adl-wp-5663/target",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove strict all-target Rust lint cleanliness for the touched Runtime v3 kernel crate after review fixes.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5663/runtime-v3-local-adapters-clippy.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--numstat",
      "--",
      "adl-runtime-kernel/src/assembly.rs",
      "adl-runtime-kernel/src/governed_operations.rs",
      "adl-runtime-kernel/tests/assembly.rs",
      "adl-runtime-kernel/tests/operations.rs"
    ],
    "purpose": "Record physical LoC delta for claimed source and test paths after review fixes: before 2481, after 2471, net -10.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5663/runtime-v3-local-adapters-loc.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--target-dir",
      "/Volumes/FastWork/adl-wp-5663/target",
      "--test",
      "operations"
    ],
    "purpose": "Prove operation executor retry, idempotency, timeout, and failure classes after review fixes.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5663/runtime-v3-local-adapters-operations.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--target-dir",
      "/Volumes/FastWork/adl-wp-5663/target",
      "--test",
      "governed_operations"
    ],
    "purpose": "Prove governed local Runtime v3 restart, checkpoint, lifelog, scheduler, shepherd, cancellation, and shutdown behavior after review fixes.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5663/runtime-v3-local-adapters-governed.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--numstat",
      "--",
      "adl-runtime-kernel/src/assembly.rs",
      "adl-runtime-kernel/src/governed_operations.rs",
      "adl-runtime-kernel/tests/assembly.rs",
      "adl-runtime-kernel/tests/operations.rs"
    ],
    "purpose": "Record final physical LoC delta for claimed source and test paths: before 2481, after 2449, net -32.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5663/runtime-v3-local-adapters-loc.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--numstat",
      "--",
      "adl-runtime-kernel/src/assembly.rs",
      "adl-runtime-kernel/src/governed_operations.rs",
      "adl-runtime-kernel/tests/assembly.rs",
      "adl-runtime-kernel/tests/operations.rs"
    ],
    "purpose": "Record final physical LoC delta for claimed source and test paths: before 2481, after 2453, net -28.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5663/runtime-v3-local-adapters-loc.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--target-dir",
      "/Volumes/FastWork/adl-wp-5663/target",
      "--test",
      "assembly"
    ],
    "purpose": "Prove real Agent execution, scheduler retirement/reuse, checkpoint byte persistence/restore with integrity and identity checks, live cancellation, safe configured storage locking, production assembly wiring, ingress dispatch, and fail-closed external transports.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5663/runtime-v3-local-adapters-assembly.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--target-dir",
      "/Volumes/FastWork/adl-wp-5663/target",
      "--test",
      "governed_operations"
    ],
    "purpose": "Prove governed Runtime v3 restart, checkpoint, lifelog, scheduler, shepherd, cancellation, provider, and shutdown behavior remains green after local adapter correction.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5663/runtime-v3-local-adapters-governed.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--target-dir",
      "/Volumes/FastWork/adl-wp-5663/target",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove strict all-target Rust lint cleanliness for the touched Runtime v3 kernel crate.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5663/runtime-v3-local-adapters-clippy.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--numstat",
      "origin/main",
      "--",
      "adl-runtime-kernel/src/assembly.rs",
      "adl-runtime-kernel/src/bin/adl-runtime-kernel.rs",
      "adl-runtime-kernel/src/governed_operations.rs",
      "adl-runtime-kernel/src/operations.rs",
      "adl-runtime-kernel/tests/assembly.rs",
      "adl-runtime-kernel/tests/operations.rs"
    ],
    "purpose": "Record touched source/test physical LoC delta: before 3796, after 3728, net -68.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5663/runtime-v3-local-adapters-loc.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Prove tracked diff whitespace hygiene after source, test, typed evidence, and retained proof updates.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5663/runtime-v3-local-adapters-diff-check.log"
  },
  {
    "command": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      ".",
      "--request",
      ".csdlc/prepared/issues/5663/validate-real-fixes.json"
    ],
    "purpose": "Run the typed PVF manifest covering real adapter assembly behavior, governed regression, strict Clippy, net-negative LoC, and diff hygiene after the consolidated exact-review blocker fixes.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5663"
  },
  {
    "command": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      ".",
      "--request",
      ".csdlc/prepared/issues/5663/validate-real-fixes.json"
    ],
    "purpose": "Run the typed PVF manifest covering real Agent owner/duplicate cancellation, scheduler reuse, checkpoint byte restore identity/integrity, safe durable storage locking including partial-lock recovery, governed regression, strict Clippy, net-negative LoC, and diff hygiene after the remaining exact-review fixes.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5663"
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
