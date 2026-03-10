# Gödel Workflow / ObsMem / Demo Handoff Boundaries — v0.8

This document defines the canonical handoff boundaries between:
- #613 Gödel workflow template
- #614 ObsMem indexing surfaces
- #615 deterministic demo flow

It is a boundary contract for v0.8 planning and implementation sequencing. It does not introduce runtime behavior.

## Purpose
- Make layer ownership explicit.
- Prevent schema/runtime/demo responsibilities from being mixed.
- Provide a deterministic handoff map for contributors implementing adjacent issues.

## Layer Ownership

| Layer | Primary Issue | Owns | Must Not Own |
|---|---|---|---|
| Workflow template layer | #613 | Stage sequence and stage IO contracts (`failure -> hypothesis -> mutation -> experiment -> evaluation -> record`) | Indexing internals, retrieval ranking, demo orchestration policy |
| Indexing surface layer | #614 | `run_summary` and `experiment_index_entry` schema/spec contracts | Workflow stage logic, demo narrative behavior |
| Demo integration layer | #615 | Deterministic demonstration artifacts and runnable integration narrative | Redefining schema contracts from #609-#614 |

## Canonical Handoff Points

### Handoff A: Workflow template -> artifact surfaces (#613 -> #609/#610/#611/#612)
Template outputs and references:
- `ExperimentRecord` (`#609`)
- `Canonical Evidence View` (`#610`)
- `Mutation` (`#611`)
- `EvaluationPlan` (`#612`)

Boundary:
- #613 defines stage-level contracts and references.
- #613 does not redefine schema internals.

### Handoff B: Workflow artifacts -> indexing surfaces (#613/#609 -> #614)
Indexing-consumable surfaces:
- run summary view
- experiment index entry view
- failure class / hypothesis / outcome linkage

Boundary:
- #614 defines normalized indexing contracts.
- #614 does not redefine workflow-stage order from #613.

### Handoff C: Indexed surfaces + workflow references -> demo artifacts (#614/#613 -> #615)
Demo-consumed surfaces:
- deterministic failure signal
- deterministic hypothesis artifact
- deterministic experiment proposal artifact
- query/result examples mapped to indexed surfaces

Boundary:
- #615 demonstrates and validates integration behavior.
- #615 does not alter schema ownership from #609-#614.

## Acceptance Boundaries

### #613 completion boundary
Complete when:
- Stage order and stage IO are explicit and deterministic.
- Required upstream schema references are clear.

Not required:
- Runtime execution engine changes.

### #614 completion boundary
Complete when:
- Indexing schema/spec artifacts are explicit and deterministic.
- Cross-artifact linkage fields are present for retrieval.

Not required:
- Production indexing engine implementation.

### #615 completion boundary
Complete when:
- Deterministic demo artifacts illustrate failure -> hypothesis -> experiment handoff using canonical surfaces.
- Demo documentation references canonical artifacts and expected flow.

Not required:
- Autonomous Gödel loop implementation.

## Deterministic Interface Rules
1. Handoff data must be artifact-based and repo-relative.
2. Cross-layer references must use stable identifiers/paths.
3. If two candidate handoff points exist, choose the one already canonicalized in milestone docs.
4. Layer ownership conflicts resolve in favor of lower-level canonical schema docs (#609-#614), not demo narrative.

## Security / Privacy Boundaries
- No secrets, tokens, raw prompts, raw tool arguments, or absolute host paths in handoff artifacts.
- Demo artifacts should reference sanitized, deterministic surfaces only.

## Non-goals
- Redesigning #613/#614/#615 deliverables.
- Defining new runtime policies.
- Expanding scope into adaptive policy/online learning.
