# ADR 0052: ADL v2 Modular Execution Architecture

- Status: Accepted
- Date: 2026-07-30
- Accepted in: v0.91.8
- Related issues: #5336, #5337, #5338, #5339, #5340, #5345, #5350, #5384
- Related ADRs: ADR 0001, ADR 0021, ADR 0043
- Source evidence:
  - `adl-v2/crates/adl-language/README.md`
  - `adl-v2/crates/adl-compiler/README.md`
  - `adl-v2/crates/adl-engine/README.md`
  - `docs/milestones/v0.91.8/review/V0918_WP14A_PLATFORM_ACCEPTANCE_5384.md`
  - merge commits `860aa9f18`, `fbf96beac`, and `19601faec`

## Context

The incumbent `adl` crate combined parsing, validation, compilation, execution,
providers, runtime behavior, and command handling. That coupling made semantic
changes difficult to isolate and made broad validation the default cost of
small changes.

v0.91.8 implemented a clean-room ADL v2 path and accepted it as part of the
integrated platform baseline.

## Decision

ADL v2 is a one-way modular pipeline:

1. `adl-language` owns strict YAML/JSON parsing, the six source primitives,
   validation, schema generation, and canonical source bytes.
2. `adl-compiler` purely lowers a validated document into an inert,
   deterministic `ExecutionPlan`.
3. `adl-engine` is a pure, bounded, logical-time state machine that consumes an
   execution plan and emits typed effects.
4. Host adapters own I/O, persistence, clocks, providers, tools, and Runtime
   integration.

The language, compiler, and engine crates do not acquire Runtime or C-SDLC
authority.

## Consequences

- Semantic boundaries and validation can remain focused.
- Determinism is testable without network, filesystem, provider, or clock I/O.
- Runtime adapters must explicitly bridge emitted effects to external systems.
- Compatibility behavior must be proved at the boundary rather than preserved
  by copying the incumbent implementation.

## Alternatives Considered

### Refactor the incumbent crate in place

Rejected. It would preserve accidental coupling and make deletion, rollback,
and parity harder to reason about.

### Put execution into the compiler

Rejected. Compilation must remain pure and reviewable.

## Validation Notes

Validate with the six-primitives language tests, deterministic compiler tests,
engine turn/checkpoint/resume tests, exact v1/v2 characterization parity, and
the reversible default-selection proof.

## Non-Claims

- This ADR does not give ADL v2 direct provider, filesystem, or network access.
- This ADR does not authorize deletion of the rollback generation.
