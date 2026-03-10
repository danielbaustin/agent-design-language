# Gödel Schema Delivery Order and Acceptance Boundaries — v0.8

This document defines the canonical delivery sequence and acceptance boundaries for the v0.8 Gödel schema/spec surfaces.

It is a planning/control artifact only. It does not introduce runtime implementation.

## Purpose
- Prevent schema drift across independently implemented issues.
- Make issue-level completion criteria explicit.
- Separate docs/schema completion from later runtime/demo integration work.

## Canonical Delivery Order

| Order | Surface | Issue | Depends on |
|---|---|---|---|
| 1 | ExperimentRecord schema v1 | #609 | v0.8 docs baseline (#662) |
| 2 | Canonical Evidence View v1 | #610 | v0.8 docs baseline (#662) |
| 3 | Mutation format v1 | #611 | v0.8 docs baseline (#662) |
| 4 | EvaluationPlan v1 | #612 | #609, #610, #611 |
| 5 | Gödel workflow template v1 | #613 | #609, #610, #611, #612 |
| 6 | ObsMem indexing surfaces (run summary + experiment index entry) | #614 | #609, #610, #613 |
| 7 | Deterministic demo (failure -> hypothesis -> experiment) | #615 | #609 through #614 |
| 8 | Gödel docs integration pass | #616 | #609 through #615 |
| 9 | ExperimentRecord enhancement (`improvement_delta`, `experiment_seed`) | #683 | #609, #614, #615 |

## Acceptance Boundaries by Layer

### Layer A: Core schema/spec surfaces (#609-#612)
Issue is complete when:
- Canonical markdown spec exists under `docs/milestones/v0.8/`.
- Schema/spec JSON artifact exists where applicable.
- At least one deterministic example artifact exists where applicable.
- Field semantics and deterministic constraints are explicit.

Not required for completion:
- Runtime parser/executor integration.
- Autonomous behavior or adaptive policy logic.

### Layer B: Workflow composition surface (#613)
Issue is complete when:
- Canonical stage order and stage IO contracts are explicit.
- Workflow template references Layer A schemas by stable artifact surfaces.
- Deterministic ordering constraints are documented.

Not required for completion:
- Runtime execution of all workflow stages.

### Layer C: Indexing and retrieval handoff surface (#614)
Issue is complete when:
- Indexing artifact contracts are explicit for run summary and experiment-index entries.
- Linkage from failure class -> hypothesis -> experiment outcome is represented.
- Replay-safe fields (`experiment_seed`) and ranking support fields (`improvement_delta`) are explicitly accounted for.

Not required for completion:
- Production indexing engine behavior.

### Layer D: Demo and docs integration surfaces (#615, #616)
Issue is complete when:
- Deterministic demo/docs artifacts reflect the defined schema surfaces.
- Cross-reference consistency across v0.8 docs is maintained.

Not required for completion:
- Full autonomous Gödel loop implementation.

### Layer E: Enhancement boundary (#683)
Issue is complete when:
- `ExperimentRecord` enhancement fields are added with deterministic semantics.
- Schema/doc/example stay in lockstep.

Not required for completion:
- Automated mutation acceptance, online learning, or policy self-modification.

## Schema Completion vs Runtime Completion

Schema/spec completion means:
- The artifact contract is canonical, deterministic, and reviewable.

Runtime completion means:
- The runtime consumes and enforces that contract during execution.

For v0.8, #665 governs schema delivery ordering and acceptance boundaries only; runtime closure remains in downstream issues.

## Deterministic Ordering Rule
When dependencies are otherwise tied:
1. Lower work package ID first.
2. Lower issue number next.
3. Lexicographic artifact filename tie-break.

## Non-goals
- Redefining schema semantics already accepted in #609-#616/#683.
- Expanding scope into new v0.8 feature surfaces.
- Introducing runtime behavior commitments not already tracked by implementation issues.
