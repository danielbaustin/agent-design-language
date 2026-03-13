# v0.8 Demo Matrix and Review Surfaces

This document defines the canonical review-entry matrix for v0.8 milestone review.

It covers both:
- runnable demo surfaces that a reviewer can execute, and
- inspect-only documentation/specification surfaces that a reviewer should read rather than run.

## Quick Reviewer Split

### Runnable demo commands

Run these commands from repository root:

```bash
cargo run --manifest-path tools/transpiler_demo/Cargo.toml --quiet
cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-c-godel-runtime --run --out ./out
cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-d-godel-obsmem-loop --run --trace --out ./out
cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-e-multi-agent-card-pipeline --run --trace --out ./out
cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-f-obsmem-retrieval --run --trace --out ./out
```

### Inspect-only review surfaces

Inspect these docs/spec artifacts rather than trying to run them:
- `docs/milestones/v0.8/CANONICAL_EVIDENCE_VIEW_V1.md`
- `docs/milestones/v0.8/MUTATION_FORMAT_V1.md`
- `docs/milestones/v0.8/EVALUATION_PLAN_V1.md`
- `docs/milestones/v0.8/EXPERIMENT_RECORD_V1.md`
- `docs/milestones/v0.8/OBSMEM_INDEXING_SURFACES_V1.md`
- `docs/milestones/v0.8/GODEL_EXPERIMENT_WORKFLOW_TEMPLATE_V1.md`
- `docs/tooling/README.md`

## Deterministic Ordering Rules

Review-surface entries are ordered by:
1. milestone criticality,
2. review surface ID (`D8-01`, `D8-02`, ...),
3. issue number tie-break.

## Required Review Surfaces

| Surface ID | Surface Type | Reviewer Action | Scope | Primary Evidence Surface | Canonical Validation Command |
|---|---|---|---|---|---|
| D8-01 | inspect_only | Read / inspect | Gödel schema spine | schema/example artifacts under `docs/milestones/v0.8/` | targeted `jq` / path checks |
| D8-02 | inspect_only_with_runnable_support | Read workflow docs, then optionally run a bounded demo | Gödel workflow integration | `GODEL_EXPERIMENT_WORKFLOW_TEMPLATE_V1.md` + template JSON + Demo D artifacts | `cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-d-godel-obsmem-loop --run --trace --out ./out` |
| D8-03 | inspect_only_with_runnable_support | Read indexing docs, then optionally run a bounded demo | ObsMem indexing integration | `OBSMEM_INDEXING_SURFACES_V1.md` + Demo F artifacts | `cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-f-obsmem-retrieval --run --trace --out ./out` |
| D8-04 | runnable_demo | Run and inspect output | Rust transpiler flagship | `RUST_TRANSPILER_DEMO.md` + `RUST_TRANSPILER_VERIFICATION_V0.8.md` + transpiler verification JSON | `cargo run --manifest-path tools/transpiler_demo/Cargo.toml --quiet` |
| D8-05 | inspect_only_with_runnable_support | Read authoring/reviewer contracts, then optionally run a bounded demo | Authoring/reviewer compatibility | tooling docs/contracts + Demo E artifacts | `cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-e-multi-agent-card-pipeline --run --trace --out ./out` |

## Supporting Runnable Demos

- `demo-c-godel-runtime`
  - Exercises the bounded milestone-surface validator and emits `godel_runtime_surface_status.json`.
- `demo-d-godel-obsmem-loop`
  - Exercises bounded stage loop, experiment record persistence, and ObsMem index persistence.
- `demo-e-multi-agent-card-pipeline`
  - Exercises deterministic card-pipeline artifact flow.
- `demo-f-obsmem-retrieval`
  - Exercises deterministic retrieval over persisted runtime index entries.

## Review Notes

- Not every important review surface is a runnable demo.
- v0.8 should be reviewed as a mix of:
  - bounded implemented runtime/demo surfaces, and
  - inspect-only schema/spec/planning surfaces.
- Reviewers should not infer that every planned surface is runtime-implemented.
