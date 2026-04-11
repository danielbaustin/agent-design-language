# Design - v0.88

## Metadata
- Milestone: `v0.88`
- Version: `v0.88`
- Date: `2026-04-10`
- Owner: `Daniel Austin`
- Related issues: `#1527`, `#1579`

## Purpose
Define the milestone design boundary for the `v0.88` chronosense package.

## Problem Statement

ADL has accumulated substantial planning around time, continuity, identity, and cost-aware execution, but until now that work has lived primarily in local planning docs. The tracked milestone shell did not expose a coherent public feature set, leaving `v0.88` hard to review and easy to misread as an empty placeholder milestone.

## Goals
- define a bounded public `v0.88` package for chronosense and temporal substrate work
- make temporal schema, continuity, retrieval, commitment, causality, and cost-policy surfaces reviewable in tracked docs
- tie execution-policy and cost semantics back to trace-level reviewability

## Non-Goals
- final implementation sequencing for every downstream `v0.88` issue
- broader governance, instinct, aptitude, or multi-agent society systems
- later temporal features such as cross-agent alignment, forks, or accountability

## Scope

### In scope
- chronosense as foundational temporal self-location
- canonical temporal schema and anchor contracts
- continuity and identity semantics grounded in time
- temporal query/retrieval and staleness-aware access
- commitments and deadlines as persistent temporal records
- bounded temporal causality and explanation
- execution mode / compute policy and realized cost semantics

### Out of scope
- cross-agent temporal alignment
- counterfactual branch/timeline semantics
- full temporal accountability and reputation interpretation
- broader higher-order constitutional and social systems

## Proposed Design

The promoted `v0.88` package is organized as one coherent temporal band:

- `SUBSTANCE_OF_TIME.md` defines why chronosense matters
- `TEMPORAL_SCHEMA_V01.md` defines the canonical temporal and execution-policy fields
- `CHRONOSENSE_AND_IDENTITY.md` defines continuity and identity semantics
- `TEMPORAL_QUERY_AND_RETRIEVAL.md` defines time-aware retrieval behavior
- `COMMITMENTS_AND_DEADLINES.md` defines future obligations and deadline states
- `TEMPORAL_CAUSALITY_AND_EXPLANATION.md` defines bounded explanatory causality
- `ADL_COST_MODEL.md` defines execution mode, realized cost, and economics

## Validation Plan
- promoted docs exist under `docs/milestones/v0.88/features/`
- `FEATURE_DOCS_v0.88.md` indexes the promoted set
- milestone shell docs point to the promoted package truthfully
- placeholder-heavy shell wording is replaced with bounded milestone truth
