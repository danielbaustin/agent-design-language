# v0.86 Milestone Docs

## Metadata
- Milestone: `v0.86`
- Version: `0.86`
- Date: `2026-04-01`
- Owner: `Daniel Austin / Agent Logic`

## Purpose
This directory is the canonical tracked documentation set for **v0.86**, including the promoted feature-defining docs under `docs/milestones/v0.86/features/`.

v0.86 is the milestone where ADL establishes its **first working bounded cognitive system**. The milestone is not about landing every later cognitive concept at once. It is about making the full bounded cognitive loop real, bounded, inspectable, and demonstrable.

Use this README as the single entry point for understanding:
- what v0.86 is
- what it includes and excludes
- how the milestone is structured
- where to find the canonical design, execution, validation, and release surfaces

## Overview

v0.86 moves ADL from repository stabilization into the first explicit **bounded cognitive architecture**.

This milestone focuses on:
- the canonical cognitive stack
- the canonical cognitive loop
- cognitive signals (instinct + affect)
- cognitive arbitration
- fast/slow reasoning control
- bounded agency via candidate selection
- bounded execution (AEE-lite)
- evaluation and termination signals
- minimal reframing / frame adequacy
- memory participation (ObsMem-lite)
- Freedom Gate decision control
- local demos and artifact traces that prove the full cognitive loop is real

Key outcomes:
- one authoritative bounded cognitive path for the milestone
- observable signals, arbitration, candidate selection, execution, evaluation, reframing, memory participation, and Freedom Gate behavior
- runnable local demo proof surfaces for the full loop
- milestone docs that match implementation truthfully

## Scope Summary

### In scope
- canonical cognitive stack definition
- canonical cognitive loop definition
- cognitive signals (instinct + affect)
- cognitive arbitration and fast/slow routing
- bounded agency via candidate selection
- bounded execution (AEE-lite)
- evaluation and termination signals
- minimal reframing / frame adequacy
- memory participation (ObsMem-lite)
- Freedom Gate (minimal allow / defer / refuse surface)
- local demos, demo matrix, and structured proof artifacts
- milestone docs, review surfaces, and release mechanics

### Out of scope
- PHI / Φ_ADL metrics
- advanced AEE convergence or iterative-improvement systems beyond bounded execution
- advanced reframing or meta-reasoning systems beyond minimal bounded adaptation
- long-horizon or session-persistent memory systems beyond ObsMem-lite participation
- richer affect, identity, governance, or signed-trace systems beyond v0.86 bounded scope

## Document Map

Canonical milestone documents:
- Vision: `VISION_v0.86.md`
- Design: `DESIGN_v0.86.md`
- Work Breakdown Structure (WBS): `WBS_v0.86.md`
- Sprint plan: `SPRINT_v0.86.md`
- Decisions log: `DECISIONS_v0.86.md`
- Demo matrix: `DEMO_MATRIX_v0.86.md`
- Milestone checklist: `MILESTONE_CHECKLIST_v0.86.md`
- Release plan: `RELEASE_PLAN_v0.86.md`
- Release notes: `RELEASE_NOTES_v0.86.md`

Promoted feature-defining docs:
- `docs/milestones/v0.86/FEATURE_DOCS_v0.86.md`
- `docs/milestones/v0.86/FUTURE_FEATURE_HANDOFF_v0.86.md`
- `docs/milestones/v0.86/features/AGENCY_AND_AGENTS.md`
- `docs/milestones/v0.86/features/COGNITIVE_ARBITRATION.md`
- `docs/milestones/v0.86/features/COGNITIVE_LOOP_MODEL.md`
- `docs/milestones/v0.86/features/COGNITIVE_STACK.md`
- `docs/milestones/v0.86/features/FAST_SLOW_THINKING_MODEL.md`
- `docs/milestones/v0.86/features/FREEDOM_GATE.md`
- `docs/milestones/v0.86/features/LOCAL_AGENT_DEMOS.md`
- `docs/milestones/v0.86/features/CONCEPT_PLANNING_FOR_v0.86.md`

Context / supplementary planning notes:
- `.adl/docs/v0.86planning/INTELLECTUAL_INFLUENCES.md`
- `.adl/docs/v0.86planning/VISION_NOTES__INSTINCT_AGENCY_AND_CIVILIZING_LLMS.md`

