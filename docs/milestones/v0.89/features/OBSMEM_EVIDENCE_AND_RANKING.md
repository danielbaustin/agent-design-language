# ObsMem Evidence And Ranking

## Metadata
- Milestone: `v0.89`
- Status: `Planned`
- Source planning input: local `v0.89` planning corpus
- Planned WP: `WP-08`

## Purpose

Make ObsMem retrieval evidence-aware, explainable, and fit for governed reasoning rather than only deterministic storage/retrieval.

## Scope

`v0.89` should establish:
- explicit ranking signal families
- evidence categories
- explanation-bearing retrieval results
- provenance-aware tie-break and trust behavior

## Main Runtime Commitments

- retrieval ranking remains deterministic but becomes more legible
- provenance and evidence classes affect ranking in named ways
- later AEE, experiment, and governance work can cite retrieval explanations rather than hidden ranking behavior

## Non-Goals

- the full later four-layer memory model
- rich identity-linked memory semantics

## Dependencies

- existing ObsMem baseline from `v0.87`
- Godel Experiment System
- future reasoning and governance consumers

## Exit Criteria

- the milestone package defines what evidence-aware retrieval means in ADL
- ranking/explanation outputs are concrete enough to drive later issue wave seeding
