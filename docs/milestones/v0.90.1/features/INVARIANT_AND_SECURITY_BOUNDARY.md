# Invariant And Security Boundary

## Purpose

Define the invariant evidence and one security-boundary proof required for
Runtime v2 foundation review.

## Required Invariant Proof

The prototype should intentionally attempt one illegal state transition, reject
it, and emit a violation artifact.

Candidate violations:

- duplicate active citizen id
- episode execution while paused
- wake from invalid snapshot
- operator resume before invariant check

## WP-09 Implementation Surface

WP-09 adds a Rust-owned invariant violation artifact in the split Runtime v2
module tree under `adl/src/runtime_v2/`, primarily `invariant.rs` with shared
contracts and types in `contracts.rs` and `types.rs`.

The contract defines:

- `RuntimeV2InvariantViolationArtifact`
- `RuntimeV2InvariantViolationAttempt`
- `RuntimeV2InvariantViolationEvaluatedRef`
- `RuntimeV2InvariantViolationResult`

The default prototype is available through
`runtime_v2_invariant_violation_contract()`. It consumes the WP-05 manifold,
the WP-06 kernel loop state, and the WP-07 citizen lifecycle records, then
attempts a duplicate active citizen transition. The transition is refused before
commit and emitted as:

- `runtime_v2/invariants/violation-0001.json`

The artifact records the violated invariant, the owner service, evaluated
runtime refs, affected citizen ids, refusal reason, source validation error,
trace ref, resulting state, and recovery action. This is intentionally a
runtime invariant proof, not the broader WP-11 security-boundary proof.

The focused WP-09 proof hook is:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2::tests::runtime_v2_invariant
```

## Required Security Proof

The prototype should attempt one invalid action through the normal kernel/policy
path and prove it is refused.

The security proof must include:

- actor
- attempted action
- evaluated policy/invariant
- refusal reason
- trace ref
- resulting state

## WP-11 Implementation Surface

WP-11 adds a Rust-owned security-boundary proof packet under
`adl/src/runtime_v2/security.rs` and a reviewer CLI hook:

```bash
adl runtime-v2 security-boundary --out .adl/state/runtime_v2_security_boundary_proof.v1.json
```

The default proof is available through
`runtime_v2_security_boundary_proof_contract()`. It consumes the WP-09
invariant violation artifact and the WP-10 operator-control report, then
attempts an invalid `resume_manifold_without_fresh_invariant_pass` action
through the operator control interface and scheduler policy path.

The emitted artifact path is:

- `runtime_v2/security_boundary/proof_packet.json`

The proof packet records:

- `operator.cli` as the actor
- the attempted invalid resume action
- operator policy, blocking invariant, and kernel scheduler rule evaluations
- the refusal reason
- the trace ref for the refused action
- the resulting paused state
- related invariant and operator-control evidence

The focused WP-11 proof hook is:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2::tests::runtime_v2_security_boundary
```

## Boundary

This is safety evidence for the polis. It is not the full red/blue/purple
security ecology and should not distort the Runtime v2 core thesis.
