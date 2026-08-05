## Summary
Implement WP-13A as real Runtime v3 behavior, not another planning queue.

## Required Outcome
Deliver working evaluation bindings, durable adaptation deltas, policy-governed graph-change proposals, accepted and rejected mutation paths, and deterministic replay proof on the WP-01-qualified Runtime v3 loop substrate.

## Dependencies
- WP-01 issue #5817
- WP-13 issue #5830
- merged #5104 historical semantic input
- current Runtime v3 loop qualification retained by #5817

## Acceptance Criteria
- The Adaptive Learning DAG executes real `adl-runtime-kernel` behavior at the exact reviewed revision.
- Every accepted state or graph change is policy-authorized, durable, and deterministically replayable.
- Evaluation evidence, state hashes, rationale, policy decisions, and graph deltas remain inspectable.
- Rejected mutation, forged history, discontinuous resume, invalid binding, unbounded recurrence, missing evidence, and unauthorized mutation are proven negative cases.
- Native proof is source-SHA, exact-argv, runner-identity, output-digest, and artifact bound.
- The implementation PR includes `Closes #5831`.

## Non-goals
- No Runtime v2 fallback, ungoverned self-modification, adjacent WP absorption, or historical evidence rewrite.

<!-- csdlc-github-operation:v092-wp13a-dependency-reconciled -->
