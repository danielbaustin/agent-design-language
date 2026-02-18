# Agent Design Language (ADL)

Agent Design Language (ADL) is a declarative, contract-driven way to define AI workflows as data, not ad-hoc glue code. Instead of wiring prompts, scripts, and shell calls together by convention, you describe agents, tasks, providers, and workflow steps in a schema-validated document.

ADL is built for teams that want repeatability. Documents are parsed and validated, then resolved into a deterministic plan before execution. That plan-first model makes behavior inspectable, testable, and easier to review than runtime-only orchestration.

The v0.3 runtime focuses on predictable execution semantics: deterministic sequential execution, deterministic fork/join behavior in declared order (single-threaded, not parallel), explicit failure policies (`on_error: fail|continue`), and deterministic retry via `retry.max_attempts`.

ADL also supports a remote HTTP provider MVP for controlled integration with external inference endpoints. Every run can emit stable artifacts under `.adl/runs/<run_id>/` (`run.json`, `steps.json`), which helps with reproducibility, debugging, and auditability.

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

[![swarm-ci (main)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml/badge.svg?branch=main&event=push)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml)
[![coverage](https://codecov.io/gh/danielbaustin/agent-design-language/graph/badge.svg?branch=main)](https://app.codecov.io/gh/danielbaustin/agent-design-language/tree/main)
[![swarm-coverage-gate (main)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml/badge.svg?branch=main&event=push)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml)

Badges above report the `main` branch workflow status, not the currently open PR branch.

## Current Status (v0.3)

Implemented in the `swarm/` runtime:
- Deterministic sequential execution
- Deterministic v0.3 fork/join execution (`workflow.kind: concurrent`) using single-threaded declared step order
- Step-level failure policy: `on_error: fail|continue`
- Deterministic retries: `retry.max_attempts` (no backoff)
- Remote HTTP provider (MVP)
- Run state artifacts under `.adl/runs/<run_id>/` (`run.json`, `steps.json`)

Explicitly deferred:
- True parallel execution (v0.4 target)
- Cancellation propagation and replay engine

## Repository Layout

- `swarm/`: Rust reference runtime and CLI
- `adl-spec/`: language-level specification docs
- `docs/`: contributor workflow and roadmap docs
- `.adl/`: cards, reports, and run/report artifacts

## v0.3 Example Commands

From repo root:

```bash
cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-concurrency-fork-join.adl.yaml --print-plan
cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-on-error-retry.adl.yaml --print-plan
cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-remote-http-provider.adl.yaml --print-plan
```

To execute (`--run`) local-provider examples, run from `swarm/` with a local Ollama available.

## v0.4 Demos

These demos are deterministic, non-interactive, and run without network by pinning the local mock provider binary.

Fork/Join swarm (3 branches + deterministic join barrier):

```bash
SWARM_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-4-demo-fork-join-swarm.adl.yaml --run --trace --out .adl/reports/demo-v0.4/fork-join-swarm
```

Bounded parallelism stress (8 branch steps with bounded executor):

```bash
SWARM_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-4-demo-bounded-parallelism.adl.yaml --run --trace --out .adl/reports/demo-v0.4/bounded-parallelism
```

Deterministic replay (run twice with same command, then compare `replay/join.txt` hash):

```bash
SWARM_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-4-demo-deterministic-replay.adl.yaml --run --trace --out .adl/reports/demo-v0.4/deterministic-replay
```

Run all three demos in sequence:

```bash
swarm/tools/demo_v0_4.sh
```

## Default Workflow

Default contributor workflow uses `adl_pr_cycle` (`start -> codex -> finish -> report`).
- Guide: `docs/default_workflow.md`
- Roadmap index: `docs/v0.3-roadmap.md`
- Tools: `swarm/tools/README.md`

## License

Apache-2.0
