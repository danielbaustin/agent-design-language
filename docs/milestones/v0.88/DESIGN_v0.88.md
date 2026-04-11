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

The tracked `v0.88` milestone docs were seeded correctly, but they underrepresented the actual local planning package.

That left two defects:
- the tracked milestone read like a thin pre-execution package instead of a coherent next-milestone package
- real planned `v0.88` feature surfaces, especially instinct and bounded-agency docs, remained local-only even though they belong to the milestone boundary

The design task for `v0.88` is therefore to define one bounded public package without sweeping in every exploratory planning note.

## Goals
- define one coherent public `v0.88` package for temporal / chronosense and instinct / bounded-agency work
- make the tracked milestone package match the intended scope of the promoted feature set
- preserve the prior closeout pattern used in `v0.86` and `v0.87` instead of inventing a new process layer
- keep local-only exploratory planning notes out of tracked canon unless they are ready to be treated as real milestone promises
- ensure every major milestone claim maps to eventual runtime, trace, demo, or review surfaces
- close scope cleanly so execution issue seeding can proceed without further milestone expansion

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
- helper cluster-map docs
- later social/governance temporal work
- full persistent identity guarantees

### Accepted bounded pull-ins
- the temporal/deadline pressure slice from `SENSE_OF_URGENCY_AND_TASK_PRIORITIZATION.md` via backlog issue `#1614`
- the bounded comparative-demo / positioning slice from `DEEP_AGENTS_AND_ADL.md` via backlog issue `#1618`

## Requirements

### Functional
- define the chronosense substrate as an explicit architectural surface rather than scattered temporal notes
- define a temporal schema that can carry anchors, clocks, execution posture, and realized cost
- define continuity and identity semantics that can be evaluated from temporal structure and trace
- define time-aware retrieval, commitments, deadlines, and bounded temporal explanation as concrete runtime-facing surfaces
- define PHI-style integration metrics as an engineering comparison surface for low-, medium-, and high-integration execution paths
- define instinct as a bounded runtime influence with inspectable declaration and observable downstream effect
- define at least one bounded-agency proof path where instinct affects prioritization or routing without escaping policy control
- define one flagship multi-agent demo that ties temporal continuity, artifact truth, and bounded orchestration together in a public-facing showcase
- define the demo and artifact surfaces required to prove the milestone truthfully

### Non-functional
- milestone claims must remain reviewable through trace, artifacts, or explicit proof surfaces
- execution policy, cost, PHI, and instinct surfaces must be interpretable by reviewers rather than hidden in implementation detail
- docs must not silently absorb later identity, governance, or multi-agent scope
- the milestone must produce a bounded public package that can be executed issue-by-issue without re-litigating scope

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

### Flagship Demo Surface
- `PAPER_SONATA_DEMO.md`
- `PAPER_SONATA_IMPLEMENTATION_PLAN.md`

### Local Planning Inputs, Not Canonical Tracked Features
- `v0.89planning/APTITUDE_MODEL.md` as later-band learning / skills planning
- `TEMPORAL_CLUSTER_MAP.md`
- `RUNTIME_PROVIDER_AND_ECONOMICS_CLUSTER_MAP.md`
- `WP_INSTINCT_AND_BOUNDED_AGENCY.md` as the local WP note that fed the instinct docs and WBS shape

### Integrated Milestone Design

`v0.88` is not just a pile of feature docs. It is one bounded architectural story:

temporal structure -> continuity semantics -> temporal retrieval / commitments -> execution policy and cost reviewability -> PHI-style integration comparison -> instinct-sensitive routing or prioritization -> bounded proof artifacts

The flagship public-facing expression of that story should be `Paper Sonata`: a bounded research-writing workflow where role handoffs, artifact flow, and runtime truth are all visible.

That means the milestone should deliver a runtime and review story where:
- time is explicit in the execution model
- continuity can be reasoned about from traceable temporal structure
- execution mode and realized cost are visible together
- integration can be discussed without pretending to solve consciousness
- instinct is real enough to affect behavior but still bounded by policy and determinism
- one memorable flagship demo makes the milestone legible to audiences beyond internal reviewers

