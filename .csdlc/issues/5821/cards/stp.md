# Structured Task Prompt

Template: 1.0.0

Issue: 5821

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Freeze the distributed Guardian trust, membership, lease/epoch, fencing, placement, migration, rollback, observability, and threat contracts; route exactly 16 nonduplicative children; then prove and reconcile one real integrated multi-node program.

## Deliverables

- Reviewed architecture decision packet and threat model before implementation credit
- Exact 16-row child ledger mapping requirement, dependency, owner, paths, proof, review, PR, and terminal receipt
- Maintained QUIC/TLS membership plus epoch/lease/fencing and migration state-machine implementation across child issues
- Real integrated partition, fencing, migration, rollback, certificate rotation/revocation, recovery, and relocation evidence

## Acceptance

1. A reviewed architecture and threat model freeze identity, trust, COTS transport, schemas, membership, epochs/leases, fencing, placement, migration, rollback, and observability before implementation credit
2. Exactly 16 concrete nonduplicative child issues map every required behavior and threat to owner, paths, dependencies, proof, review, PR, and terminal receipt
3. All 16 children are implemented, reviewed, merged, terminal, and integrated without hidden red or deferred work
4. Real production transport proves multi-node membership, partition, failure detection, epochs/leases, and fencing with one authoritative owner
5. Real migration proves prepare, quiesce, checkpoint, transfer, validate, fence, activate, commit, and safe rollback including relocation failures
6. mTLS enrollment, purpose separation, rotation, revocation, expiry, wrong-node, wrong-domain, replay, and stale-message cases fail safely
7. Topology, certificate health, migration, fencing, and failure causes are visible through versioned API and tracing/Vector without private-state exposure
8. One exact-revision review has no unresolved actionable findings and reconciles the entire child program

## Dependencies

- WP-03 issue 5820 terminal with stable ingress, lifecycle, state, and readiness contracts
- Exactly 16 concrete distributed-runtime child issues with no duplicate ownership
- Reviewed COTS transport and cryptographic boundary
- Sprint 5855 serial gate before WP-14 issue 5832

## Inputs

- docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md
- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
- docs/adr/0054-runtime-v3-guardian-owned-kernel-and-api-boundary.md
- adl-runtime/src/guardian.rs
- adl-runtime/src/networking.rs
- adl-runtime/src/topology.rs
- adl-runtime/src/acip.rs
- adl-runtime-kernel/src/topology.rs
- adl-runtime-kernel/src/continuity.rs
- adl-runtime-kernel/src/durable_state.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/observability.rs

## Non Goals

- v0.93 constitutional governance or polis policy redesign
- Custom cryptography, custom framing, or cognition-owned placement
- Runtime v2 modification or fallback
- Treating architecture prose, fixtures, receipts, demo mode, or opened children as completion
- Absorbing WP-14 protocol reconciliation or WP-17 identity migration policy