Interpretation note:
- promoted feature docs may preserve future-facing architectural context
- `FUTURE_FEATURE_HANDOFF_v0.86.md` defines which parts remain bounded `v0.86` commitments and which are explicitly handed forward

## Execution Model

This milestone is executed as an ordered set of work packages (WPs):

- `WP-01` design pass (canonical docs + planning)
- `WP-02` cognitive stack canonicalization
- `WP-03` cognitive loop canonicalization
- `WP-04` cognitive signals
- `WP-05` cognitive arbitration
- `WP-06` fast / slow thinking paths
- `WP-07` agency and candidate selection
- `WP-08` bounded execution (AEE-lite)
- `WP-09` evaluation signals and termination
- `WP-10` frame adequacy and reframing
- `WP-11` memory participation (ObsMem-lite)
- `WP-12` Freedom Gate (v0.86 minimal)
- `WP-13` canonical bounded cognitive path
- `WP-14` artifact schema enforcement
- `WP-15` local agent demo program
- `WP-16` demo matrix and review surface
- `WP-17` coverage / quality gate
- `WP-18` docs + review pass
- `WP-19` internal review
- `WP-20` external / 3rd-party review preparation
- `WP-21` review findings remediation
- `WP-22` release ceremony
- `WP-23` next milestone planning

Execution expectations:
- Every WP is tied to a concrete milestone outcome.
- Every major milestone claim must map to artifacts, demos, traces, or other reviewable outputs.
- There must be exactly one authoritative bounded cognitive path for the milestone.
- If docs and implementation diverge, that divergence is a defect.

## Demo And Validation Surface

Primary validation is defined in:
- `DEMO_MATRIX_v0.86.md`

Primary milestone proof surface:
- **D1 — Canonical Bounded Cognitive Path**

Additional validation surfaces:
- local demo scripts under `adl/tools/`
- generated artifact outputs under `artifacts/v086/` after running the demo scripts
- generated `D5` review-surface manifest at `artifacts/v086/review_surface/demo_manifest.json`
- quality-gate evidence from tests / coverage / CI
- internal and external review records

Success criteria:
- the bounded cognitive path is runnable end-to-end
- signals influence behavior and routing
- arbitration decisions are visible and reviewable
- agency is observable through candidate selection
- bounded execution performs at least one iteration
- evaluation signals affect behavior or termination
- reframing/adaptation occurs in at least one case
- memory participation is visible
- Freedom Gate behavior is real and inspectable
- demo matrix and artifacts match actual implementation truthfully

## Determinism And Reproducibility

The milestone should demonstrate:
- deterministic or bounded-repeatable cognitive behavior where required
- replayable artifact traces for the local demo program
- stable command entry points for milestone review

Evidence locations:
- generated outputs under `artifacts/v086/`
- generated review-surface manifest at `artifacts/v086/review_surface/demo_manifest.json`
- `docs/milestones/v0.86/DEMO_MATRIX_v0.86.md`
- `docs/milestones/v0.86/MILESTONE_CHECKLIST_v0.86.md`

## Risks And Open Questions

Known risks:
- conceptual drift between planning docs and runtime implementation
- demos proving isolated components instead of the integrated cognitive path
- accidental reintroduction of unbounded later-milestone variants into v0.86
- artifact schemas drifting across demos, traces, and runtime surfaces

Open questions:
- what is the minimal long-term stable artifact schema for the cognitive path?
- how should arbitration confidence be normalized across different local models?

## Status

Current status: **implementation landed / Sprint 7 review-and-release tail in progress**

- Planning: complete
- Execution: implemented through the bounded cognitive path, artifact enforcement, and local demo/review surfaces
- Validation: local demo program and quality-gate proof surfaces are present for D1-D5 and Sprint 7 quality review
- Release readiness: pending docs/review convergence, review sign-off, and release ceremony closeout

## Exit Criteria

- All canonical milestone documents are complete and internally consistent.
- All WBS items are implemented or explicitly deferred with clear ownership.
- Demo matrix is runnable and validated.
- Quality gates are passing or exceptions are documented.
- The bounded cognitive path is demonstrable and inspectable end-to-end.
- Signals, execution, evaluation, reframing, memory participation, and Freedom Gate behavior are visible in proof surfaces.
- Release artifacts (notes, tag, docs) are ready.
