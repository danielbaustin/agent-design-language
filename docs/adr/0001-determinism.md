# ADR 0001: Deterministic Execution Semantics

## Status
Accepted (v0.5)

## Decision
ADL runtime execution must remain deterministic for identical inputs and
configuration:
- ready-step ordering is lexicographic by full step id
- bounded execution preserves stable output ordering
- plan generation is deterministic for canonicalized workflow/pattern inputs

## Rationale
Determinism improves reproducibility, debugging, and trust in CI/regression
signals. It is foundational for trace comparison and replay-oriented workflows.

## Alternatives Considered
- Preserve declaration-order execution only
- Opportunistic scheduler ordering based on runtime readiness timing
- Max-throughput scheduling without deterministic constraints

## Consequences
- Runtime and tests enforce deterministic ordering invariants explicitly.
- Some throughput optimizations are deferred when they would break stability.
- Documentation and demos must present deterministic behavior as a contract.
