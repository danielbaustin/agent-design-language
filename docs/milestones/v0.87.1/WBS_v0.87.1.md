# Work Breakdown Structure (WBS) - v0.87.1

## Metadata
- Milestone: `v0.87.1`
- Version: `v0.87.1`
- Date: `2026-04-06`
- Owner: `Daniel Austin`

## How To Use
- Break work into independently-mergeable issues.
- Keep each item measurable and testable.
- Include deliverables + dependencies + issue links.
- `WP-01` is **always** the milestone **design pass** (canonical docs + WBS + decisions + sprint plan + checklist).
- Reserve the final WPs for the release tail in this order: demos, quality/coverage gate, docs/review convergence, internal review, external / 3rd-party review, review-findings remediation or explicit deferral capture, next-milestone planning, and release ceremony.

## WBS Summary
`v0.87.1` is the runtime-completion milestone for ADL. It is the point where the runtime moves from early planning into a full implementation target with real execution lifecycle, deterministic trace alignment, local resilience, operator surfaces, review surfaces, and runnable demos.

This milestone is not satisfied by a public shell alone. It requires substantial runtime implementation, strong proof surfaces, and the full review-and-release tail needed for a large substrate milestone.

## Work Packages
| ID | Work Package | Description | Deliverable | Dependencies | Issue |
|---|---|---|---|---|---|
| WP-01 | Design pass (milestone docs + planning) | Align canonical `v0.87.1` docs, planning, and roadmap framing for the runtime-completion milestone | Complete canonical milestone docs aligned to runtime completion scope | none | `#1435` |
| WP-02 | Runtime environment completion | Define and implement the first authoritative runtime-environment surface for `v0.87.1`, including entrypoints, env contracts, and configuration expectations | Working runtime environment substrate and docs that match it | WP-01 | `#1436` |
| WP-03 | Execution boundaries & lifecycle | Implement and document explicit lifecycle phases and runtime boundaries (`init`, `execute`, `complete`, `teardown`) | Bounded lifecycle model with real implementation and review surfaces | WP-01, WP-02 | `#1437` |
| WP-04 | Trace-aligned runtime execution | Align runtime lifecycle phases with Trace v1 events, artifacts, and replay expectations | Runtime execution mapped coherently to trace/proof surfaces | WP-02, WP-03 | `#1438` |
| WP-05 | Local runtime resilience + Shepherd preservation | Implement local failure handling, restartability, bounded recovery semantics, and Shepherd preservation behavior for interrupted or failed runtime work | Resilient local runtime behaviors and documented Shepherd guarantees | WP-02, WP-03 | `#1439` |
| WP-06 | Operator surfaces | Standardize invocation patterns, runtime commands, demo scripts, and artifact layout | Stable operator entrypoints and reviewable artifact conventions | WP-02, WP-03 | `#1440` |
| WP-07 | Runtime state / persistence discipline | Define and implement state, restart, continuity, and cleanup discipline for bounded local runs | Inspectable runtime state/persistence surface with deterministic cleanup and continuity behavior | WP-03, WP-05 | `#1441` |
| WP-08 | Runtime review surfaces | Standardize runtime review outputs, inspection surfaces, and verification artifacts | Reviewable runtime output surfaces for internal and external inspection | WP-04, WP-06, WP-07 | `#1442` |
| WP-09 | Cross-document consistency pass | Enforce agreement between VISION, DESIGN, WBS, SPRINT, CHECKLIST, README, and promoted runtime surfaces | No contradictions across milestone docs and implementation claims | WP-02..WP-08 | `#1458` |
| WP-10 | Acceptance criteria finalization | Define measurable acceptance criteria for each WP and the overall runtime-completion milestone | Completed Acceptance Mapping section with runtime-specific proof expectations | WP-09 | `#1459` |
| WP-11 | Sprint plan alignment | Ensure SPRINT doc reflects WBS sequencing, review tail, and release structure | Updated SPRINT with explicit implementation and review phases | WP-01, WP-10 | `#1460` |
| WP-12 | Checklist / release-gate completion | Populate checklist and release-gate surfaces with runtime-specific validation expectations | Complete milestone checklist and release-gate mapping | WP-10, WP-11 | `#1461` |
| WP-13 | Demo matrix + integration demos | Define and implement the milestone demo program across runtime environment, lifecycle, resilience, operator, and review surfaces | Real demo matrix and runnable runtime demos with clear proof claims | WP-04, WP-05, WP-06, WP-08 | `#1462` |
| WP-14 | Coverage / quality gate (ratchet + exclusions) | Establish truthful quality posture for the runtime milestone, including tests, validators, demos, and justified exceptions | Auditable quality/coverage posture for the runtime-completion milestone | WP-02 through WP-13 | `#1463` |
| WP-15 | Docs + review pass (repo-wide alignment) | Converge milestone docs, runtime proof surfaces, and reviewer entry docs so an uninvolved reviewer can understand the implemented runtime truthfully | Reviewed and aligned docs/review package for `v0.87.1` | WP-13, WP-14 | `#1464` |
| WP-16 | Internal review | Perform bounded internal review of milestone truth, runtime behavior, and proof surfaces | Internal review record with actionable findings | WP-15 | `#1494` |
| WP-17 | External / 3rd-party review | Execute the external review leg against the stabilized `v0.87.1` proof package and capture findings or a clean review result | Completed external review record for the runtime milestone | WP-16 | `#1495` |
| WP-18 | Review findings remediation | Remediate accepted review findings or record explicit bounded deferrals with ownership before closeout | Updated artifacts plus tracked deferrals/ownership | WP-16, WP-17 | `#1496` |
| WP-19 | Next milestone planning | Prepare the next milestone planning package before `v0.87.1` closeout | Explicit next-milestone planning materials and tracked follow-on work | WP-18 | `#1497` |
| WP-20 | Release ceremony (final validation + tag + notes + cleanup) | Perform final validation, release-note alignment, cleanup, and milestone closeout for the runtime milestone | Ready-to-merge runtime milestone state and release artifacts | WP-19 | `#1498` |

