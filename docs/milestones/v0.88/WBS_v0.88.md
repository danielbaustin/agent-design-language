# Work Breakdown Structure (WBS) - v0.88

## Metadata
- Milestone: `v0.88`
- Version: `v0.88`
- Date: `2026-04-11`
- Owner: `Daniel Austin`

## WBS Summary

`v0.88` delivers two bounded substrate bands:
- temporal / chronosense
- instinct / bounded agency

After those feature bands, the milestone follows the same demo / quality / review / release tail used in `v0.86` and `v0.87`.

## Work Packages

| ID | Work Package | Description | Deliverable | Dependencies | Issue |
|---|---|---|---|---|---|
| WP-01 | Canonical planning package | reconcile the tracked `v0.88` planning package, promoted feature index, and milestone structure so issue seeding can start from one truthful public surface | coherent milestone docs + promoted feature set | none | `#1527`, `#1579`, `#1497` |
| WP-02 | Chronosense foundation | establish the conceptual chronosense substrate | `SUBSTANCE_OF_TIME.md` aligned work | `WP-01` | pending |
| WP-03 | Temporal schema | define temporal anchors, clocks, and execution-policy trace hooks | `TEMPORAL_SCHEMA_V01.md` aligned work | `WP-01` | pending |
| WP-04 | Continuity and identity semantics | ground continuity, interruption, resumption, and identity semantics in temporal structure | `CHRONOSENSE_AND_IDENTITY.md` aligned work | `WP-02`, `WP-03` | pending |
| WP-05 | Temporal query and retrieval | make time-aware retrieval and staleness queryable | `TEMPORAL_QUERY_AND_RETRIEVAL.md` aligned work | `WP-03` | pending |
| WP-06 | Commitments and deadlines | represent future obligations and missed commitments as first-class temporal records | `COMMITMENTS_AND_DEADLINES.md` aligned work | `WP-03`, `WP-05` | pending |
| WP-07 | Temporal causality and explanation | define bounded causal / explanatory review surfaces | `TEMPORAL_CAUSALITY_AND_EXPLANATION.md` aligned work | `WP-03`, `WP-05` | pending |
| WP-08 | Execution policy and cost model | tie execution mode and realized cost back to trace reviewability | `ADL_COST_MODEL.md` aligned work | `WP-03` | pending |
| WP-09 | PHI-style integration metrics | define bounded engineering metrics for integration, irreducibility, coupling, and adaptive depth in ADL systems | `PHI_METRICS_FOR_ADL.md` aligned work | `WP-02` through `WP-08` | pending |
| WP-10 | Instinct model | define bounded instinct as an explicit cognitive substrate | `INSTINCT_MODEL.md` aligned work | `WP-01` | pending |
| WP-11 | Instinct runtime surface and bounded agency hook | make instinct visible in runtime declaration, routing, prioritization, trace, and demo proof | `INSTINCT_RUNTIME_SURFACE.md` aligned work | `WP-10` | pending |
| WP-12 | Demo matrix + integration demos | define and implement the primary proof surfaces for temporal, PHI, and instinct bands | validated demos and reviewer-facing demo matrix | `WP-02` through `WP-11` | pending |
| WP-13 | Coverage / quality gate | enforce milestone quality and coverage posture | green quality gate | `WP-12` | pending |
| WP-14 | Docs + review pass | converge reviewer-facing docs against delivered proof | reviewer-ready package | `WP-12`, `WP-13` | pending |
| WP-15 | Internal review | perform bounded internal review of milestone truth and proof surfaces | internal review record | `WP-14` | pending |
| WP-16 | 3rd-party review | perform external review of the milestone package and capture findings | 3rd-party review record | `WP-14`, `WP-15` | pending |
| WP-17 | Review findings remediation | resolve or explicitly defer accepted review findings | remediation record | `WP-15`, `WP-16` | pending |
| WP-18 | Next milestone planning | prepare the next milestone planning package before `v0.88` closeout | next-milestone package | `WP-17` | pending |
| WP-19 | Release ceremony | final validation, notes, tag, cleanup, and closeout record | release package | `WP-17`, `WP-18` | pending |

## Exit Criteria
- every tracked `v0.88` feature doc maps to at least one WBS item
- the instinct / bounded-agency band is no longer missing from tracked milestone truth
- the release tail uses the normal `v0.86` / `v0.87` pattern with no extra invented steps
- the planning package is strong enough to seed the real issue wave without another structural rewrite
