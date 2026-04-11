# Work Breakdown Structure (WBS) - v0.88

## Metadata
- Milestone: `v0.88`
- Version: `v0.88`
- Date: `2026-04-10`
- Owner: `Daniel Austin`

## WBS Summary

`v0.88` is the chronosense and temporal-substrate milestone. The work breaks into one completed planning/package-establishment band, a set of core temporal feature bands, then the usual demo/review/release tail.

## Work Packages

| ID | Work Package | Description | Deliverable | Dependencies | Issue |
|---|---|---|---|---|---|
| WP-01 | Planning shell + promoted feature package | seed the milestone shell and promote the bounded `v0.88` feature docs into tracked surfaces | milestone shell + feature-doc index + promoted docs | none | `#1527`, `#1579` |
| WP-02 | Chronosense foundation | establish the conceptual chronosense substrate | `SUBSTANCE_OF_TIME.md` aligned work | `WP-01` | pending |
| WP-03 | Temporal schema | define and implement temporal anchors and schema contracts | `TEMPORAL_SCHEMA_V01.md` aligned work | `WP-01` | pending |
| WP-04 | Continuity and identity semantics | ground identity and resumption in temporal continuity | `CHRONOSENSE_AND_IDENTITY.md` aligned work | `WP-02`, `WP-03` | pending |
| WP-05 | Temporal query and retrieval | make time-aware retrieval and staleness queryable | `TEMPORAL_QUERY_AND_RETRIEVAL.md` aligned work | `WP-03` | pending |
| WP-06 | Commitments and deadlines | represent future obligations and missed commitments as first-class records | `COMMITMENTS_AND_DEADLINES.md` aligned work | `WP-03`, `WP-05` | pending |
| WP-07 | Temporal causality and explanation | define bounded causal/explanatory review surfaces | `TEMPORAL_CAUSALITY_AND_EXPLANATION.md` aligned work | `WP-03`, `WP-05` | pending |
| WP-08 | Execution policy and cost model | tie execution mode and realized cost back to trace | `ADL_COST_MODEL.md` aligned work | `WP-03` | pending |
| WP-13 | Demo matrix + integration demos | run the demo/proof surface | validated demos | later planning | pending |
| WP-14 | Coverage / quality gate | enforce milestone quality and coverage posture | green quality gate | `WP-13` | pending |
| WP-15 | Docs + review pass | converge reviewer-facing docs against delivered proof | reviewer-ready package | `WP-13`, `WP-14` | pending |
| WP-16 | Release ceremony | final validation, notes, tag, cleanup | release package | `WP-15` | pending |
