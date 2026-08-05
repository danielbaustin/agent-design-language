# Issue 5831 Design: Runtime v3 Adaptive Learning DAG

## Outcome And Sources

Implement WP-13A's evaluated, policy-governed graph-change path described in `docs/milestones/v0.92/features/ADAPTIVE_LEARNING_DAG_v0.92.md`. Consume the requalified loop/replay substrate in `adl/src/runtime_v2/loop_runtime.rs`, reasoning graph in `adl/src/runtime_v2/reasoning_graph.rs`, bridge in `reasoning_runtime_bridge.rs`, governed-learning boundary, and `.csdlc/evidence/5817/prerequisite-and-loop-runtime-requalification.md`.

## Owned Surface

Candidate protected paths are narrowly named adaptive-learning modules under `adl/src/runtime_v2/`, Runtime v3 integration under `adl/src/cli/runtime_v3_cmd.rs` only if required, corresponding tests/fixtures, the feature contract, and `.csdlc/evidence/5831/`. Existing loop and graph schemas are changed only through explicit versioning.

## Contract

Evaluation bindings connect one loop iteration to feedback source, confidence, evidence refs, and policy context. An adaptation delta records before/after state hashes, rationale, rollback note, and canonical order. Graph changes begin as proposals; policy emits accepted or rejected disposition before mutation. Durable history links loop events, evaluation, state delta, proposal, policy decision, graph delta, and replay evidence.

## Dependencies And Invariants

WP-01/#5818, WP-13/#5830, merged #5104 semantics, and current Runtime v3 qualification must be verified. Same durable inputs replay identically. Missing evidence never becomes feedback; resume requires prefix continuity; rejected proposals cannot mutate state; bounds and cancellation remain Runtime authority.

## Validation And Rollback

Focused unit/integration tests prove accepted and rejected mutation paths plus deterministic replay. Negative tests cover forged history, substituted state, invalid graph binding, discontinuous resume, unbounded recurrence, missing evidence, unauthorized mutation, and rollback mismatch. A Runtime v3 integration lane proves the branch-built path. Rollback replays the recorded inverse or restores the prior graph/state hashes without deleting the rejected proposal history.

## Non-Goals

Unconstrained self-modification, autonomous retraining, hidden model-memory mutation, policy bypass, production autonomous learning, consciousness/personhood claims, and v0.94 signed trace completion are excluded.
