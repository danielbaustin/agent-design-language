# Structured Task Prompt

Template: 1.0.0

Issue: 5412

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair the four #5247 findings without changing Runtime v3 cutover defaults or absorbing Observatory/live-parity work from #5413.

## Deliverables

- Checkpoint signing and verification
- Lineage-bound projection API
- Scheduled/release soak runner and retained contract
- Reviewed reproducible LoC disposition
- Focused negative and compatibility tests

## Acceptance

1. Full checkpoint contents identity sequence head and lineage are authenticated
2. Private projection requires a verified record accepted into the exact lineage
3. A bounded release or scheduled lane executes the real 100-cycle soak
4. Source LoC is below 10K or a reviewed exception and reduction plan is retained

## Dependencies

- #5247 review evidence
- ed25519-dalek authority patterns
- Runtime v3 validation lane inventory

## Inputs

- docs/reviews/v0.91.7/remaining-sprints-5403/RUNTIME_V3_READINESS_REVIEW_5247.md
- adl-runtime-kernel/src/identity_memory.rs
- adl-runtime-kernel/src/private_state.rs
- adl-runtime-kernel/tests/guardian_soak.rs
- docs/architecture/runtime_v3_parity_matrix.v1.json

## Non Goals

- Runtime v2 deletion
- Default Runtime v3 cutover
- Observatory browser/feed work owned by #5413
- Reconstructing historical six-card bundles owned by #5406
