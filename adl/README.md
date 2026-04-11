# ADL Runtime (`adl/`)

The ADL runtime is the reference Rust runtime and CLI for **Agent Design Language (ADL)**. It takes schema-validated ADL documents, resolves them into a deterministic execution plan, and executes them with explicit semantics for concurrency, retries, failures, signing, tracing, and bounded remote execution.

The runtime is built for readers and builders who want AI workflow execution to be predictable, inspectable, and reviewable. It emphasizes clear execution behavior, stable artifacts, and reproducible runs over hidden orchestration logic.

[![adl-ci (main)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml/badge.svg?branch=main&event=push)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml)
[![coverage](https://codecov.io/gh/danielbaustin/agent-design-language/graph/badge.svg?branch=main)](https://app.codecov.io/gh/danielbaustin/agent-design-language/tree/main)
![License](https://img.shields.io/badge/license-Apache--2.0-blue)
![MSRV](https://img.shields.io/badge/MSRV-1.74%2B-blue)

## Why the Runtime Matters

The runtime is where ADL’s design promises become operational reality.

It provides:
- deterministic plan materialization and execution
- bounded concurrency and explicit retry/failure policy
- stable run artifacts and replay-friendly state
- signing and verification surfaces for safer execution
- CLI review surfaces for plan inspection and runtime debugging
- a foundation for bounded reflective and scientific execution loops

## Current Status

- Current active milestone in the main repo: **v0.87.1**
- Recent stable runtime milestone: **v0.8**
- Most recently completed runtime milestone in the main repo: **v0.87**
- Previous closure milestone in the main repo: **v0.86**
- Current crate version on `main`: **0.86.0**

This README describes the runtime as it exists on the current `main` branch and points to the relevant milestone and demo surfaces in the parent repository.

## Recent Runtime Milestones

### v0.87.1 — Runtime Completion And Reviewer-Facing Proof Package

v0.87.1 is the active runtime milestone. The runtime-completion implementation and bounded demo program are landed; the remaining work is the quality/docs/review/release tail that makes the package externally reviewable.

Highlights:
- runtime environment, lifecycle, execution-boundary, resilience, and review surfaces promoted into one canonical milestone package
- bounded runtime, provider, quality-gate, and release-review demo surfaces for reviewer navigation
- explicit run manifest and trace-archive surfaces for provenance and later review/export
- live-provider companion proof kept explicit as credential-gated reviewer evidence rather than default CI proof

### v0.87 — Substrate Convergence and Reviewer-Facing Runtime Truth

v0.87 focuses on making the runtime-adjacent substrate legible and deterministic across trace, provider portability, shared memory, skills, and control-plane proof surfaces.

Highlights:
- trace, provider, shared-memory, skills, and control-plane work aligned under one canonical milestone spine
- promoted milestone feature docs and review surfaces reconciled with the actual runtime-adjacent issue sequence
- bounded demo and reviewer proof surfaces for trace, provider portability, shared ObsMem, skills, and control-plane behavior
- active Sprint 3 closeout work for docs, review, quality gate, and release packaging
- explicit handoff into the `v0.87.1` runtime-completion milestone now being carried through review and release-tail closeout

### v0.86 — Bounded Cognitive System and Runtime Proof Surfaces

v0.86 established the first working bounded cognitive system on `main`.

Highlights:
- one canonical bounded cognitive path from signals through Freedom Gate
- canonical runtime artifacts for the bounded cognitive path and related proof surfaces
- local demo and review surfaces for the integrated milestone proof set
- Sprint 7 quality-gate work with passing local `fmt`, `clippy`, `test`, coverage, and demo-validation proof
- runtime docs and reviewer entry points aligned toward milestone truth

### v0.85 — Authoring Alignment and Runtime Proof Surfaces

v0.85 focused on making the surrounding authoring model, demos, and proof surfaces line up cleanly with the runtime and tooling.

Highlights:
- five-command authoring lifecycle clarified and aligned with runtime-adjacent tooling
- bounded end-to-end demo and regression proof surfaces for authoring workflows
- improved queue/worktree hygiene and execution truth surfaces
- Rust maintainability pass across large modules and oversized test surfaces
- stronger documentation and review surfaces around what is actually shipped and runnable

### v0.8 — Bounded Gödel Runtime and Artifact-Centered Review

v0.8 extended the runtime into bounded reflective execution with explicit artifact surfaces.

Highlights:
- bounded Gödel-style scientific loop integrated into runtime/demo surfaces
- canonical artifact emission for mutation, evaluation, experiment, and evidence records
- CLI surfaces for running and inspecting bounded reasoning workflows
- ObsMem-backed indexing and retrieval-assisted review flows
- runnable demos and review packets for bounded adaptive execution

### v0.7 — Deterministic Runtime Foundation

v0.7 established the deterministic runtime core that ADL builds on.

Highlights:
- ExecutionPlan-driven runtime
- deterministic fork/join and concurrency semantics
- bounded parallelism and explicit retry/failure policy
- replay-oriented trace and graph export tooling
- signing and verification surfaces for execution integrity

## Quick Start

From the `adl/` directory:

```bash
cargo run -q --bin adl -- examples/v0-5-primitives-minimal.adl.yaml --print-plan
```

This prints a deterministic plan for a small workflow fixture without requiring provider runtime setup.

A second quick check:

```bash
cargo run -q --bin adl -- examples/v0-5-pattern-fork-join.adl.yaml --print-plan
```

For a retry-oriented fixture:

```bash
cargo run -q --bin adl -- examples/v0-3-on-error-retry.adl.yaml --print-plan
```

## Common CLI Surfaces

```bash
cargo run -q --bin adl -- <path-to-adl.yaml> [OPTIONS]
```

Common options:
- `--run` — execute the workflow
- `--print-plan` — print the resolved execution plan only
- `--print-prompts` — print assembled prompts without execution
- `--trace` — emit deterministic trace events to stdout
- `--help` — usage and flag summary

Exit behavior is explicit:
- `2` — invalid CLI usage
- non-zero — schema, validation, or runtime error

## Signing Quickstart

For workflows that require signing:

```bash
# 1) generate local dev keys
cargo run -q --bin adl -- keygen --out-dir ./.keys

# 2) sign a workflow
cargo run -q --bin adl -- sign examples/v0-5-pattern-linear.adl.yaml \
  --key ./.keys/ed25519-private.b64 \
  --out /tmp/signed.adl.yaml

# 3) verify signature
cargo run -q --bin adl -- verify /tmp/signed.adl.yaml --key ./.keys/ed25519-public.b64

# 4) run signed workflow
cargo run -q --bin adl -- /tmp/signed.adl.yaml --run
```

Dev-only bypass:

```bash
cargo run -q --bin adl -- examples/v0-5-pattern-linear.adl.yaml --run --allow-unsigned
```

## Run Artifacts

When running with `--run`, `adl` writes deterministic run state under:

```bash
.adl/runs/<run_id>/
```

Typical run-state files include:
- `run.json`
- `steps.json`

These surfaces are intended to make runs inspectable, replay-friendly, and easier to debug.

## Remote Execution Surfaces

The runtime supports a minimal remote provider via `type: "http"` and also supports a minimal remote execution protocol for step placement.

Remote execution boundary:
- scheduler ownership remains local in `adl`
- the remote server executes one fully resolved step at a time
- the remote endpoint is a bounded execution surface, not a remote planner/compiler

Key endpoints in the MVP protocol:
- `GET /v1/health`
- `POST /v1/execute`

This transport boundary is intended for trusted infrastructure and bounded deployment scenarios, not as a hardened public service.

## Documentation Map

For readers who want the broader context around the runtime:
- root repo README: `../README.md`
- canonical demo index: `../demos/README.md`
- v0.87 milestone docs: `../docs/milestones/v0.87/`
- v0.86 milestone docs: `../docs/milestones/v0.86/`
- v0.85 milestone docs: `../docs/milestones/v0.85/`
- v0.8 milestone docs: `../docs/milestones/v0.8/`
- v0.7 milestone docs: `../docs/milestones/v0.7/`
- architecture decisions: `../docs/adr/`
- crate-local fixture inventory: `examples/README.md`

Important demo/readiness references:
- `../docs/milestones/v0.87/DEMO_MATRIX_v0.87.md`
- `../docs/milestones/v0.86/DEMO_MATRIX_v0.86.md`
- `../docs/milestones/v0.8/DEMOS_V0.8.md`
- `../docs/milestones/v0.85/DEMO_MATRIX_v0.85.md`
- `../docs/tooling/editor/README.md`

## Project Layout

```text
src/
  main.rs
  adl.rs
  schema.rs
  resolve.rs
  prompt.rs
  provider.rs
  execute.rs
  trace.rs
tests/
examples/
```

Broadly:
- `src/` contains the reference runtime and CLI
- `tests/` contains runtime, schema, and CLI validation surfaces
- `examples/` contains runnable fixtures used by demos, tests, and inspection workflows

## Development

Run the local quality gate from `adl/`:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Coverage is commonly checked with:

```bash
cargo llvm-cov --workspace --all-features --summary-only
```

The runtime uses coverage as a design signal for core compiler-like behavior: parsing, validation, resolution, execution, and CLI behavior should remain well exercised.

## License

Apache-2.0
