# Sprint Plan - v0.89

## Metadata
- Milestone: `v0.89`
- Sprint sequence: `v0.89-s1`, `v0.89-s2`, `v0.89-s3`
- Start date: `2026-04-12`
- End date: `TBD`
- Owner: `Daniel Austin`

## Milestone Sprint Model

`v0.89` is a three-sprint execution plan:
- Sprint 1: seed the issue wave and land the convergence / judgment / action core
- Sprint 2: land the skill / experiment / memory / security package and the explicit `v0.89.2` handoff
- Sprint 3: converge demos, quality, docs/review, and release closure

This keeps the execution model aligned with the recent milestone pattern while preserving a clean boundary between the main governed-adaptation band and the follow-on adversarial-runtime band.

## Sprint Overview

| Sprint | Purpose | WPs | Current status |
|---|---|---|---|
| `v0.89-s1` | seed the issue wave and execute the convergence / gate / action core | `WP-01` through `WP-05` | active; issue wave seeded |
| `v0.89-s2` | execute the skill / experiment / memory / security package plus explicit `v0.89.2` handoff planning | `WP-06`, `WP-07`, `WP-08`, `WP-09`, `WP-10` | planned |
| `v0.89-s3` | converge demos, quality, docs/review, and release ceremony | `WP-11` through `WP-16` | planned |

## Sprint 1

### Goal
Move `v0.89` from a strong planning package into a real execution wave by seeding the issue graph and landing the first governed-adaptation core band.

### Scope
- issue-wave seeding from the promoted `v0.89` package
- AEE convergence
- Freedom Gate v2
- decision surfaces and decision schema
- action mediation and action proposal schema

Current issue map:
- `WP-01` `#1662`
- `WP-02` `#1747`
- `WP-03` `#1748`
- `WP-04` `#1749`
- `WP-05` `#1750`

### Exit Criteria
- `WP-02` through `WP-05` are issueized and mapped in the milestone docs
- the convergence / gate / decision / action band has an executable issue wave
- the main milestone docs stop speaking about the issue wave as hypothetical

## Sprint 2

### Goal
Land the governed execution substrate that makes `v0.89` useful beyond judgment rhetoric: skills, experiments, evidence-bearing memory, and the main-band security package.

### Scope
- skill model and skill execution protocol
- Godel experiment system
- ObsMem evidence and ranking
- security, trust, and posture package
- explicit `v0.89.2` handoff planning

Current issue map:
- `WP-06` `#1751`
- `WP-07` `#1752`
- `WP-08` `#1753`
- `WP-09` `#1754`
- `WP-10` `#1755`

### Exit Criteria
- `WP-06` through `WP-10` are issueized and tracked in the milestone package
- the main `v0.89` feature band is fully represented in active issue/PR work
- `v0.89.2` carry-forward is explicit and bounded

## Sprint 3

### Goal
Close the milestone using the normal ADL pattern: demos, quality, docs/review, and release ceremony.

### Scope
- demo scaffolding and proof entry points
- milestone convergence and follow-on mapping
- demo matrix and integration demos
- coverage / quality gate
- docs + review pass
- release ceremony

Current issue map:
- `WP-11` `#1756`
- `WP-12` `#1757`
- `WP-13` `#1758`
- `WP-14` `#1759`
- `WP-15` `#1760`
- `WP-16` `#1761`

### Exit Criteria
- reviewer-facing proof surfaces exist for the core `v0.89` claims
- quality, docs, and release surfaces are consistent with delivered work
- the milestone can close without reconstructing planning intent from local-only notes

## Risks / Dependencies
- Dependency: `v0.88` release tail must close cleanly so the next wave does not inherit avoidable drift
  - Risk: unresolved `v0.88` closeout work distracts from `v0.89` issue seeding
  - Mitigation: keep `v0.89` planning package self-contained and truth-based so execution can start cleanly once `v0.88` closes
- Dependency: the seeded `v0.89` issue wave must stay aligned with the milestone docs
  - Risk: issue bodies and milestone docs drift after seeding
  - Mitigation: treat the seeded issue wave plus `WBS_v0.89.md` / `FEATURE_DOCS_v0.89.md` as one package and update them together

## Demo / Review Plan
- Demo artifact: `DEMO_MATRIX_v0.89.md` plus the later convergence/gate/experiment/security proof surfaces it governs
- Review date: milestone review date `TBD`
- Sign-off owners: Daniel Austin plus later third-party review where appropriate

## Cadence Expectations
- use issue cards (`stp` / `sip` / `sor`) for each issue
- keep changes scoped per issue; use draft PRs until checks pass
- prefer one bounded PR lane per queue unless explicit policy says otherwise
- keep `.adl` local memory preserved while tracked milestone docs and code land through normal PR flow

## Exit Criteria
- all planned scope items are completed or explicitly deferred with rationale
- linked issues and PRs are updated and traceable
- CI is green for merged work
- sprint state is reflected truthfully in milestone docs
