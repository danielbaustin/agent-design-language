# Structured Task Prompt

Template: 1.0.0

Issue: 5831

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver only the WP-13A Runtime v3 adaptive-learning graph path, focused integration, fixtures, negative replay proof, and retained evidence.

## Deliverables

- Evaluation and adaptation-delta contracts
- Graph proposal and accepted/rejected policy-decision path
- Durable history, deterministic replay, and rollback/inverse record
- Focused Runtime v3, negative replay, and bounded-resource evidence

## Acceptance

1. The WP-13A DAG durably links loop event, evaluation, evidence-backed state delta, graph proposal, policy decision, accepted or rejected mutation, replay, and rollback/inverse record.
2. WP-01/#5818, WP-13/#5830, merged #5104 semantics, and current Runtime v3 requalification are verified before implementation.
3. Changes remain in narrow adaptive-learning modules, versioned shared schemas, tests/fixtures, optional Runtime v3 command integration, and .csdlc/evidence/5831/.
4. Same durable inputs replay to identical proposal, policy disposition, state/graph hashes, and retained exact-revision history.
5. Missing/forged evidence, substituted state, invalid graph binding, discontinuous resume, unbounded recurrence, unauthorized mutation, rejected-proposal mutation, and rollback mismatch fail closed.
6. One bounded exact-head SRP review records no unresolved actionable findings.
7. The implementation PR targets the intended base and includes Closes #5831 without claiming completion of downstream Birthday work.

## Dependencies

- WP-01 / issue #5818 terminal proof
- WP-13 / issue #5830 terminal proof
- Merged issue #5104 adaptive-learning semantics
- Current Runtime v3 prerequisite and loop-runtime requalification

## Inputs

- docs/milestones/v0.92/features/ADAPTIVE_LEARNING_DAG_v0.92.md
- adl/src/runtime_v2/loop_runtime.rs
- adl/src/runtime_v2/reasoning_graph.rs
- adl/src/runtime_v2/reasoning_runtime_bridge.rs
- adl/src/runtime_v2/ governed-learning boundary
- adl/src/cli/runtime_v3_cmd.rs
- .csdlc/evidence/5817/prerequisite-and-loop-runtime-requalification.md

## Non Goals

- Unconstrained self-modification, autonomous retraining, hidden model-memory mutation, or policy bypass
- Production autonomous learning, consciousness, personhood, or v0.94 signed-trace completion
- Changing loop, graph, or Runtime schemas without explicit versioning
