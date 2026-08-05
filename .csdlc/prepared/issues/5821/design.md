# Issue 5821 Design: Distributed Guardian And Polis Runtime Program

## Outcome And Boundary

Issue 5821 is the WP-04 program gate and integration owner. It first freezes a
reviewed distributed architecture and threat model, then coordinates the
declared 16 bounded child issues, and finally proves integrated multi-node
Guardian/polis behavior. Completion requires every child to be concrete,
nonduplicative, merged, terminal, and represented in real production-path
membership, fencing, migration, rollback, certificate, and recovery evidence.

This issue does not collapse the 16 children into one code change. It owns the
program contract, architecture/security gate, child ledger, integration proof,
and final reconciliation. Individual children retain implementation authority.

## Source Baseline

- `docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md`
  defines Guardian identity, mTLS, membership, leases/epochs, fencing,
  placement, migration, rollback, and observability requirements.
- `adl-runtime/src/guardian.rs`, `topology.rs`, `networking.rs`, and `acip.rs`
  are current single-node and communication ownership inputs.
- `adl-runtime-kernel/src/topology.rs`, `continuity.rs`, `durable_state.rs`,
  `control.rs`, `observability.rs`, and `weather.rs` own Runtime-side topology,
  state, control, telemetry, and resource context.
- ADR 0054 preserves the Guardian-owned Runtime v3 kernel/API boundary; prior
  Runtime/polis architecture and custody evidence are historical inputs, not
  current distributed completion proof.
- The v0.92 wave declares WP-03 as the hard dependency and exactly 16 child
  issues as the bounded program denominator.

## Architecture And Security Gate

Before child implementation credit, the program must publish a reviewed
contract for node and Guardian identity, separate certificate purposes,
enrollment, discovery, membership, authenticated transport, epochs/leases,
fencing, capability/resource advertisements, snapshot catalog, migration state
machine, audit events, and operator projections. The threat model must cover
partition, replay, stale lease, cloned checkpoint, wrong node/trust domain,
certificate compromise/expiry, rollback failure, and split-brain activation.

Guardian remains process 0 on every node. A maintained QUIC/TLS stack is the
first transport; no custom cryptography or framing. Placement consumes signed
capability and resource-weather advertisements but does not transfer polis,
governance, cognition, or identity authority into the mesh.

Migration follows prepare, quiesce, checkpoint, transfer, validate, fence,
activate, commit. Source authority is retained until target validation and
fencing succeed. Any ambiguity aborts or rolls back to one authoritative owner.
Topology and migration status are projected through versioned Runtime APIs and
the existing tracing/Vector channel without exposing private state or keys.

Candidate ownership spans narrowly allocated files under `adl-runtime/`,
`adl-runtime-kernel/`, protocol/schema surfaces, architecture/threat-model docs,
and issue evidence. Each child must declare a disjoint protected path set; the
umbrella does not preclaim all product paths.

## Invariants And Failure Semantics

- Exactly one authoritative owner exists for a Runtime lineage at any epoch.
- All node/control traffic is mutually authenticated; no insecure mode.
- Replay, stale epoch, cloned state, and wrong-domain messages fail closed.
- Failed migration never activates two targets or discards the recoverable
  source checkpoint.
- Network partitions degrade placement/relocation but do not grant authority.
- Fixtures, receipts, opened child issues, and demo mode receive no completion
  credit for production paths.
- Runtime v2 remains unchanged and is not a rollback target.

## Dependencies And Child Program

WP-03 issue 5820 must land stable ingress, lifecycle, state, and readiness
contracts before this program executes. The 16-child ledger must map each
required behavior and threat to one owner, dependency, path set, proof lane,
and terminal receipt. Missing, duplicate, or overlapping ownership blocks the
architecture gate. WP-14 issue 5832 waits for this integrated substrate.

## Validation Boundary

Gate proof validates architecture, threat model, child denominator, and path
allocation. Deterministic lanes cover schema/epoch/fencing/migration state
machines and adversarial messages. Real multi-node lanes exercise membership,
partition, failover, migration, rollback, certificate rotation/revocation, and
recovery over production transport. Platform coverage includes macOS/Linux and
the declared native Windows posture. Final proof reconciles all 16 terminal
children to one exact integration revision.

## Rollback

Rollback disables distributed enrollment/placement, fences unfinished target
epochs, restores the last validated source checkpoint under its prior Guardian,
and verifies single-node WP-03 operation. It retains audit evidence and never
uses plaintext, Runtime v2, or dual authority as a fallback.

## Non-Goals

- v0.93 constitutional governance or polis policy redesign.
- Custom cryptography, custom transport framing, or cognition-owned placement.
- Runtime v2 modification/deletion.
- Treating architecture prose or 16 opened issues as implementation complete.
- Absorbing WP-14 protocol reconciliation or WP-17 identity migration policy.
