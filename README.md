# Agent Design Language (ADL)

Agent Design Language (ADL) is a deterministic, contract-driven orchestration language for AI systems. It lets you define agents, tasks, providers, delegation metadata, and workflows as structured data — not brittle glue code. ADL elevates orchestration from “prompt wiring” to a reviewable, testable, and reproducible engineering discipline.

ADL is built for teams that care about determinism and auditability. Documents are schema-validated, compiled into a deterministic ExecutionPlan, and executed under explicit concurrency, failure, retry, and signing semantics. Every run emits stable artifacts under `.adl/runs/<run_id>/` to support replay, debugging, and post-mortem analysis.

[![adl-ci (main)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml/badge.svg?branch=main&event=push)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml)
[![coverage](https://codecov.io/gh/danielbaustin/agent-design-language/graph/badge.svg?branch=main)](https://app.codecov.io/gh/danielbaustin/agent-design-language/tree/main)
![Milestone](https://img.shields.io/badge/milestone-v0.6-green)


## Try It Now (Happy Path)

From repo root:

```bash
cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-fork-join-seq-run.adl.yaml --print-plan
```

This prints a deterministic v0.3 fork/join plan with clean output and no provider runtime setup.

If you want a second quick check:

```bash
cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-on-error-retry.adl.yaml --print-plan
```

Badge semantics:
- `adl-ci`: main branch CI workflow status
- `coverage`: Codecov line-coverage signal for `main` (informational; CI still passes if Codecov upload is unavailable)
- `milestone`: current documentation milestone marker

## Demos

Story-driven demo packs live in [`docs/milestones/v0.7/DEMOS_v0.7.md`](docs/milestones/v0.7/DEMOS_v0.7.md) under **Story-driven demo packs (user-facing)**:

- `S-01` Determinism You Can Trust
- `S-02` From Failure to Clarity
- `S-03` Portable Learning (Exportable Intelligence)
- `S-04` Enterprise Trust Boundary (Signed Remote Requests)
- `S-05` ADL is the Product Name (Compatibility Window)
- `S-06` The Agent That Learns (planned/deferred)

Canonical reference: [`docs/milestones/v0.7/DEMOS_v0.7.md`](docs/milestones/v0.7/DEMOS_v0.7.md)

## Status

Current release: **v0.6.0**

## v0.7 Naming Migration (Compatibility Window)

- Canonical CLI/runtime naming is `adl` and `adl-remote`.
- Legacy compatibility shim commands remain available in v0.7 with deprecation warnings.
- Canonical env vars use `ADL_*`; legacy compatibility env vars remain supported in v0.7 with deprecation warnings.

## Features by Release

### v0.6 (Current)

* ExecutionPlan-driven runtime execution
* Deterministic sequential + concurrent fork/join semantics
* Canonical concurrent ready-step ordering (lexicographic by `step_id`)
* Deterministic join barrier semantics
* Bounded parallelism enforcement in runtime
* Step-level failure policy (`on_error: fail|continue`)
* Deterministic retries (`retry.max_attempts`, no backoff)
* Deterministic replay demos + trace diff / graph export tooling
* Streaming trace events (observational)
* Human-readable trace timestamps + run/step progress banners
* Pattern compiler (`linear`, `fork_join`) with deterministic canonical IDs
* Provider profile registry (predefined profiles)
* Signing and verification CLI (`keygen`, `sign`, `verify`) with unsigned-run rejection on `--run`
* Remote execution MVP (`/v1/health`, `/v1/execute`) with local scheduler ownership
* HITL pause/resume (step-boundary-only) with deterministic, versioned, atomic pause state

### v0.5

* Full primitives support (agents, tasks, providers, workflows)
* Deterministic plan-only mode
* Signing canonicalization groundwork

### v0.4

* Deterministic, no-network demo harness (`swarm/tools/demo_v0_4.sh`)
* Bounded executor prototype demos
* Stable artifact emission

### v0.3

* Fork/join planning semantics
* Concurrency planning model
* Plan printing + deterministic ID normalization

## Repository Layout

- `swarm/`: Rust reference runtime and CLI
- `adl-spec/`: language-level specification docs
- `docs/`: contributor workflow and roadmap docs
- `docs/adr/`: architecture decision records (major technical decisions)
- `.adl/`: cards, reports, and run/report artifacts

## Historical v0.3 Plan-Only Commands

From repo root:

```bash
cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-concurrency-fork-join.adl.yaml --print-plan
cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-on-error-retry.adl.yaml --print-plan
cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-remote-http-provider.adl.yaml --print-plan
```

To execute (`--run`) local-provider examples, run from `swarm/` with a local Ollama available.

## Legacy v0.4 Demos

These demos are deterministic, non-interactive, and run without network by pinning the local mock provider binary.

Fork/Join demo (3 branches + deterministic join barrier):

```bash
ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-4-demo-fork-join-swarm.adl.yaml --run --trace --out .adl/reports/demo-v0.4/fork-join-swarm
```

Bounded parallelism stress (8 branch steps with bounded executor):

```bash
ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-4-demo-bounded-parallelism.adl.yaml --run --trace --out .adl/reports/demo-v0.4/bounded-parallelism
```

Current engine concurrency is intentionally fixed at `MAX_PARALLEL=4` in v0.4; this demo proves bounded execution at that shipped limit.

Deterministic replay (run twice with same command, then compare `replay/join.txt` hash):

```bash
ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-4-demo-deterministic-replay.adl.yaml --run --trace --out .adl/reports/demo-v0.4/deterministic-replay
```

Run all three demos in sequence:

```bash
swarm/tools/demo_v0_4.sh
```

## Why v0.6 Matters

v0.6 proves:
- Concurrent execution in the real runtime
- Deterministic replay behavior
- Bounded parallelism
- Stable artifacts under concurrency
- Signed workflow execution defaults for safer `--run` operation
- Pattern-driven workflow authoring with deterministic expansion
- Remote execution MVP wiring without giving up local deterministic scheduling

## Default Workflow

Default contributor workflow uses `adl_pr_cycle` (`start -> codex -> finish -> report`).
- Guide: `docs/default_workflow.md`
- Milestone docs: `docs/milestones/v0.6/`
- Tools: `swarm/tools/README.md`

## License

Apache-2.0

## Security

- Security policy: `SECURITY.md`
- Threat model (v0.7): `docs/security/THREAT_MODEL_v0.7.md`
