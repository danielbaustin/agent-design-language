# CT Demo 001 Transition DAG

## Purpose

This tracked DAG fixture shows the first bounded C-SDLC transition shape for
`v0.91.3`. It defines which steps stay serial, which steps can be sharded, and
where synchronization barriers must halt progress before review, merge
readiness, and closeout.

## Transition Identity

- transition id: `cts.v0_91_3.issue_3200.ct_demo_001`
- issue lineage: `WP-02` manifest fixture, `WP-03` tracked card bundle, and
  `WP-04` DAG/shard proof packet
- proof scope: transition topology and coordination rules only

## Serial Nodes

| Node | Purpose | Exit condition |
| --- | --- | --- |
| `serial.issue_contract_ready` | Source issue, cards, and manifest target paths are known. | Inputs are bound and repo-relative. |
| `serial.transition_manifest_validated` | The manifest schema and seed roles validate. | `WP-02` fixture passes validator checks. |
| `serial.review_synthesis_ready` | Shard outputs are converged into one reviewable packet. | All shard handoff contracts are satisfied. |
| `serial.merge_readiness_gate` | Governance checks run after shard convergence. | Review, CI, and branch truth are complete. |
| `serial.closeout_ready` | Final outcome truth can be recorded. | Merge/closure state and evidence references agree. |

## Shard Nodes

| Node | Owner role | Allowed write surface | Required handoff |
| --- | --- | --- | --- |
| `shard.packet_docs` | `implementation_owner.docs_packet` | `docs/milestones/v0.91.3/review/transition_dag/**` and `docs/milestones/v0.91.3/features/TRANSITION_DAG_AND_SHARD_COORDINATION.md` | DAG packet with explicit node/barrier semantics |
| `shard.validator_contracts` | `implementation_owner.validation_owner` | `adl/src/cognitive_transition_schema.rs`, `adl/tools/validate_transition_dag_packet.py`, `adl/tools/test_transition_dag_packet.sh` | focused proof that tracked packet exists and satisfies required sections |

## Barrier Nodes

| Node | Trigger | Required evidence |
| --- | --- | --- |
| `barrier.review_barrier` | Both shards finish and freeze their declared interfaces. | packet docs and validator/test surface agree on file names and required semantics |
| `barrier.merge_readiness_barrier` | Review findings are addressed and validations pass. | focused proof commands plus truthful `SRP`/`SOR` |
| `barrier.closeout_barrier` | PR state and main-branch truth converge. | merged/closed issue truth, final `SOR`, and durable packet references |

## Edge Summary

1. `serial.issue_contract_ready -> serial.transition_manifest_validated`
2. `serial.transition_manifest_validated -> shard.packet_docs`
3. `serial.transition_manifest_validated -> shard.validator_contracts`
4. `shard.packet_docs -> barrier.review_barrier`
5. `shard.validator_contracts -> barrier.review_barrier`
6. `barrier.review_barrier -> serial.review_synthesis_ready`
7. `serial.review_synthesis_ready -> barrier.merge_readiness_barrier`
8. `barrier.merge_readiness_barrier -> serial.merge_readiness_gate`
9. `serial.merge_readiness_gate -> barrier.closeout_barrier`
10. `barrier.closeout_barrier -> serial.closeout_ready`

## Interface Freeze Rules

- `shard.packet_docs` cannot rename packet files after the review barrier
  without updating validator-backed references in the same change.
- `shard.validator_contracts` cannot widen the packet contract beyond the files
  declared in this DAG and shard plan.
- Neither shard may edit unrelated milestone features or sprint-state records.
- Any additional write surface requires an explicit replan before execution
  continues.

## Coordination Metrics Split

This fixture defines the timing boundary used later by the first proof:

- coordination latency:
  - lifecycle routing
  - shard assignment
  - barrier waiting time
  - review gating
  - merge-readiness gating
- implementation time:
  - shard-local authoring
  - shard-local validation
  - bounded artifact repair inside declared write surfaces

`WP-04` defines the split. `WP-09` is the milestone slice that measures it.
