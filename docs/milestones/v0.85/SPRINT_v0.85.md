# Sprint Plan — v0.85

## Metadata
- Sprint plan: `v0.85`
- Milestone: `v0.85`
- Start date: `2026-03-10`
- End date: `TBD`
- Owner: `Daniel Austin / Agent Logic`

## Role Of This Document

This sprint plan follows the canonical execution model in [WBS_v0.85.md](WBS_v0.85.md).

Use this file to understand:
- the phased milestone structure
- what has already landed
- what remains in the closeout queue

Do not use the mid-flight review document as the sprint plan. That file is now diagnostic and closure-oriented.

## Working Rules

- Execute work in dependency order from the WBS.
- Treat the WBS as the single source of truth for execution sequencing and deliverable expectations.
- Keep issue, PR, STP, SIP, and SOR records aligned for every active issue.
- Treat demos, review records, and validation outputs as milestone evidence, not optional extras.

## Current Sprint State

- Sprint 1 foundation work is largely landed:
  - `WP-02` through `WP-04` are landed
  - `WP-01` remains the umbrella alignment owner under `#886`
- Sprint 2 authoring/runtime tooling work is landed:
  - `WP-05` through `WP-08` are landed
- Sprint 3 cognitive/runtime work is landed:
  - `WP-09` through `WP-17` are landed
- Sprint 4 demo-program work is landed:
  - `WP-18` is landed
- The active closeout queue is now:
  - `WP-19` through `WP-25`
- Supporting closeout refresh work also landed late in the milestone:
  - five-command authoring reconciliation and command hardening
  - Rust maintainability/refactor cleanup
  - demo/readiness refresh

## Sprint Structure

### Sprint 1 — Foundation And Milestone Reorganization
Goal:
Land the execution substrate and milestone-alignment foundation needed for the rest of v0.85.

Landed / active scope:
- `WP-01` milestone reorganization and docs alignment (`#886`) — still active
- `WP-02` deterministic queue / checkpoint / steering substrate (`#674`) — landed
- `WP-03` cluster / distributed execution groundwork (`#868`) — landed
- `WP-04` Prompt Spec completeness for editors (`#716`) — landed

### Sprint 2 — Authoring Surfaces And Runtime Trust Tooling
Goal:
Make authoring, review, dependable execution, and verifiable inference materially real.

Landed scope:
- `WP-05` first authoring/editor surfaces (`#870`) — landed
- `WP-06` editing and review tooling surfaces (`#871`) — landed
- `WP-07` dependable execution runtime surfaces (`#872`) — landed
- `WP-08` verifiable inference runtime surfaces (`#873`) — landed

### Sprint 3 — AEE, Godel, Affect, And Reasoning
Goal:
Deliver the milestone’s major cognitive/runtime step with bounded, inspectable artifacts.

Landed scope:
- `WP-09` Adaptive Execution Engine bounded progress (`#874`)
- `WP-10` deterministic hypothesis generation engine (`#748`)
- `WP-11` policy-learning and adaptive Godel loop (`#749`)
- `WP-12` experiment prioritization and strategy confidence (`#750`)
- `WP-13` cross-workflow learning and recursive improvement (`#751`)
- `WP-14` promotion and eval-report artifact loop (`#752`)
- `WP-15` affect engine core (`#875`)
- `WP-16` reasoning graph and affect integration (`#876`)
- `WP-17` affect-plus-Godel vertical slice (`#877`)

### Sprint 4 — Integration, Review, And Release
Goal:
Prove the milestone, complete review/release work, and prepare the next milestone cleanly.

Current scope:
- `WP-18` demo program for v0.85 features (`#878`) — landed
- `WP-19` coverage / quality gate (`#879`) — closeout queue
- `WP-20` documentation consistency pass (`#880`) — closeout queue
- `WP-21` internal review (`#901`) — closeout queue
- `WP-22` external review (`#902`) — closeout queue
- `WP-23` review findings remediation (`#903`) — closeout queue
- `WP-24` release ceremony (`#881`) — closeout queue
- `WP-25` next milestone planning (`#882`) — closeout queue

## Parallel Alignment Tranche

These items are not a separate sprint, but they are still blocking enough to track alongside execution:
- cognitive authority and deduplication (`A1` through `A6`)
- proof-surface discipline harmonization (`B3`)
- terminology/public-artifact harmonization (`B4`)
- WBS/scope rewrite and sprint harmonization (`D1`, `D2`, including `#927`)

This work exists to keep the executing milestone coherent. It should not replace the WBS as the execution plan.

## Current Work Plan

| Order | Item | Issue | Status | Concrete deliverable |
|---|---|---|---|---|
| 1 | `WP-01` milestone reorganization and docs alignment | `#886` | ACTIVE | aligned milestone docs and issue graph |
| 2 | blocking alignment tranche (`A1-A6`, `B3`, `B4`, `D1`, `D2`) | `#886`, `#927`, follow-ons as needed | ACTIVE | canonical cognitive docs, harmonized terminology/proof rules, rewritten WBS/sprint framing |
| 3 | `WP-19` coverage / quality gate | `#879` | OPEN | coverage evidence and documented release-quality posture |
| 4 | `WP-20` documentation consistency pass | `#880` plus final refresh work | OPEN | milestone docs and canonical issue bodies reconciled to repo truth |
| 5 | `WP-21` internal review | `#901` | OPEN | internal review record and findings |
| 6 | `WP-22` external review | `#902` | OPEN | external review record and findings |
| 7 | `WP-23` review findings remediation | `#903` | OPEN | remediations or explicit deferrals |
| 8 | `WP-24` release ceremony | `#881` | OPEN | release record, tag, and notes |
| 9 | `WP-25` next milestone planning | `#882` | OPEN | next-milestone planning package |

## Demo / Review Plan

Required milestone proof surfaces:
- steering / queueing / checkpoint behavior
- authoring and review workflow behavior
- five-command editing lifecycle behavior
- hypothesis-engine / Godel behavior
- affect-engine behavior
- affect-plus-Godel vertical-slice behavior

Review sequence:
1. documentation consistency pass
2. internal review
3. external review
4. findings remediation or explicit deferral
5. release ceremony
6. next milestone planning

## Exit Criteria

- The WBS and issue graph remain aligned.
- Remaining open work can be explained directly from the WBS and this sprint plan.
- Every remaining work package has a concrete proof surface.
- Required demos, review records, and release evidence are present before milestone closeout.
