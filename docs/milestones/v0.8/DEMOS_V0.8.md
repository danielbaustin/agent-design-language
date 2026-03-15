# v0.8 Demo Matrix and Review Surfaces

This document defines the canonical review-entry matrix for v0.8 milestone review.

It covers both:
- runnable demo surfaces that a reviewer can execute, and
- inspect-only documentation/specification surfaces that a reviewer should read rather than run.

## Quick Reviewer Split

If you are asking "what do I run?" versus "what do I inspect?", use this split first.

### Runnable demo commands

Run these commands from repository root:

```bash
cargo run --manifest-path tools/transpiler_demo/Cargo.toml --quiet
cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-c-godel-runtime --run --out ./out
cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-d-godel-obsmem-loop --run --trace --out ./out
cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-e-multi-agent-card-pipeline --run --trace --out ./out
cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-f-obsmem-retrieval --run --trace --out ./out
```

For the bounded AEE recovery path, use:
- `demos/aee-recovery/README.md`
- It documents the current repo-root failure -> suggestion -> overlay -> recovery flow and the replayable artifacts for both runs.

Bounded Adaptive Execution Demo:
- `demos/godel_failure_hypothesis_experiment.md`

For the bounded Gödel CLI catch-up surfaces, use the dedicated runbook:
- `demos/godel_failure_hypothesis_experiment.md`
- It provides the current repo-root `adl godel run`, `adl godel inspect`, and `adl godel evaluate` sequence against the reorganized demo layout.

These runbooks live under `demos/`, which is now the canonical user-facing demo entrypoint for the active v0.8 review surfaces.

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

| Surface ID | Surface Type | Reviewer Action | Workstream | Scope | Primary Issue(s) | Primary Evidence Surface | Canonical Validation Command |
|---|---|---|---|---|---|---|---|
| D8-01 | inspect_only | Read / inspect | Gödel schema spine | ExperimentRecord + Evidence + Mutation + EvaluationPlan schema alignment | `#609`, `#610`, `#611`, `#612`, `#683` | canonical schema/example artifacts under `adl-spec/` | targeted `jq` / path checks |
| D8-02 | inspect_only_with_runnable_support | Read workflow docs, then optionally run supporting demo | Gödel workflow integration | Failure -> hypothesis -> mutation -> experiment -> evaluation -> record loop template alignment | `#613`, `#615`, `#616` | `GODEL_EXPERIMENT_WORKFLOW_TEMPLATE_V1.md` + `adl-spec/examples/v0.8/godel_experiment_workflow.template.v1.json` + supporting Demo D runtime artifacts | `cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-d-godel-obsmem-loop --run --trace --out ./out` |
| D8-03 | inspect_only_with_runnable_support | Read indexing docs, then optionally run supporting demo | ObsMem indexing integration | Run summary + ExperimentRecord-derived indexing surfaces | `#614` | `OBSMEM_INDEXING_SURFACES_V1.md` + supporting Demo F runtime artifacts | `cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-f-obsmem-retrieval --run --trace --out ./out` |
| D8-04 | runnable_demo | Run and inspect output | Runtime/transpiler flagship | Bounded Rust-first transpiler demo scaffold: deterministic mapping verification + stable evidence artifact | `#702`, `#703`, `#704`, `#759` | `RUST_TRANSPILER_DEMO.md` + `RUST_TRANSPILER_VERIFICATION_V0.8.md` + `demos/rust-transpiler/output/transpiler_verification.v0.8.json` | `cargo run --manifest-path tools/transpiler_demo/Cargo.toml --quiet` |
| D8-05 | inspect_only_with_runnable_support | Read authoring/reviewer contracts, then optionally run supporting demo | Authoring/reviewer compatibility | Prompt spec + reviewer checklist/output contracts and ordering | `#633`, `#650`, `#651`, `#649`, `#667`, `#677` | tooling docs/contracts + supporting Demo E runtime artifacts | `cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-e-multi-agent-card-pipeline --run --trace --out ./out` |

## Supporting Runnable Demos

- `demo-c-godel-runtime`
  - Exercises the bounded milestone-surface validator and emits `godel_runtime_surface_status.json`.
- `demo-d-godel-obsmem-loop`
  - Exercises bounded stage loop, experiment record persistence, and ObsMem index persistence.
- `demo-e-multi-agent-card-pipeline`
  - Exercises deterministic card-pipeline artifact flow.
- `demo-f-obsmem-retrieval`
  - Exercises deterministic retrieval over persisted runtime index entries.
- `demos/aee-recovery/README.md`
  - Exercises bounded retry-policy recovery using a deterministic fail-once provider mock, a retry-budget suggestion, an explicit overlay, and replayable run artifacts.
- `demos/godel_failure_hypothesis_experiment.md`
  - Exercises the bounded Gödel CLI review flow and the persisted `canonical_evidence_view.v1`, `mutation.v1`, `evaluation_plan.v1`, and `experiment_record.v1` runtime artifacts.
  - This is also the bounded adaptive execution demo pointer for reviewer follow-through on failure -> hypothesis -> experiment surfaces.

## Required Validation/Evidence Expectations

Each required review surface should provide:
1. A canonical doc/spec pointer in `docs/milestones/v0.8/`.
2. Deterministic artifact/evidence references where applicable.
3. Clear implemented-vs-illustrative boundary notes.
4. No secrets, tool arguments, raw prompts, or absolute host paths in persisted evidence.
5. An explicit reviewer action: `run`, `inspect`, or `inspect with runnable support`.

## Review Notes

- Not every important review surface is a runnable demo.
- v0.8 should be reviewed as a mix of:
  - bounded implemented runtime/demo surfaces, and
  - inspect-only schema/spec/planning surfaces.
- Reviewers should not infer that every planned surface is runtime-implemented.
