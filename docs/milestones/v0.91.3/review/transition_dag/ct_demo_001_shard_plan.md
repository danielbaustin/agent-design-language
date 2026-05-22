# CT Demo 001 Shard Plan

## Purpose

This shard plan gives the first bounded ownership model for the `ct_demo_001`
transition fixture. It is intentionally narrow: two shards, one review
barrier, one merge-readiness barrier, and one closeout barrier.

## Shards

| Shard | Owner role | Allowed write surfaces | Forbidden writes |
| --- | --- | --- | --- |
| `packet_docs` | `implementation_owner.docs_packet` | `docs/milestones/v0.91.3/review/transition_dag/**`; `docs/milestones/v0.91.3/features/TRANSITION_DAG_AND_SHARD_COORDINATION.md`; `docs/milestones/v0.91.3/DEMO_MATRIX_v0.91.3.md`; `docs/milestones/v0.91.3/FEATURE_PROOF_COVERAGE_v0.91.3.md` | Rust source, unrelated review packets, sprint records |
| `validator_contracts` | `implementation_owner.validation_owner` | `adl/src/cognitive_transition_schema.rs`; `adl/tools/validate_transition_dag_packet.py`; `adl/tools/test_transition_dag_packet.sh` | milestone feature docs outside the declared DAG packet scope |

## Interface Freeze Rules

- Packet filenames are fixed for this proof:
  - `ct_demo_001_transition_dag.md`
  - `ct_demo_001_shard_plan.md`
  - `TRANSITION_DAG_PROOF_PACKET_v0.91.3.md`
- The validator contract must accept only the tracked packet root and required
  section set declared by this issue.
- Shards may refine wording inside their allowed surfaces, but they may not
  expand ownership boundaries without an explicit replan.

## Handoff Contracts

### `packet_docs`

- must name serial, shard, and barrier nodes explicitly
- must describe coordination latency separately from implementation time
- must record the barrier vocabulary needed for later evidence and first-proof
  work

### `validator_contracts`

- must fail closed when required packet files or required sections are missing
- must prove the `WP-02` manifest fixture still points at real `WP-04` packet
  paths
- must remain focused on this packet contract and avoid inventing later
  C-SDLC enforcement

## Barrier Contracts

### `review_barrier`

- both shards complete their declared outputs
- packet docs and validator expectations agree on file names and required
  semantics

### `merge_readiness_barrier`

- focused validation passes
- review has no unresolved blocking findings
- `SRP` and `SOR` truth match the actual branch state

### `closeout_barrier`

- PR merge or intentional closure is recorded truthfully
- final `SOR` reflects the durable packet and validation paths

## Coordination Metrics Split

This plan fixes the metrics vocabulary for later proof:

- coordination latency:
  - shard assignment time
  - barrier waiting time
  - review routing time
  - merge-readiness routing time
- implementation time:
  - docs packet authoring
  - validator/test authoring
  - focused proof execution

No measured values are claimed here; only the reporting boundary is fixed.
