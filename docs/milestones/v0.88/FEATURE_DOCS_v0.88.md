# Feature Docs - v0.88

## Metadata
- Milestone: `v0.88`
- Version: `v0.88`
- Date: `2026-04-10`
- Owner: `Daniel Austin`

## Purpose
Provide the canonical index for the promoted `v0.88` feature documents.

This page defines the bounded public feature-doc package for `v0.88`. It is an index, not a replacement for the feature docs themselves.

## Scope Interpretation

`v0.88` is the chronosense and temporal-substrate milestone.

The promoted package covers:
- chronosense as a foundational cognitive substrate
- temporal anchors and schema contracts
- continuity and identity semantics tied to time
- temporal query and retrieval behavior
- commitments and deadlines as first-class temporal records
- bounded temporal causality and explanation
- execution-policy and cost semantics tied back to trace

This package does not yet include:
- cross-agent temporal alignment
- timeline forks and counterfactuals
- temporal accountability and social/governance interpretation
- instinct, aptitude, or broader constitutional/governance systems planned for later milestones

## Feature Index

| Feature doc | Primary concern | Main WPs |
|---|---|---|
| `features/SUBSTANCE_OF_TIME.md` | conceptual chronosense foundation | `WP-02` |
| `features/TEMPORAL_SCHEMA_V01.md` | temporal anchors, clock stack, execution-policy trace hooks | `WP-03`, `WP-08` |
| `features/CHRONOSENSE_AND_IDENTITY.md` | continuity, interruption, resumption, identity semantics | `WP-04` |
| `features/TEMPORAL_QUERY_AND_RETRIEVAL.md` | time-aware retrieval and staleness-aware querying | `WP-05` |
| `features/COMMITMENTS_AND_DEADLINES.md` | future obligations, deadline states, missed-commitment visibility | `WP-06` |
| `features/TEMPORAL_CAUSALITY_AND_EXPLANATION.md` | bounded order/dependency/explanation surface | `WP-07` |
| `features/ADL_COST_MODEL.md` | execution mode, realized cost, and economics | `WP-08` |

## Review Guidance
- Treat `README.md`, `DESIGN_v0.88.md`, and `WBS_v0.88.md` as the milestone planning shell.
- Treat the files in `features/` as the promoted feature surface for the current `v0.88` package boundary.
- Treat contradictions between the planning shell and the promoted feature docs as defects to resolve before implementation planning advances.
