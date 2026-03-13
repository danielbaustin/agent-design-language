# v0.8 Demo Matrix and Integration Demos

This document defines the canonical review-entry matrix for v0.8 milestone review.

It covers both:
- runnable demo surfaces that a reviewer can execute, and
- inspect-only documentation/specification surfaces that a reviewer should read rather than run.

## Quick Reviewer Split

If you are asking "what do I run?" versus "what do I inspect?", use this split first.

### Runnable demo commands

Run these commands from repository root:

```bash
cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-d-godel-obsmem-loop --run --trace --out ./out
cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-e-multi-agent-card-pipeline --run --trace --out ./out
cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-f-obsmem-retrieval --run --trace --out ./out
cargo run --manifest-path tools/transpiler_demo/Cargo.toml --quiet
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

1. required milestone criticality,
2. review surface ID (`D8-01`, `D8-02`, ...),
3. issue number tie-break.

## Required Review Surfaces (Pre-Review / Pre-Release)

These surfaces are required before third-party review (`#707`) and release convergence.

| Surface ID | Surface Type | Reviewer Action | Workstream | Scope | Primary Issue(s) | Primary Evidence Surface | Canonical Validation Command |
|---|---|---|---|---|---|---|---|
| D8-01 | inspect_only | Read / inspect | Gödel schema spine | ExperimentRecord + Evidence + Mutation + EvaluationPlan schema alignment | `#609`, `#610`, `#611`, `#612`, `#683` | canonical schema/example artifacts under `docs/milestones/v0.8/` | `jq . docs/milestones/v0.8/*.json` (targeted schema/example checks) |
| D8-02 | inspect_only_with_runnable_support | Read workflow template/docs, then optionally run supporting demo | Gödel workflow integration | Failure -> hypothesis -> mutation -> experiment -> evaluation -> record loop template alignment | `#613`, `#615`, `#616` | `GODEL_EXPERIMENT_WORKFLOW_TEMPLATE_V1.md` + `godel_experiment_workflow.template.v1.json` + supporting Demo D runtime artifacts | `cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-d-godel-obsmem-loop --run --trace --out ./out` |
| D8-03 | inspect_only_with_runnable_support | Read indexing surfaces, then optionally run supporting demo | ObsMem indexing integration | Run summary + ExperimentRecord-derived indexing surfaces | `#614` | indexing surface definitions and retrieval linkage notes + supporting Demo F runtime artifacts | `cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-f-obsmem-retrieval --run --trace --out ./out` |
| D8-04 | runnable_demo | Run and inspect output | Runtime/transpiler flagship | Bounded Rust-first transpiler mapping + deterministic verification evidence | `#702`, `#703`, `#704`, `#759` | `RUST_TRANSPILER_DEMO.md` + `RUST_TRANSPILER_VERIFICATION_V0.8.md` + `demos/rust_output/transpiler_verification.v0.8.json` | `cargo run --manifest-path tools/transpiler_demo/Cargo.toml --quiet` |
| D8-05 | inspect_only_with_runnable_support | Read authoring/reviewer contracts, then optionally run supporting demo | Authoring/reviewer compatibility | Prompt spec + reviewer checklist/output contracts and ordering | `#633`, `#650`, `#651`, `#649`, `#667`, `#677` | tooling docs/contracts + template references + supporting Demo E runtime artifacts | `cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-e-multi-agent-card-pipeline --run --trace --out ./out` |

## Supporting Review Surfaces (Helpful, Not Release-Blocking Alone)

| Surface ID | Surface Type | Reviewer Action | Workstream | Scope | Primary Issue(s) | Evidence Surface |
|---|---|---|---|---|---|---|
| D8-S1 | inspect_only | Read / inspect | AEE boundary clarity | Bounded v0.8 adaptive execution scope statement | `#669` | `BOUNDED_AEE_V1_SCOPE_V0.8.md` |
| D8-S2 | inspect_only | Read / inspect | Execution sequencing | Milestone dependency/order check surface | `#664`, `#665`, `#666` | `EXECUTION_ORDER_V0.8.md` + related boundary docs |

## Runnable Demo Mapping

The following runtime demos are implemented today and can be run directly.

- Supporting Demo D for D8-02: `demo-d-godel-obsmem-loop`
  - Exercises bounded stage loop, experiment record persistence, and ObsMem index persistence.
  - Emits:
    - `out/demo-d-godel-obsmem-loop/godel_obsmem_demo_summary.json`
    - `out/demo-d-godel-obsmem-loop/runs/demo-d-run-001/godel/experiment_record.runtime.v1.json`
    - `out/demo-d-godel-obsmem-loop/runs/demo-d-run-001/godel/obsmem_index_entry.runtime.v1.json`
- Supporting Demo E for D8-05: `demo-e-multi-agent-card-pipeline`
  - Exercises deterministic multi-agent card pipeline artifact flow.
  - Emits:
    - `out/demo-e-multi-agent-card-pipeline/pipeline/input_card.md`
    - `out/demo-e-multi-agent-card-pipeline/pipeline/pipeline_manifest.json`
- Supporting Demo F for D8-03: `demo-f-obsmem-retrieval`
  - Exercises deterministic retrieval over persisted runtime index entries.
  - Emits:
    - `out/demo-f-obsmem-retrieval/obsmem_retrieval_result.json`
    - `out/demo-f-obsmem-retrieval/runs/demo-f-run-a/godel/obsmem_index_entry.runtime.v1.json`
    - `out/demo-f-obsmem-retrieval/runs/demo-f-run-b/godel/obsmem_index_entry.runtime.v1.json`
- Flagship runnable demo D8-04: transpiler scaffold verification
  - Emits deterministic verification status through `tools/transpiler_demo/` and references:
    - `examples/workflows/rust_transpiler_demo.yaml`
    - `demos/rust_output/workflow_runtime.rs`
    - `demos/rust_output/transpiler_verification.v0.8.json`

See `docs/demos/v0.8-bounded-critical-demos.md` for a compact runbook focused only on runnable demo commands.

## Required Validation/Evidence Expectations

Each required review surface should provide:

1. A canonical doc/spec pointer in `docs/milestones/v0.8/`.
2. Deterministic artifact/evidence references (schema/example/template/contract).
3. Clear implemented-vs-illustrative boundary notes where applicable.
4. No secrets, tool arguments, raw prompts, or absolute host paths in persisted evidence.
5. An explicit reviewer action: `run`, `inspect`, or `inspect with runnable support`.

## Review Gate Usage

- Use this matrix as the integration-demo checklist for `#706` docs convergence and `#707` third-party review pass.
- A required review surface is considered complete when its evidence surface exists, is cross-linked, and matches milestone scope boundaries.
- Only rows explicitly marked `runnable_demo` or `inspect_only_with_runnable_support` should be treated as execution entry points.

## Out of Scope

- Adding new milestone features solely to satisfy demos.
- Reclassifying deferred v0.9+ autonomy work into v0.8.
- Replacing issue-level acceptance criteria with this matrix.
