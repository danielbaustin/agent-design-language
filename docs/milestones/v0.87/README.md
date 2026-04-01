# Milestone README: v0.87

## Metadata
- Milestone: `v0.87`
- Version: `0.87`
- Date: `2026`
- Owner: `adl`

## Purpose
Provide the single canonical entry point for `v0.87`: what the milestone is, why it matters, what is in scope, how to navigate the canonical docs, and which proof surfaces a reviewer should inspect first.

This README is intentionally concise and navigational. It should help an uninvolved reviewer understand `v0.87` as a **substrate milestone** without reconstructing context from issue threads or chat history.

## Overview

`v0.87` is the milestone where ADL moves from a bounded cognitive system that was proven in pieces in `v0.86` to a more **coherent, deterministic, and externally credible substrate**.

This milestone focuses on:
- trace as execution truth
- provider / transport normalization and portability
- shared ObsMem foundation and trace-linked memory coherence
- operational skills and control-plane/tooling stabilization
- reviewer-facing proof surfaces and canonical milestone docs

Key outcomes:
- a canonical substrate spine: `contracts -> execution -> trace -> review -> documentation`
- real, bounded proof surfaces for trace, provider portability, shared memory, skills, and control-plane behavior
- milestone docs and review surfaces that truthfully reflect implementation and are usable by an uninvolved reviewer

## Scope Summary

### In scope
- Trace v1 schema, event model, and runtime/control-surface emission
- Provider / transport substrate v1 with explicit `vendor`, `transport`, and `model_ref` separation
- Shared ObsMem foundation tied to execution/trace truth
- Operational skills substrate and structured review outputs
- Control-plane / PR tooling stabilization and workflow determinism
- Demo matrix, docs, review package, and release-tail truth surfaces

### Out of scope
- Persistent identity, chronosense, and later `v0.9+` personhood/continuity work
- PR Demo execution, capability-aware routing, and later governance / delegation / Freedom Gate evolution

## Start Here

If you are new to this milestone, read in this order:

1. `docs/milestones/v0.87/VISION_v0.87.md`
2. `docs/milestones/v0.87/DESIGN_v0.87.md`
3. `docs/milestones/v0.87/WBS_v0.87.md`
4. `docs/milestones/v0.87/DEMO_MATRIX_v0.87.md`
5. `docs/milestones/v0.87/MILESTONE_CHECKLIST_v0.87.md`

If you want the fastest reviewer path, jump to:
- demo matrix: `docs/milestones/v0.87/DEMO_MATRIX_v0.87.md`
- milestone checklist: `docs/milestones/v0.87/MILESTONE_CHECKLIST_v0.87.md`
- release plan: `docs/milestones/v0.87/RELEASE_PLAN_v0.87.md`

## Document Map

Canonical milestone documents:
- Vision: `docs/milestones/v0.87/VISION_v0.87.md`
- Design: `docs/milestones/v0.87/DESIGN_v0.87.md`
- Work Breakdown Structure (WBS): `docs/milestones/v0.87/WBS_v0.87.md`
- Sprint plan: `docs/milestones/v0.87/SPRINT_v0.87.md`
- Decisions log: `docs/milestones/v0.87/DECISIONS_v0.87.md`
- Demo matrix: `docs/milestones/v0.87/DEMO_MATRIX_v0.87.md`
- Milestone checklist: `docs/milestones/v0.87/MILESTONE_CHECKLIST_v0.87.md`
- Release plan: `docs/milestones/v0.87/RELEASE_PLAN_v0.87.md`
- Release notes: `docs/milestones/v0.87/RELEASE_NOTES_v0.87.md`

Supporting / domain-specific docs:
- Feature docs index: `docs/milestones/v0.87/FEATURE_DOCS_v0.87.md`
- Roadmap context: `.adl/docs/roadmaps/ROAD_TO_v0.95.md`
- Review artifacts: `.adl/reviews/`

## Execution Model

This milestone is executed as a sequence of work packages (WPs):

- `WP-01`: design pass (docs + planning)
- `WP-02` – `WP-11`: substrate implementation bands
- `WP-12`: documentation canonicalization + feature index
- `WP-13`: demo matrix and integration demos
- `WP-14`: coverage / quality gate
- `WP-15`: docs and review convergence
- `WP-16`: release ceremony

Execution expectations:
- each WP is tracked by one or more issues and implemented through bounded PRs
- each issue should produce structured input/output cards and validation evidence
- the milestone should remain substrate-first and should not silently absorb `v0.88+` systems
- all claims should be backed by runnable demos, artifact roots, review outputs, or release-tail validation

## Demo and Validation Surface

Primary validation is defined in:
- demo matrix: `docs/milestones/v0.87/DEMO_MATRIX_v0.87.md`

Primary bounded demos:
- `D1`: Trace v1 substrate truth
- `D2`: Provider portability substrate
- `D3`: Shared ObsMem foundation coherence
- `D4`: Operational skills substrate
- `D5`: Control-plane / PR tooling substrate
- `D6`: Reviewer-facing substrate package

Primary artifact root:
- `artifacts/v087/`

Success criteria:
- each major substrate claim maps to a bounded demo or explicit alternate proof surface
- a reviewer can identify the first command, primary artifact, and expected success signal for each demo
- no demo inflates later-milestone capabilities that are intentionally out of scope

## Determinism and Reproducibility

`v0.87` should demonstrate:
- deterministic or bounded-repeatable execution where required
- stable structured schemas and event vocabularies
- replayable or inspectable substrate artifacts
- stable command entry points once demos land

Evidence locations:
- `artifacts/v087/`
- issue output cards for implementing/demo/validation issues
- `.adl/reviews/` for internal review outputs

Determinism notes:
- determinism for this milestone is judged primarily by stable structure, schemas, event vocabularies, and proof-surface truth
- timestamps, run IDs, or other documented runtime-generated metadata may vary and should not be treated as failures by themselves

## Risks and Open Questions

Known risks:
- provider and tooling redesign may sprawl if not kept tightly bounded to substrate v1 goals
- late `v0.86` closeout fixes may temporarily force small roadmap/doc adjustments while `v0.87` planning begins

Open questions:
- which exact issue sequence should start the first implementation-heavy slice after doc seeding
- whether trace v1 or control-plane consolidation should land first as the strongest initial substrate proof
- what the minimum provider set is for a credible portability claim in `v0.87`

## Status

Current status: `PLANNING / EARLY EXECUTION`

- Planning: canonical doc set seeded and being reviewed
- Execution: first `v0.87` issue sequence is being prepared
- Validation: demo matrix and checklist seeded; concrete command surfaces still landing
- Release readiness: not ready yet; this README describes the intended canonical review path once issues land

## Exit Criteria

- All canonical milestone documents are complete and internally consistent.
- All in-scope WBS items are implemented or explicitly deferred.
- Demo matrix is runnable and validated or blocked truthfully with alternate proof surfaces.
- Quality gates, review surfaces, and release-tail evidence are recorded truthfully.
- A skeptical uninvolved reviewer can use this README to locate the first doc, first command, and primary proof surfaces without additional reconstruction.
