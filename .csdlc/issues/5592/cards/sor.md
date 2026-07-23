# Structured Output Record

Template: 1.0.0

Issue: 5592

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented signed-policy Runtime v3 Parity-B graph execution through the external production guardian, TLS signed control API, canonical ingress, narrow schema router, and persistent configured executor, with bounded resume, one-shot adaptation, advisory isolation, monotonic cognition gates, authenticated recovery, and truthful feature dispositions.

## Artifacts

- adl-runtime-kernel/src/parity_b.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/parity_b_live_kernel.rs
- adl-runtime-kernel/src/operations.rs
- adl-runtime-kernel/src/parity_b.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/parity_b_live_kernel.rs

## Execution

- Add a production Parity-B operation executor for typed canonical-ingress graph requests
- Retain deterministic loop, checkpoint, restore, idempotency, and tamper-evident evidence identity
- Compose existing signed mutation authority with one-shot consumption and verified rollback
- Reject task-content authority, unsupported subjective claims, hidden-state inference, unbounded curiosity, Freedom Gate denial, and shutdown bypass
- Retain twelve explicit live Runtime v3 or accepted non-authoritative feature dispositions
- Route only the signed Parity-B request schema from the existing Agent operational adapter into one persistent environment-configured ParityBExecutor; retain all other operation behavior unchanged
- Authenticate policy signals and gates with configured Ed25519 authority and reject forged policy before persistent executor initialization
- Halt review-required work, persist shutdown across signed checkpoint restore, validate receipt sequence and evidence-anchor semantics, and preserve remaining iteration/deadline/cancellation budgets
- Compose signed one-shot MutationGate evidence into canonical execution and retain deterministic rollback proof
- Replace in-process guardian claims with an external production guardian, TLS Observatory identity, signed /v1/control submission, canonical ingress result, adversarial rejection, and deterministic terminal identity proof
- Assign every retained feature one evidence-specific live or explicit accepted-boundary disposition without metadata-only live credit

## Validation

[
  {
    "command": [
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-5592/exact-target ruby .csdlc/prepared/issues/5592/run_exact_live_test_lane.rb --lane <each-seven-declared-lanes>",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-5592/full-target cargo test --manifest-path adl-runtime-kernel/Cargo.toml",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-5592/clippy-target cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets --all-features -- -D warnings",
      "cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --all -- --check",
      "cargo tree --locked --manifest-path adl-runtime-kernel/Cargo.toml",
      "bash adl/tools/report_runtime_v3_loc.sh",
      "git diff --check"
    ],
    "purpose": "Prove all seven exact Parity-B identities, the complete 203-test Runtime v3 suite, strict warning-free code, formatting, dependency independence, collision hygiene, and exact budget truth. Runtime v3 is 13,146 physical lines: +937 over the pinned 12,209 baseline and +1,146 over the reviewed target, under the 20,000 safety ceiling but requiring exact review disposition.",
    "outcome": "passed",
    "evidence_ref": "owner:019f836b-dfdb-7b33-8e27-4c9478b75421@working-tree:/Volumes/FastWork/adl-5592/{exact-target,full-target,clippy-target}"
  },
  {
    "command": [
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-5592/exact-final ruby .csdlc/prepared/issues/5592/run_exact_live_test_lane.rb --lane <each-seven-declared-lanes>",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-5592/full-final cargo test --manifest-path adl-runtime-kernel/Cargo.toml",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-5592/guardian-route cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets --all-features -- -D warnings",
      "cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --all -- --check",
      "cargo tree --locked --manifest-path adl-runtime-kernel/Cargo.toml",
      "bash adl/tools/report_runtime_v3_loc.sh",
      "git diff --check"
    ],
    "purpose": "Prove all seven exact Parity-B identities including external guardian-launched TLS signed-control canonical ingress, the complete Runtime v3 suite, strict warning-free code, format/dependency/collision hygiene, and exact budget truth. Runtime v3 is 13,504 physical lines: +1,295 over pinned baseline and +1,504 over reviewed target, under the 20,000 safety ceiling but requiring explicit exact-review exception disposition.",
    "outcome": "passed",
    "evidence_ref": "owner:019f836b-dfdb-7b33-8e27-4c9478b75421@fa7a37551:/Volumes/FastWork/adl-5592/{exact-final,full-final,guardian-route}"
  },
  {
    "command": [
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-5592/integrated cargo test --manifest-path adl-runtime-kernel/Cargo.toml",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-5592/integrated-clippy cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings",
      "git merge --no-edit origin/main",
      "git diff --check"
    ],
    "purpose": "Prove current main integrates without protected-path collision and the complete Runtime v3 suite, guardian black-box path, operations regressions, and strict lint remain green at the integrated head.",
    "outcome": "passed",
    "evidence_ref": "owner:019f836b-dfdb-7b33-8e27-4c9478b75421@05dc14883:/Volumes/FastWork/adl-5592/{integrated,integrated-clippy}"
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
