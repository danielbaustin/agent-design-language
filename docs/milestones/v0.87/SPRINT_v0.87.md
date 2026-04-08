# Sprint Plan: v0.87

## Metadata
- Milestone: `v0.87`
- Sprint sequence: `v0.87-s1`, `v0.87-s2`, `v0.87-s3`
- Start date: `2026`
- End date: `TBD`
- Owner: `adl`

## Milestone Sprint Model

`v0.87` is not a single-sprint milestone. It is a three-sprint execution plan:

- Sprint 1: canonicalization and first substrate slices
- Sprint 2: substrate expansion and convergence across trace, provider, shared memory, skills, tooling, and review
- Sprint 3: documentation convergence, demos, quality gate, internal review, 3rd party review, release closeout, and next-milestone planning

The purpose of this file is to describe the sprint sequence for the whole milestone, not just the first kickoff tranche.

## Milestone Goal

Move ADL from the bounded cognitive proof in `v0.86` to a coherent, deterministic, reviewer-legible substrate in `v0.87`.

This milestone should leave ADL with:
- canonical docs that truthfully describe the milestone
- a real Trace v1 substrate
- a real provider/transport substrate and compatibility path
- a real shared-memory foundation tied back to execution truth
- operational skills and review surfaces as executable substrate features
- stable control-plane/workflow surfaces
- demo, quality, docs, and release-tail proof for an uninvolved reviewer

## Sprint Overview

| Sprint | Purpose | WPs | Current status |
|---|---|---|---|
| `v0.87-s1` | Lock the canonical milestone surfaces and land the first substrate slices | `WP-01`, `WP-02`, `WP-04`, `WP-06` | active / partially implemented |
| `v0.87-s2` | Expand and connect the substrate across trace, provider, memory, skills, tooling, and review | `WP-03`, `WP-05`, `WP-07`, `WP-08`, `WP-09`, `WP-10`, `WP-11` | seeded / underway |
| `v0.87-s3` | Converge docs, demos, quality gates, internal review, 3rd party review, release tail, and next-milestone planning | `WP-12`, `WP-13`, `WP-14`, `WP-15`, `WP-15A`, `WP-16`, `WP-17` | active |

## Sprint 1

### Goal
Establish the first executable slice of `v0.87` by locking the canonical milestone docs and landing the earliest foundational substrate work.

### Scope
- canonical milestone docs and issue map
- Trace v1 schema and event model
- provider / transport substrate v1
- shared ObsMem foundation

### Work Packages
| Order | Item | Issue | Owner | Status |
|---|---|---|---|---|
| 1 | Lock the canonical milestone docs and issue map | `#1292` | `Daniel / Codex.app` | closed |
| 2 | Trace v1 schema + event model | `#1293` | `Daniel / Codex.app` | closed |
| 3 | Provider / transport substrate v1 | `#1295` | `Daniel / Codex.app` | closed |
| 4 | Shared ObsMem foundation | `#1297` | `Daniel / Codex.app` | closed |

### Exit Criteria
- canonical `v0.87` milestone docs are aligned to the roadmap and seeded execution plan
- Trace v1 schema exists as a real implementation surface
- provider substrate v1 exists as a real implementation surface
- shared ObsMem foundation exists as a real implementation surface

## Sprint 2

### Goal
Expand the foundational substrate into a connected milestone surface: runtime trace export, provider portability, trace-memory coherence, operational skills, control-plane consolidation, tooling hardening, and review formalization.

### Scope
- trace instrumentation and artifact linkage
- provider portability and config compatibility
- trace-to-memory coherence
- operational skills substrate
- PR tooling / control-plane consolidation
- tooling hardening and workflow stability
- review-surface formalization

### Work Packages
| Order | Item | Issue | Owner | Status |
|---|---|---|---|---|
| 1 | Trace instrumentation + artifact linkage | `#1294` | `Daniel / Codex.app` | closed |
| 2 | Provider portability + config compatibility | `#1296` | `Daniel / Codex.app` | closed |
| 3 | Trace-to-memory coherence | `#1298` | `Daniel / Codex.app` | closed |
| 4 | Operational skills substrate | `#1299` | `Daniel / Codex.app` | closed |
| 5 | PR tooling / control-plane consolidation | `#1300` | `Daniel / Codex.app` | closed |
| 6 | Tooling hardening + workflow stability | `#1301` | `Daniel / Codex.app` | closed |
| 7 | Review-surface formalization | `#1302` | `Daniel / Codex.app` | closed |

### Exit Criteria
- runtime Trace v1 export and artifact linkage are real and reviewable
- provider substrate is threaded through real config and compatibility surfaces
- memory entries can be tied back to execution truth
- at least one real operational skill path exists
- control-plane/tooling behavior is more centralized and stable
- review outputs use a real substrate contract

## Sprint 3

