# Decisions - ADL v0.4

## Metadata
- Milestone: `v0.4`
- Version: `0.4`
- Date: `2026-02-18`
- Owner: Daniel Austin

## Purpose
Record the decisions that shaped shipped v0.4 behavior.

## Decision Log

| ID | Decision | Status | Rationale | Alternatives | Impact | Link |
|---|---|---|---|---|---|---|
| D-01 | Build and execute a validated `ExecutionPlan` graph before runtime execution. | accepted | Keep execution deterministic and auditable. | Implicit runtime ordering only. | Enabled DAG validation and deterministic scheduling. | #298, [#299](https://github.com/danielbaustin/agent-design-language/pull/299) |
| D-02 | Use bounded fork execution in runtime. | accepted | Real concurrency with constrained resource usage. | Unbounded thread fan-out. | Predictable concurrency behavior under load. | #297, [#300](https://github.com/danielbaustin/agent-design-language/pull/300) |
| D-03 | Use deterministic join barrier semantics. | accepted | Join must wait for all branches and preserve stable outputs. | Merge-by-completion-order joins. | Stable artifacts and replay-friendly traces. | #296, [#301](https://github.com/danielbaustin/agent-design-language/pull/301) |
| D-04 | Route runtime concurrency through `ExecutionPlan` + structural fork/join dependencies. | accepted | Keep runtime behavior aligned with explicit plan semantics. | Partial test-only validation. | Real engine wiring for fork/join behavior. | #304, [#305](https://github.com/danielbaustin/agent-design-language/pull/305) |
| D-05 | Keep deterministic retry/on-error semantics from v0.3 unchanged in v0.4 milestone. | accepted | Avoid regressions while adding concurrency runtime behavior. | Redesign retry semantics during v0.4. | Reduced migration risk and kept CI stable. | #256, #304 |
| D-06 | Ship no-network demo harness with deterministic mock provider. | accepted | Demonstrate runtime concurrency without external dependencies. | Network-backed demo workflows. | Reproducible demos and release-quality UX. | #306, [#307](https://github.com/danielbaustin/agent-design-language/pull/307) |
| D-07 | Keep runtime concurrency limit fixed at engine level (`MAX_PARALLEL=4`) for v0.4. | accepted | Stabilize behavior first; defer configurability to next milestone. | Expose runtime parallelism knobs immediately. | Predictable behavior now; configurable parallelism remains roadmap work. | #304, #306 |

## Open Questions
- Should configurable runtime parallelism be exposed in schema/runtime in v0.5? (Owner: Daniel) (Tracking: next milestone planning)
- Should join/parallel markers be expanded in trace schema for external tooling consumers? (Owner: Daniel) (Tracking: v0.5 design)
