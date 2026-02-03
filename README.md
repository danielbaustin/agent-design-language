# Agent Design Language (ADL)

Agent Design Language (ADL) is a **declarative, contract-driven language** for designing and executing AI agent workflows with **deterministic prompting, explicit inputs, and auditable execution**.

This repository is the **monorepo** for ADL v0.1.

---

## Repository layout

- `swarm/` — **Reference runtime (Rust)**  
  Parses, validates, resolves, and executes ADL workflows.  
  Includes strict/loose schema validation, deterministic prompt assembly, tracing, and robust file input handling.

- `adl-spec/` — **Language specification**  
  Markdown specification, glossary, and examples describing the ADL language itself.

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

The reference runtime enforces a strong quality baseline:

- >90% **line coverage**
- Full schema validation test coverage
- `cargo fmt`, `cargo clippy -D warnings`, and `cargo test` required clean

Coverage is measured locally via `coverage.sh` (LCOV).

---

## Status

- **v0.1** — single‑workflow execution, sequential steps
- Concurrency is defined in the schema but intentionally gated
- Provider support: Ollama (real) + mock (tests)

Future versions will expand contracts, providers, and concurrency semantics.

---

## License

Apache‑2.0
