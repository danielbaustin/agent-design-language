# Agent Design Language (ADL)

Agent Design Language (ADL) is a declarative, contract-driven language for building deterministic AI workflow execution.

[![swarm-ci](https://img.shields.io/github/actions/workflow/status/danielbaustin/agent-design-language/ci.yaml?branch=main&label=swarm-ci)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml)
[![coverage](https://codecov.io/gh/danielbaustin/agent-design-language/graph/badge.svg?branch=main)](https://app.codecov.io/gh/danielbaustin/agent-design-language/tree/main)
[![swarm-coverage-gate](https://img.shields.io/github/actions/workflow/status/danielbaustin/agent-design-language/ci.yaml?branch=main&label=swarm-coverage-gate)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml)

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
cargo run --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-concurrency-fork-join.adl.yaml --print-plan
cargo run --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-on-error-retry.adl.yaml --print-plan
cargo run --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-remote-http-provider.adl.yaml --print-plan
```

To execute (`--run`) local-provider examples, run from `swarm/` with a local Ollama available.

## Default Workflow

Default contributor workflow uses `adl_pr_cycle` (`start -> codex -> finish -> report`).
- Guide: `docs/default_workflow.md`
- Roadmap index: `docs/v0.3-roadmap.md`
- Tools: `swarm/tools/README.md`

## License

Apache-2.0
