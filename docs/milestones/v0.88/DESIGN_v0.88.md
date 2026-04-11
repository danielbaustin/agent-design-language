# Design - v0.88

## Metadata
- Milestone: `v0.88`
- Version: `v0.88`
- Date: `2026-04-11`
- Owner: `Daniel Austin`
- Related issues: `#1527`, `#1579`, `#1497`

## Purpose
Define the milestone design boundary for the `v0.88` package.

## Problem Statement

The tracked `v0.88` milestone shell was seeded correctly, but it underrepresented the actual local planning package.

That left two defects:
- the tracked milestone read like a thin pre-execution shell instead of a coherent next-milestone package
- real planned `v0.88` feature surfaces, especially instinct and bounded-agency docs, remained local-only even though they belong to the milestone boundary

The design task for `v0.88` is therefore to define one bounded public package without sweeping in every exploratory planning note.

## Goals
- define one coherent public `v0.88` package for temporal / chronosense and instinct / bounded-agency work
- make the tracked milestone shell match the intended scope of the promoted feature set
- preserve the prior closeout pattern used in `v0.86` and `v0.87` instead of inventing a new process layer
- keep local-only exploratory planning notes out of tracked canon unless they are ready to be treated as real milestone promises

## Non-Goals
- promoting every file from `.adl/docs/v0.88planning/` into tracked docs
- treating historical or exploratory planning copies as canonical feature docs
- finalizing later-band work such as cross-agent temporal alignment, temporal accountability, or full aptitude benchmarking
- claiming implementation completion before the `v0.88` issue wave exists

## Scope

### In scope
- chronosense and temporal structure
- continuity and identity semantics
- temporal query and retrieval
- commitments and deadlines
- temporal causality and explanation
- execution policy and cost model semantics
- PHI-style engineering metrics for integration and irreducibility
- instinct model
- instinct runtime surface
- bounded-agency proof expectations tied back to instinct

### Out of scope
- historical aptitude planning copies
- exploratory PHI metrics as canonical milestone commitments
- helper cluster-map docs
- later social/governance temporal work
- full persistent identity guarantees

## Proposed Design

The promoted `v0.88` package is organized as two bounded feature bands.

### Temporal / Chronosense Band
- `SUBSTANCE_OF_TIME.md`
- `TEMPORAL_SCHEMA_V01.md`
- `CHRONOSENSE_AND_IDENTITY.md`
- `TEMPORAL_QUERY_AND_RETRIEVAL.md`
- `COMMITMENTS_AND_DEADLINES.md`
- `TEMPORAL_CAUSALITY_AND_EXPLANATION.md`
- `ADL_COST_MODEL.md`
- `PHI_METRICS_FOR_ADL.md`

### Instinct / Bounded-Agency Band
- `INSTINCT_MODEL.md`
- `INSTINCT_RUNTIME_SURFACE.md`

### Local Planning Inputs, Not Canonical Tracked Features
- `v0.89planning/APTITUDE_MODEL.md` as later-band learning / skills planning
- `TEMPORAL_CLUSTER_MAP.md`
- `RUNTIME_PROVIDER_AND_ECONOMICS_CLUSTER_MAP.md`
- `WP_INSTINCT_AND_BOUNDED_AGENCY.md` as the local WP note that fed the instinct docs and WBS shape

## Validation Plan
- promoted docs exist under `docs/milestones/v0.88/features/`
- `FEATURE_DOCS_v0.88.md` indexes the promoted set truthfully
- milestone shell docs no longer contradict the promoted package or the intended closeout pattern
- tracked docs clearly separate canonical feature docs from local-only planning notes

## Exit Criteria
- goals and non-goals are explicit
- tracked `v0.88` docs are internally coherent
- feature-doc coverage matches intended milestone scope
- the planning package is strong enough to seed the real issue wave without another structural rewrite
