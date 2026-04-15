# ObsMem Evidence And Ranking

## Metadata
- Milestone: `v0.89`
- Status: `Landed`
- Source planning input: local `v0.89` planning corpus
- Milestone home: `WP-08`

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

## Runtime Contract

`WP-08` now uses the bounded ObsMem retrieval demo as the reviewer-facing proof entrypoint
 instead of leaving evidence-aware ranking implicit inside policy tests alone.

The bounded proof entrypoint is:

```bash
cargo run --manifest-path adl/Cargo.toml -- demo demo-f-obsmem-retrieval --run --trace --out ./out
```

The reviewer-facing proof surfaces are:
- `runs/_shared/obsmem_store.v1.json`
- `obsmem_retrieval_result.json`
- the bounded experiment/runtime scaffolding that feeds the retrieval query:
  - `runs/demo-f-run-a/godel/experiment_record.runtime.v1.json`
  - `runs/demo-f-run-a/godel/obsmem_index_entry.runtime.v1.json`
  - `runs/demo-f-run-b/godel/experiment_record.runtime.v1.json`
  - `runs/demo-f-run-b/godel/obsmem_index_entry.runtime.v1.json`
  - `runs/demo-f-run-c/godel/experiment_record.runtime.v1.json`
  - `runs/demo-f-run-c/godel/obsmem_index_entry.runtime.v1.json`

`obsmem_retrieval_result.json` now records:
- the normalized query and policy order
- the returned memory records
- explicit ranking explanations for each hit, including:
  - prior vs effective score
  - matched query tags and failure-code matches
  - status-based boosts or penalties
  - provenance paths and provenance families
  - trace-event reference counts
  - deterministic tie-break values

This keeps `WP-08` bounded to evidence-aware retrieval ranking and reviewer-legible explanation.
It does not claim the later full memory architecture, adversarial trust model, or identity-linked
 memory semantics that belong downstream.

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
- the bounded D6 proof surface makes ranking reasons and provenance legible without requiring a
  reviewer to reverse-engineer retrieval policy from code