### Core Design Principles

#### 1. Time is architectural, not incidental
Time must appear in schema, retrieval, commitments, and review surfaces. It cannot remain only a narrative theme.

#### 2. Trace is the main review surface
Execution posture, realized cost, temporal anchors, and instinct-related effects must be reviewable in artifacts or trace-derived outputs.

#### 3. PHI is an engineering metric family, not metaphysics
`PHI_METRICS_FOR_ADL.md` exists to help compare integration and coupling across execution paths. It should inform design and demos, not overclaim a final theory.

#### 4. Instinct is bounded influence, not theatrical autonomy
Instinct should affect prioritization or routing in visible ways while remaining deterministic, policy-constrained, and reviewable.

#### 5. v0.88 prepares later bands without stealing their scope
Identity, learning, governance, and multi-agent society should become easier after `v0.88`, but they should not be smuggled into this milestone as implicit commitments.

### Interfaces / Review Contracts

#### Temporal / Cost Review Surface
Minimum expected surface:
- temporal anchors or timestamps
- execution policy / mode
- realized cost dimensions
- enough structure to compare planned posture vs actual behavior

#### PHI Review Surface
Minimum expected surface:
- at least one comparison between differently integrated execution paths
- named dimensions for coupling / integration
- reviewer-readable explanation of what changed and why it matters

#### Instinct Review Surface
Minimum expected surface:
- declared instinct settings
- affected candidate actions, routes, or priorities
- evidence that policy bounds remained in force

#### Demo / Artifact Surface
Minimum expected surface:
- runnable command or clearly defined execution path
- expected artifact or trace output
- one-sentence explanation of what the proof row demonstrates

## Execution Semantics

The expected milestone flow is:
1. establish temporal structure and schema
2. connect continuity, retrieval, commitments, and causality to that structure
3. expose execution policy and realized cost through the same review surfaces
4. compare execution paths using PHI-style integration metrics
5. introduce instinct as a bounded influence on routing or prioritization
6. prove the resulting behavior through demos and reviewer-facing artifacts

## Risks and Mitigations

- Risk: temporal docs stay conceptual and never become execution-facing
  - Mitigation: require schema, retrieval, cost, and demo surfaces to line up explicitly
- Risk: PHI becomes rhetorical or pseudo-theoretical
  - Mitigation: constrain it to engineering comparison surfaces and bounded demos
- Risk: instinct becomes decorative language rather than real behavior
  - Mitigation: require at least one inspectable proof path where instinct affects routing or prioritization
- Risk: later-band identity / governance / learning ideas leak into `v0.88`
  - Mitigation: keep aptitude, governance, and multi-agent documents out of tracked `v0.88` canon
- Risk: docs become cleaner without actually setting up execution
  - Mitigation: tie WPs to concrete code/demo issue seeding and reviewer-facing proof rows

## Alternatives Considered

- Keep `v0.88` as a thin temporal-only milestone
  - Tradeoff: simpler package, but instinct and PHI work stay fragmented and local-only
- Push PHI metrics to a later milestone
  - Tradeoff: cleaner short-term scope, but loses a useful chance to learn how integrated cognition should be evaluated
- Push instinct entirely to later governance/identity work
  - Tradeoff: cleaner chronology, but bounded-agency design remains unproven as a runtime surface

## Validation Plan
- promoted docs exist under `docs/milestones/v0.88/features/`
- `FEATURE_DOCS_v0.88.md` indexes the promoted set truthfully
- milestone planning docs no longer contradict the promoted package or the intended closeout pattern
- tracked docs clearly separate canonical feature docs from local-only planning notes
- eventual implementation issues and demo issues can be derived directly from the WBS without additional scope rewrites
- the issue map distinguishes completed planning/package issues from the still-pending execution and closeout issue wave
- the planning package explicitly states that scope closure has been reached

## Exit Criteria
- goals and non-goals are explicit
- tracked `v0.88` docs are internally coherent
- feature-doc coverage matches intended milestone scope
- the planning package is strong enough to seed the real issue wave without another structural rewrite
