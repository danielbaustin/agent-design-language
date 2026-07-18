# Structured Task Prompt

Template: 1.0.0

Issue: 5410

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Resolve the four open #5174 runtime findings routed to #5410 without widening into #5409 or later Runtime v3 remediation issues.

## Deliverables

- Live registry-based Runtime v3 composition root
- Signed live continuity restore and shutdown path
- Bounded qualified Chronosense adapter
- Reproducible current inventory and historical count labeling
- Focused and full Runtime v3 validation evidence

## Acceptance

1. Live kernel is assembled through the component registry with required service health and lifecycle proof
2. Restored continuity is cryptographically authenticated against identity and lineage
3. Time remains degraded until real synchronization qualification succeeds
4. Historical review snapshots are labeled and current counts are generated reproducibly
5. Runtime v3 remains below 12000 Rust implementation LoC and 1000 tests
6. Independent exact-revision review has no unresolved actionable findings

## Dependencies

- #5406 terminal records remediation
- #5174 review packet

## Inputs

- docs/reviews/v0.91.7/remaining-sprints-5403/RUNTIME_V3_PARITY_REVIEW_5174.md
- adl-runtime-kernel/src/topology.rs
- adl-runtime-kernel/src/continuity.rs
- adl-runtime-kernel/src/operations.rs
- adl-runtime-kernel/src/reasoning.rs
- adl-runtime-kernel/src/governance.rs

## Non Goals

- Runtime v2 changes or decommission
- Production cloud/provider credentials
- Distributed Runtime v3 architecture
- Changes to #5409 protected files
