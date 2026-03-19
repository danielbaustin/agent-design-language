# Sprint Plan — v0.85

## Metadata
- Sprint plan: `v0.85`
- Milestone: `v0.85`
- Start date: `2026-03-10`
- End date: `TBD`
- Owner: `Daniel Austin / Agent Logic`

## Role Of This Document

This sprint plan follows the canonical execution model in [WBS_v0.85.md](/Users/daniel/git/agent-design-language/docs/milestones/v0.85/WBS_v0.85.md).

Use this file to understand:
- the phased milestone structure
- what has already landed
- what the next execution queue is

Do not use the mid-flight review document as the sprint plan. That file is now diagnostic and closure-oriented.

## Working Rules

- Execute work in dependency order from the WBS.
- Treat the WBS as the single source of truth for execution sequencing and deliverable expectations.
- Keep issue, PR, STP, SIP, and SOR records aligned for every active issue.
- Treat demos, review records, and validation outputs as milestone evidence, not optional extras.

## Current Sprint State

- Sprint 1 foundation work is largely landed:
  - `WP-02` through `WP-04` are closed
  - `WP-01` remains active as the umbrella alignment owner under `#886`
- Sprint 2 authoring/runtime tooling work is largely landed:
  - `WP-05` through `WP-08` are closed
- The next active execution queue is Sprint 3 work:
  - `WP-09` through `WP-17`
- A blocking alignment tranche still runs in parallel:
  - cognitive authority and terminology
  - proof-surface and terminology harmonization
  - WBS/scope rewrite and sprint harmonization
- Sprint 4 remains the closeout phase:
  - demos, quality gate, review, release, next milestone planning

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

Active scope:
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

Queued scope:
- `WP-18` demo program for v0.85 features (`#878`)
- `WP-19` coverage / quality gate (`#879`)
- `WP-20` documentation consistency pass (`#880`)
- `WP-21` internal review (`#901`)
- `WP-22` external review (`#902`)
- `WP-23` review findings remediation (`#903`)
- `WP-24` release ceremony (`#881`)
- `WP-25` next milestone planning (`#882`)

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
| 3 | `WP-09` Adaptive Execution Engine bounded progress | `#874` | OPEN | bounded AEE runtime progress with inspectable artifacts |
| 4 | `WP-10` through `WP-14` Godel pipeline | `#748` through `#752` | OPEN | deterministic hypothesis/policy/prioritization/learning/eval artifact chain |
| 5 | `WP-15` through `WP-17` affect + reasoning integration | `#875` through `#877` | OPEN | affect engine, reasoning-graph integration, runnable vertical slice |
| 6 | `WP-18` demo program | `#878` | OPEN | bounded runnable demos proving milestone claims |
| 7 | `WP-19` through `WP-25` review/release closeout | `#879`, `#880`, `#901`, `#902`, `#903`, `#881`, `#882` | OPEN | coverage evidence, review records, remediations, release record, next-milestone plan |

## Demo / Review Plan

Required milestone proof surfaces:
- steering / queueing / checkpoint behavior
- authoring and review workflow behavior
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
