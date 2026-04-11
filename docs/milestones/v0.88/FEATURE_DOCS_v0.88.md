# Feature Docs - v0.88

## Metadata
- Milestone: `v0.88`
- Version: `v0.88`
- Date: `2026-04-11`
- Owner: `Daniel Austin`

## Purpose
Provide the canonical index for the promoted `v0.88` feature documents.

This page defines the bounded public feature-doc package for `v0.88`. It is an index, not a replacement for the feature docs themselves.

## Scope Interpretation

`v0.88` has two real tracked feature bands:
- temporal / chronosense substrate
- instinct / bounded-agency substrate

This package intentionally excludes:
- later-band aptitude and learning / skills planning
- helper cluster maps and planning notes
- later social/governance temporal systems

## Feature Index

| Feature doc | Primary concern | Main WPs |
|---|---|---|
| `features/SUBSTANCE_OF_TIME.md` | conceptual chronosense foundation | `WP-02` |
| `features/TEMPORAL_SCHEMA_V01.md` | temporal anchors, clock stack, execution-policy trace hooks | `WP-03`, `WP-08` |
| `features/CHRONOSENSE_AND_IDENTITY.md` | continuity, interruption, resumption, identity semantics | `WP-04` |
| `features/TEMPORAL_QUERY_AND_RETRIEVAL.md` | time-aware retrieval and staleness-aware querying | `WP-05` |
| `features/COMMITMENTS_AND_DEADLINES.md` | future obligations, deadline states, missed-commitment visibility | `WP-06` |
| `features/TEMPORAL_CAUSALITY_AND_EXPLANATION.md` | bounded order / dependency / explanation surface | `WP-07` |
| `features/ADL_COST_MODEL.md` | execution mode, realized cost, and economics | `WP-08` |
| `features/PHI_METRICS_FOR_ADL.md` | engineering metrics for integration, irreducibility, and coupling | `WP-09` |
| `features/INSTINCT_MODEL.md` | bounded instinct as a cognitive substrate | `WP-10` |
| `features/INSTINCT_RUNTIME_SURFACE.md` | runtime declaration, influence, and proof surface for instinct | `WP-11` |

## Local Planning Inputs Not Promoted

The following remain local planning material and should not be treated as public `v0.88` feature commitments yet:
- `.adl/docs/v0.89planning/APTITUDE_MODEL.md`
- `.adl/docs/v0.88planning/TEMPORAL_CLUSTER_MAP.md`
- `.adl/docs/v0.88planning/RUNTIME_PROVIDER_AND_ECONOMICS_CLUSTER_MAP.md`
- `.adl/docs/v0.88planning/WP_INSTINCT_AND_BOUNDED_AGENCY.md`

## Review Guidance
- Treat `README.md`, `VISION_v0.88.md`, `DESIGN_v0.88.md`, `WBS_v0.88.md`, and `SPRINT_v0.88.md` as the milestone planning package.
- Treat the files in `features/` as the canonical tracked feature surface.
- Treat contradictions between the planning package and the promoted feature docs as defects.
