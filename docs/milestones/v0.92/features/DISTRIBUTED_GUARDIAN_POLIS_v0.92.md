# Distributed Guardian and Polis Runtime

## Status

Scheduled in v0.92 as architecture/security gate #5821, implementation
umbrella #5862, and exactly sixteen prepared children #5863 through #5878.
No distributed-completion claim is made until all children are terminal and
WP-04.16 provides real integrated proof.

## Purpose

Extend Guardian ownership across multiple nodes without moving cognition,
governance, identity authority, or certificate lifecycle into an ad hoc network
layer.

## Required Behavior

- Every node has a durable Guardian identity and Guardian remains process 0.
- Guardian mesh and Guardian-to-Runtime control use verified mTLS with separate
  certificate purposes, rotation, revocation, expiry, and audit behavior.
- Enrollment, discovery, membership, failure detection, epochs/leases, and
  fencing prevent two authoritative owners for one Runtime lineage.
- Transport is replaceable; a maintained QUIC/TLS stack provides the first
  implementation rather than custom cryptography or framing.
- Signed capability and resource-weather advertisements drive bounded placement
  without transferring scheduling authority into cognition.
- Snapshot catalog and migration implement prepare, quiesce, checkpoint,
  transfer, validate, fence, activate, commit, and rollback semantics.
- Partitions, stale messages, cloned state, wrong-node identity, wrong trust
  domain, and failed certificate renewal have explicit safe outcomes.
- Topology, certificate health, migration, fencing, and failure causes are
  observable through the versioned API and tracing/Vector path.

## Proof

The architecture issue must freeze trust boundaries, schemas, COTS choices,
failure semantics, and threat model before implementation publication. The
program then requires real multi-node membership, partition, fencing,
migration, rollback, certificate-rotation, recovery, and relocation evidence.
WP-04.16 #5878 exclusively owns module registration and the production
Guardian/kernel integration proof, including authenticated API/WSS and
digest-bound native macOS, Linux, and Windows receipts. WP-14 #5832 remains
blocked until WP-04-IMP #5862 has terminal integrated output.

## Non-Goals

- No plaintext or verification-disabled mode.
- No network substrate becomes polis authority.
- No Runtime v2 modification or deletion.
- No claim that a single-node restart proves distributed survival.
