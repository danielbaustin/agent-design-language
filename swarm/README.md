# Agent Design Language (ADL)

**Version:** v0.1  
**Status:** Ready for early users — deterministic, validated, sequential execution

ADL is a schema-validated language and runtime for defining and executing
agent workflows with deterministic resolution and clear failure modes.

![Build](https://img.shields.io/badge/build-passing-brightgreen)
![Coverage](https://img.shields.io/badge/coverage-92%25-brightgreen)
![License](https://img.shields.io/badge/license-Apache--2.0-blue)
![MSRV](https://img.shields.io/badge/MSRV-1.74%2B-blue)
# swarm

`swarm` is a small, conservative reference runtime for **Agent Design Language (ADL)**.

It is intentionally *compiler-like* in how it processes ADL documents:

1. **Parse** an ADL YAML document into a typed in-memory model.
2. **Validate** the document against a JSON Schema with crisp, path-specific errors.
3. **Resolve** references deterministically (run → workflow → steps → task → agent → provider).
4. **Materialize** deterministic artifacts (execution plan, assembled prompts).
5. **Execute** sequential workflows (v0.1), with optional tracing.

Provider execution, tracing, contracts, and repair policies are being added incrementally.

---

## Status (v0.1)

**Implemented**

- Load and validate ADL YAML (schema + semantic validation)
- Deterministic resolution of run / workflow / steps / agents / tasks / providers
- Deterministic prompt assembly with precedence:
  - `step.prompt` → `task.prompt` → `agent.prompt`
- File-backed inputs with safety checks (size, encoding, paths)
- Sequential workflow execution
- Local Ollama provider (real binary or test mock)
- Deterministic tracing (`--trace`)
- CLI smoke tests and schema tests

**Explicitly deferred**

- Concurrent workflows (schema allows; runtime errors clearly)
- Multi-run documents
- Provider retries / contracts / repair policies

---

## 5‑Minute Demo

From the `swarm` directory:

```bash
cargo build

# Run the example and print the resolved execution plan
cargo run -- examples/adl-0.1.yaml --print-plan

# Run the example and execute it locally
cargo run -- examples/adl-0.1.yaml --run

# Run with deterministic trace output
cargo run -- examples/adl-0.1.yaml --run --trace
```

Expected output includes:

- A resolved step-by-step plan
- Assembled prompts (deterministic hashes)
- Optional trace events:
  - `StepStarted`
  - `PromptAssembled`
  - `StepFinished`

---

## CLI

```bash
swarm <path-to-adl.yaml> [OPTIONS]
```

**Options**

- `--run` — execute the workflow
- `--print-plan` — print the resolved execution plan only
- `--print-prompts` — print assembled prompts without execution
- `--trace` — emit deterministic trace events to stdout
- `--help` — usage and flag summary

Exit codes are consistent:
- `2` — invalid CLI usage
- non-zero — schema, validation, or runtime error

---

## Schema Validation

- ADL documents are validated **before** parsing.
- Unknown top-level fields are rejected.
- Required fields (e.g. `run`) are enforced.
- Errors include the failing path and reason.

The committed schema artifact lives at:

```
schemas/adl.schema.json
```

Schema tests live in:

```
tests/schema_tests.rs
```

The example document used for validation lives in:

```
examples/adl-0.1.yaml
```

The schema is considered **stable for v0.1**.

---

## Project Layout

```
src/
  main.rs        # CLI + wiring
  adl.rs         # ADL data model + loader
  schema.rs      # JSON Schema validation
  resolve.rs     # Deterministic resolution + plan materialization
  prompt.rs      # Prompt assembly + hashing
  provider.rs    # Provider abstraction (Ollama)
  execute.rs     # Sequential execution engine
  trace.rs       # Deterministic trace events
tests/
examples/
```

---


## Development

Run the full quality gate locally:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

All of the above must pass for changes to be accepted.

---

## Code Coverage

`swarm` enforces a **high bar for test coverage**, especially for core compiler-like behavior (parsing, validation, resolution, and execution).

As of v0.1:

- **Overall line coverage:** ~**92%**
- **All critical paths covered:**
  - Schema validation (strict + loose modes)
  - ADL parsing and semantic validation
  - Deterministic resolution
  - Prompt assembly and hashing
  - CLI behavior and error handling
  - Provider execution (mocked and real)
- Lower coverage areas (e.g. some execution branches) are intentional and documented, not accidental gaps.

### Running coverage locally

We use a simple, deterministic coverage script (checked into the repo) rather than relying on CI-specific tooling:

```bash
./coverage.sh
```

This generates:

- `lcov.info`
- An HTML report showing per-file and per-directory coverage

Coverage artifacts (the `coverage/` directory and `lcov.info`) are intentionally not checked into git.

The report makes it easy to identify:
- Untested branches
- Intentionally deferred logic
- Areas that would benefit from additional tests

### Coverage philosophy

- **Line coverage > function coverage** for v0.1  
  (many small helper functions are intentionally exercised indirectly)
- No “coverage theater”:
  - No dummy tests
  - No blanket `#[allow(dead_code)]` without justification
- Coverage is used as a **design signal**, not a vanity metric

Contributors are expected to keep overall line coverage **≥ 85%**, and ideally improve it with each change.

---

## License

Apache-2.0