## Sequencing
- Phase 1: WP-01..WP-08 (runtime foundations, lifecycle, resilience, operator and review surfaces)
- Phase 2: WP-09..WP-12 (alignment, acceptance, sprint, checklist, and release-gate preparation)
- Phase 3: WP-13..WP-20 (demos, quality, docs/review, internal review, external / 3rd-party review, remediation, next-milestone planning, and release)

## Acceptance Mapping
Use this section as the canonical acceptance contract for `v0.87.1`. Downstream demo, checklist, quality, review, and release-tail work should point back here rather than creating independent definitions of done.

| WP | Acceptance criteria | Required proof surface / gate |
|---|---|---|
| WP-01 | Canonical milestone docs exist, agree that `v0.87.1` is the runtime-completion milestone, and name the non-goals that remain future work. | `README.md`, `VISION_v0.87.1.md`, `DESIGN_v0.87.1.md`, `WBS_v0.87.1.md`, `SPRINT_v0.87.1.md`, `DECISIONS_v0.87.1.md`, and `MILESTONE_CHECKLIST_v0.87.1.md` are reviewed together. |
| WP-02 | Runtime environment entrypoints, environment contract, configuration expectations, and runtime artifact roots are implemented and documented. | Runtime environment implementation, promoted feature docs, tests, and demo/review artifacts for the environment surface. |
| WP-03 | Runtime lifecycle phases and execution boundaries are explicit in implementation, docs, and trace/review surfaces. | Lifecycle/boundary implementation, promoted feature docs, tests, and traceable runtime summaries showing bounded phase transitions. |
| WP-04 | Runtime execution emits Trace v1-aligned events and artifacts with replay-aware proof expectations. | Trace-aligned runtime implementation, trace/replay validation, runtime proof artifacts, and demo rows that name exact proof roots. |
| WP-05 | Local failure handling, restartability, bounded recovery, and Shepherd preservation behavior are implemented or explicitly dispositioned with reviewer-visible evidence. | Resilience and Shepherd implementation, tests, promoted feature docs, and failure/recovery proof artifacts. |
| WP-06 | Operator-facing commands and demo entrypoints are stable, documented, and runnable from the repository root. | Operator command docs, demo scripts, validation output, and artifact layout records. |
| WP-07 | Runtime state, persistence, continuity, and cleanup behavior are deterministic enough for bounded local review and do not imply future identity or agency-continuity claims. | State/persistence implementation, cleanup validation, feature docs, and runtime-state demo/review artifacts. |
| WP-08 | Runtime review surfaces let an uninvolved reviewer locate, inspect, and validate primary runtime outputs without reconstructing context from issue history. | Runtime review artifacts, reviewer entry docs, demo matrix links, and inspection commands. |
| WP-09 | Canonical docs and implementation-facing claims have no known contradictions after the cross-document consistency pass. | Updated milestone docs plus the WP-09 SOR and focused contradiction/stale-language scans. |
| WP-10 | This acceptance mapping gives each WP measurable criteria and points to the proof surfaces later gates must use. | Updated WBS acceptance mapping plus linked checklist, demo, feature-doc, and release-plan references. |
| WP-11 | Sprint sequencing mirrors the WBS dependency order and separates runtime foundation, convergence, demos, quality, review, remediation, planning, and release-tail work. | `SPRINT_v0.87.1.md` aligned to this WBS and issue graph. |
| WP-12 | Checklist and release-gate surfaces can be executed as a ship/no-ship review, with each exception requiring owner, due date, and disposition. | `MILESTONE_CHECKLIST_v0.87.1.md`, `RELEASE_PLAN_v0.87.1.md`, and release-tail issue cards. |
| WP-13 | Demo matrix lists every milestone demo/proof issue, distinguishes CI-safe from credential-gated evidence, and provides runnable commands plus primary proof surfaces. | `DEMO_MATRIX_v0.87.1.md`, D0 suite results, D13/D13L disposition, and per-demo proof artifacts. |
| WP-14 | Quality posture is auditable: required checks pass or each exception is explicitly justified, owned, and tracked. | CI/local validation records, quality/coverage issue output, and documented exclusions or deferrals. |
| WP-15 | Docs and reviewer entry surfaces converge on implemented runtime truth and do not overclaim future systems. | Docs/review pass output, reviewer package entrypoints, and stale-claim scans. |
| WP-16 | Internal review records concrete findings, severity, ownership, and whether each finding is accepted, rejected, or deferred. | Internal review record and linked issue/PR findings. |
| WP-17 | External / 3rd-party review is completed against a legible package that identifies what to run, what to inspect, and what is intentionally out of scope. | External review record, demo/review entrypoints, and proof-surface navigation. |
| WP-18 | Accepted review findings are remediated before release or deferred with explicit rationale, owner, and follow-up issue. | Changed artifacts, remediation SORs, and tracked deferral issues. |
| WP-19 | Next milestone planning exists before release closeout and captures follow-on work that `v0.87.1` deliberately does not complete. | Next-milestone planning docs, WBS/sprint seeds, and follow-on issues. |
| WP-20 | Release ceremony validates the final state, publishes release artifacts, and leaves the repository/milestone clean for the next tranche. | Final checklist, release notes, tag/release records, post-release verification, and closeout notes. |

## Milestone-Level Acceptance
- Runtime completion is accepted only when WP-02 through WP-08 are implemented, documented, and represented in runnable or reviewable proof surfaces.
- Convergence is accepted only when WP-09 through WP-12 align the docs, acceptance contract, sprint plan, checklist, and release gates without contradictory definitions of done.
- Demo coverage is accepted only when WP-13 names every milestone demo/proof issue, marks non-CI or credential-gated surfaces truthfully, and gives reviewers runnable commands or explicit substitute proof surfaces.
- Quality and review are accepted only when WP-14 through WP-18 have passing evidence or explicit owner-bound deferrals for every remaining exception.
- Release closeout is accepted only when WP-19 planning exists before release and WP-20 records the final validation, release, and cleanup state.

## Exit Criteria
- Every in-scope requirement maps to at least one WBS item.
- Every WBS item has an owner, issue reference, and concrete deliverable.
- Dependency order is explicit enough to execute deterministically.
