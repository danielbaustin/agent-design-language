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
| WP-02 | Chronosense foundation | establish the conceptual chronosense substrate | runtime-facing chronosense definitions, acceptance criteria, and at least one bounded proof hook | `WP-01` | `#1644` closed |
| WP-03 | Temporal schema | define temporal anchors, clocks, and execution-policy trace hooks | concrete schema fields, runtime serialization surface, and targeted tests | `WP-01` | `#1646` closed |
| WP-04 | Continuity and identity semantics | ground continuity, interruption, resumption, and identity semantics in temporal structure | continuity artifact contract, implementation slice, and at least one proof fixture | `WP-02`, `WP-03` | `#1648` closed |
| WP-05 | Temporal query and retrieval | make time-aware retrieval and staleness queryable | query surface, fixture-backed examples, and validation tests | `WP-03` | `#1650` closed |
| WP-06 | Commitments and deadlines | represent future obligations and missed commitments as first-class temporal records | commitment/deadline artifact model, bounded runtime path, and proof fixtures | `WP-03`, `WP-05` | `#1651` closed; bounded supporting slice `#1614` closed |
| WP-07 | Temporal causality and explanation | define bounded causal / explanatory review surfaces | explanation artifact format, bounded evaluation path, and reviewer-facing examples | `WP-03`, `WP-05` | `#1653` closed |
| WP-08 | Execution policy and cost model | tie execution mode and realized cost back to trace reviewability | execution-policy contract, cost fields/artifacts, and comparison proof path | `WP-03` | `#1655` closed |
| WP-09 | PHI-style integration metrics | define bounded engineering metrics for integration, irreducibility, coupling, and adaptive depth in ADL systems | metric definitions, comparison runner or fixture set, and reviewable outputs | `WP-02` through `WP-08` | `#1645` closed |
| WP-10 | Instinct model | define bounded instinct as an explicit cognitive substrate | runtime-facing instinct contract, bounded semantics, and acceptance tests | `WP-01` | `#1649` closed |
| WP-11 | Instinct runtime surface and bounded agency hook | make instinct visible in runtime declaration, routing, prioritization, trace, and demo proof | implementation slice, trace/artifact evidence, and bounded-agency proof case | `WP-10` | `#1654` closed |
| WP-12 | Paper Sonata flagship demo | implement a bounded investor-/reviewer-facing multi-agent manuscript demo with durable artifacts and truthful runtime proof | bounded runner, synthetic fixture packet, stable artifact tree, and smoke/validation path | `WP-02` through `WP-11` | `#1656` closed; protected local follow-on planning retained |
| WP-13 | Demo matrix + integration demos | define and implement the primary proof surfaces for temporal, PHI, instinct, and Paper Sonata bands | runnable demo entrypoints, validated artifacts, and reviewer-facing demo matrix | `WP-02` through `WP-12` | `#1657` closed; bounded supporting proof `#1618` closed |
| WP-14 | Coverage / quality gate | enforce milestone quality and coverage posture | green quality gate | `WP-13` | `#1652` closed |
| WP-15 | Docs + review pass | converge reviewer-facing docs against delivered proof | reviewer-ready package | `WP-13`, `WP-14` | `#1658` closed |
| WP-16 | Internal review | perform bounded internal review of milestone truth and proof surfaces | internal review record | `WP-15` | `#1659` open |
| WP-17 | 3rd-party review | perform external review of the milestone package and capture findings | 3rd-party review record | `WP-15`, `WP-16` | `#1660` open |
| WP-18 | Review findings remediation | resolve or explicitly defer accepted review findings | remediation record | `WP-16`, `WP-17` | `#1661` open |
| WP-19 | Next milestone planning | prepare the next milestone planning package before `v0.88` closeout | next-milestone package | `WP-18` | `#1662` open |
| WP-20 | Release ceremony | final validation, notes, tag, cleanup, and closeout record | release package | `WP-18`, `WP-19` | `#1663` open |

Issue-column note:
- `WP-01` is already represented by tracked planning/package issues.
- `WP-02` through `WP-20` are now represented by the seeded `v0.88` work-package issue wave.
- `#1614` and `#1618` are bounded supporting pull-ins, not replacements for the main work-package issues.

## Exit Criteria
- every tracked `v0.88` feature doc maps to at least one WBS item
- the instinct / bounded-agency band is no longer missing from tracked milestone truth
- the release tail uses the normal `v0.86` / `v0.87` pattern with no extra invented steps
- the WBS tells the truth about both completed implementation work and the still-open closeout tail
