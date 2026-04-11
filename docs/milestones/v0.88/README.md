# v0.88 Milestone Docs

## Metadata
- Milestone: `v0.88`
- Version: `0.88`
- Date: `2026-04-11`
- Owner: `Daniel Austin / Agent Logic`

## Purpose
This directory is the canonical tracked documentation set for `v0.88`.

`v0.88` is the milestone where ADL turns two previously scattered planning bands into one coherent public package:
- temporal / chronosense substrate
- instinct / bounded-agency substrate

The milestone is not supposed to be a speculative catch-all. It is a bounded planning and execution package for the next real feature wave after `v0.87.1`.

## Overview

`v0.88` focuses on making cognition more structurally coherent and reviewable.

This milestone centers on:
- chronosense as a first-class substrate
- explicit temporal schema and anchors
- continuity and identity semantics grounded in time
- time-aware retrieval, commitments, and bounded temporal explanation
- execution-policy and cost semantics carried in trace
- instinct as a bounded runtime influence
- bounded agency hooks that remain deterministic, inspectable, and policy-constrained

Key outcomes:
- one coherent tracked `v0.88` planning shell
- a promoted feature-doc package that includes both temporal and instinct surfaces
- a WBS and sprint plan that follow the same closeout pattern used in `v0.86` and `v0.87`
- clear separation between tracked milestone canon and local-only exploratory planning notes

## Scope Summary

### In scope
- chronosense and temporal self-location
- temporal anchors, clocks, and trace-linked execution policy
- continuity and identity semantics tied to time
- temporal query and retrieval behavior
- commitments and deadlines
- bounded temporal causality and explanation
- execution mode / cost model semantics
- instinct as a bounded, inspectable runtime influence
- bounded agency proof surfaces where instinct affects routing or prioritization

### Out of scope
- cross-agent temporal alignment
- timeline forks and counterfactual reasoning
- temporal accountability and later social/governance interpretation
- full persistent identity guarantees
- fully developed aptitude benchmarking
- exploratory planning notes that are not yet canonical feature docs

## Document Map

Canonical milestone documents:
- Vision: `VISION_v0.88.md`
- Design: `DESIGN_v0.88.md`
- Work Breakdown Structure (WBS): `WBS_v0.88.md`
- Sprint plan: `SPRINT_v0.88.md`
- Decisions log: `DECISIONS_v0.88.md`
- Demo matrix: `DEMO_MATRIX_v0.88.md`
- Milestone checklist: `MILESTONE_CHECKLIST_v0.88.md`
- Release plan: `RELEASE_PLAN_v0.88.md`
- Release notes: `RELEASE_NOTES_v0.88.md`

Promoted feature-defining docs:
- `FEATURE_DOCS_v0.88.md`
- `features/SUBSTANCE_OF_TIME.md`
- `features/TEMPORAL_SCHEMA_V01.md`
- `features/CHRONOSENSE_AND_IDENTITY.md`
- `features/TEMPORAL_QUERY_AND_RETRIEVAL.md`
- `features/COMMITMENTS_AND_DEADLINES.md`
- `features/TEMPORAL_CAUSALITY_AND_EXPLANATION.md`
- `features/ADL_COST_MODEL.md`
- `features/INSTINCT_MODEL.md`
- `features/INSTINCT_RUNTIME_SURFACE.md`

Local planning docs intentionally not promoted yet:
- historical / transitional: `.adl/docs/v0.88planning/APTITUDE_MODEL.md`
- exploratory metrics: `.adl/docs/v0.88planning/PHI_METRICS_FOR_ADL.md`
- helper maps and planning notes: `.adl/docs/v0.88planning/TEMPORAL_CLUSTER_MAP.md`, `.adl/docs/v0.88planning/RUNTIME_PROVIDER_AND_ECONOMICS_CLUSTER_MAP.md`, `.adl/docs/v0.88planning/WP_INSTINCT_AND_BOUNDED_AGENCY.md`

## Execution Model

`v0.88` should follow the same closeout structure used in `v0.86` and `v0.87`:
- bounded feature work packages first
- demo and proof surfaces next
- quality gate, docs/review convergence, internal review, 3rd-party review, remediation, release ceremony, and next-milestone planning last

The milestone should not invent an extra process sprint beyond that established pattern.

## Status

Current status: **tracked planning package reconciled; execution issue wave pending**

- Planning shell: active
- Promoted feature package: present
- Issue wave: not yet seeded for the substantive `v0.88` work packages
- Demo/review/release surfaces: seeded and aligned to the normal milestone pattern, but not yet populated with implementation evidence

## Exit Criteria

- canonical `v0.88` milestone docs are internally consistent
- tracked feature docs match the intended bounded milestone scope
- local-only exploratory docs are not silently treated as canonical milestone promises
- the `v0.88` issue wave can be seeded from this package without re-litigating milestone scope
