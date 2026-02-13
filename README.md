# Agent Design Language (ADL)

Agent Design Language (ADL) is a **declarative, contract-driven language** for designing and executing AI agent workflows with **deterministic prompting, explicit inputs, and auditable execution**.

This repository is the **monorepo** for ADL v0.1 and the in-progress v0.2 capability expansion.

---

## Repository layout

- `swarm/` — **Reference runtime (Rust)**  
  Parses, validates, resolves, and executes ADL workflows.  
  Includes strict/loose schema validation, deterministic prompt assembly, tracing, and robust file input handling.

- `adl-spec/` — **Language specification**  
  Markdown specification, glossary, and examples describing the ADL language itself.

## Default Development Workflow

New ADL development should use the Codex.app `adl_pr_cycle` workflow as the default path (`start -> codex -> finish -> report`).

---

## 5‑minute demo

> Requires Rust and a local Ollama installation (for real runs).  
> Tests use a mock provider and do **not** require Ollama.

```bash
# Build and run tests
cd swarm
cargo test

# Show prompt override precedence (step > task > agent)
cargo run -- examples/override-precedence.adl.yaml --print-prompts

# Execute with tracing enabled
cargo run -- examples/override-precedence.adl.yaml --run --trace

# Demonstrate clean failure on missing @file input
cargo run -- examples/failure-missing-file.adl.yaml --run --trace
```

For a v0.2 coordinator-style example (agents SDK–style), see:
`swarm/examples/v0-2-coordinator-agents-sdk.adl.yaml`.

---

## Core ideas

- **Determinism by design**  
  Prompt resolution order is fixed and testable:
  `step → task → agent`.

- **Explicit inputs**  
  Inputs may reference files via `@file:path`, with strong validation:
  missing files, directories, invalid UTF‑8, and oversize inputs fail fast.

- **Auditable execution**  
  Optional tracing emits a stable, structured event stream:
  `StepStarted → PromptAssembled → StepFinished`.

- **Schema‑first**  
  ADL documents are validated against a published JSON Schema before execution.

---

## Coverage & quality bar

The reference runtime enforces a pragmatic quality baseline:

- `cargo fmt`, `cargo clippy -D warnings`, and `cargo test` required clean
- Coverage guardrail enforced in CI via `cargo llvm-cov`

Run locally from `swarm/`:

```bash
cargo llvm-cov --fail-under-lines 10
```

---

## Status

- **v0.1** — single‑workflow execution, sequential steps
- Concurrency is defined in the schema but intentionally gated
- Provider support: Ollama (real) + mock (tests)

Future versions will expand contracts, providers, and concurrency semantics.

---

## License

Apache‑2.0