### Goal
Converge the milestone into a reviewer-legible package with truthful demos, quality gates, docs, internal review outputs, 3rd party review readiness, release closeout, and an explicit next-milestone planning handoff.

### Scope
- docs canonicalization and feature index truth
- demo matrix execution and integration demos
- quality gate and coverage posture
- docs / internal review convergence
- 3rd party review pass and resulting follow-up capture
- review findings remediation
- release ceremony and milestone handoff
- canonical `v0.87.1` milestone shell and planning package before `v0.87` closes

### Work Packages
| Order | Item | Issue | Owner | Status |
|---|---|---|---|---|
| 1 | Documentation canonicalization + feature index | `#1345` | `Daniel / Codex.app` | closed |
| 2 | Demo matrix + integration demos | `#1346` | `Daniel / Codex.app` | closed |
| 3 | Coverage / quality gate | `#1347` | `Daniel / Codex.app` | closed |
| 4 | Docs + internal review pass | `#1348` | `Daniel / Codex.app` | open |
| 5 | 3rd party review pass | `#1349` | `Daniel / Codex.app` | open |
| 6 | Release ceremony | `#1350` | `Daniel / Codex.app` | open |
| 7 | Next milestone planning (`v0.87.1`) | `#1354` | `Daniel / Codex.app` | closed |
| 8 | Review findings remediation | `#1414` | `Daniel / Codex.app` | open |

### Exit Criteria
- canonical docs truthfully describe the implemented milestone
- demo matrix is runnable or explicitly blocked with alternate proof surfaces
- quality posture is recorded with real commands and justified exceptions
- docs and internal review surfaces are coherent for an uninvolved reviewer
- 3rd party review is completed or explicitly recorded with bounded follow-up disposition
- accepted review findings are remediated or explicitly deferred
- release-tail validation and handoff are complete
- the canonical `v0.87.1` milestone shell exists before `v0.87` closes

## Current Execution Status

As of this plan revision:
- Sprint 1 is implemented and closed at the issue level
- Sprint 2 is implemented and closed at the issue level
- Sprint 3 is live: `#1345`, `#1346`, `#1347`, and `#1354` are closed; `#1348`, `#1349`, `#1414`, and `#1350` remain open in the review/remediation/release tail

Current issue posture:
- `#1292` through `#1302` are closed
- `#1345`, `#1346`, and `#1347` are closed
- `#1348`, `#1349`, `#1414`, and `#1350` are open
- `#1354` is closed

## Cadence Expectations
- Use issue cards (`input` / `output`) for every execution item.
- Keep each issue mergeable, narrow, and truthfully documented.
- Prefer substrate-first sequencing:
  - Sprint 1: schema + provider + shared-memory foundation
  - Sprint 2: linkage + compatibility + skills + tooling + review
  - Sprint 3: docs + demos + quality + release tail + next-milestone planning
- Run required quality gates (`fmt`, `clippy`, `test`, and any validator/demo command relevant to the changed surface).
- Record proof surfaces as they land instead of reconstructing them later from memory.

## Risks / Dependencies
- Dependency: late `v0.86` fallout can still force local doc and workflow adjustments.
  - Risk: milestone docs drift while early `v0.87` work is underway.
  - Mitigation: keep canonical docs synchronized to the issue spine and real PR state.

- Dependency: trace, provider, and memory work are tightly coupled.
  - Risk: later WP ownership gets blurred and issues silently absorb adjacent scope.
  - Mitigation: keep WP boundaries explicit and record follow-ons instead of widening patches.

- Dependency: tooling/control-plane work can sprawl.
  - Risk: bounded workflow fixes turn into a broad rewrite.
  - Mitigation: keep Sprint 2 control-plane work as narrow consolidation and hardening slices.

## Demo / Review Plan
- Primary demo artifact: `docs/milestones/v0.87/DEMO_MATRIX_v0.87.md`
- Reviewer entry surfaces:
  - `docs/milestones/v0.87/README.md`
  - `docs/milestones/v0.87/WBS_v0.87.md`
  - `docs/milestones/v0.87/FEATURE_DOCS_v0.87.md`
  - `docs/milestones/v0.87/DEMO_MATRIX_v0.87.md`
- Sign-off owners: `Daniel Austin`, `Codex.app`, internal review, and 3rd party review before release closeout

## Milestone Exit Criteria
- Canonical `v0.87` milestone docs are filled, internally consistent, and aligned with the roadmap.
- All three milestone sprints are described truthfully in this file.
- The issue sequence is explicit for the foundational and convergence substrate work.
- Sprint 3 release-tail work is defined before release closeout begins.
- Internal review and 3rd party review are both completed or explicitly dispositioned before release closeout.
- Accepted review findings are remediated or explicitly deferred before release closeout.
- The `v0.87.1` planning shell exists before `v0.87` is considered fully closed.
- Scope remains bounded to `v0.87` substrate work; no silent pull-forward of `v0.88+` systems.
