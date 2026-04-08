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
- Reserve the final WPs for the release tail in this order: demos, quality/coverage gate, docs/review convergence, internal review, external review preparation, review-findings remediation or explicit deferral capture, next-milestone planning, and release ceremony.

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
| WP-16 | Internal review | Perform bounded internal review of milestone truth, runtime behavior, and proof surfaces | Internal review record with actionable findings | WP-15 | TBD |
| WP-17 | External / 3rd-party review preparation | Prepare `v0.87.1` for external review legibility and proof-surface clarity | External-review-ready package for the runtime milestone | WP-16 | TBD |
| WP-18 | Review findings remediation | Remediate accepted review findings or record explicit bounded deferrals with ownership before closeout | Updated artifacts plus tracked deferrals/ownership | WP-16, WP-17 | TBD |
| WP-19 | Next milestone planning | Prepare the next milestone planning package before `v0.87.1` closeout | Explicit next-milestone planning materials and tracked follow-on work | WP-18 | TBD |
| WP-20 | Release ceremony (final validation + tag + notes + cleanup) | Perform final validation, release-note alignment, cleanup, and milestone closeout for the runtime milestone | Ready-to-merge runtime milestone state and release artifacts | WP-19 | TBD |

## Sequencing
- Phase 1: WP-01..WP-08 (runtime foundations, lifecycle, resilience, operator and review surfaces)
- Phase 2: WP-09..WP-12 (alignment, acceptance, sprint, checklist, and release-gate preparation)
- Phase 3: WP-13..WP-20 (demos, quality, docs/review, internal review, external review prep, remediation, next-milestone planning, and release)

## Acceptance Mapping
- WP-01 (Design pass) -> milestone docs are aligned and explicit about the runtime-completion milestone
- WP-02 -> runtime environment surface exists in docs and implementation
- WP-03 -> lifecycle phases are explicit, bounded, and implemented
- WP-04 -> runtime execution maps coherently to trace artifacts and replay expectations
- WP-05 -> local runtime resilience, failure handling, and Shepherd preservation expectations are real and reviewable
- WP-06 -> operator surfaces are stable and reviewer-usable
- WP-07 -> state/persistence and continuity semantics are deterministic and inspectable
- WP-08 -> runtime review surfaces exist and are usable for milestone inspection
- WP-09 -> no contradictions remain across docs and implementation claims
- WP-10 -> measurable acceptance criteria are defined for each WP and the milestone overall
- WP-11 -> sprint sequencing mirrors the WBS and review tail
- WP-12 -> checklist and release-gate surfaces are complete and actionable
- WP-13 (Demos) -> runtime demos prove the major milestone claims
- WP-14 (Quality gate) -> tests, demos, and quality-gate posture are truthful and auditable
- WP-15 (Docs/review) -> docs and reviewer entry surfaces converge around actual runtime truth
- WP-16 (Internal review) -> internal review findings are recorded and actionable
- WP-17 (External review prep) -> external review package is legible and runnable
- WP-18 (Review findings remediation) -> accepted review findings are remediated or explicitly deferred with owner and rationale
- WP-19 (Next milestone planning) -> next-milestone planning materials exist before closeout
- WP-20 (Release ceremony) -> milestone is validated, cleaned up, and releasable

## Exit Criteria
- Every in-scope requirement maps to at least one WBS item.
- Every WBS item has an owner, issue reference, and concrete deliverable.
- Dependency order is explicit enough to execute deterministically.
