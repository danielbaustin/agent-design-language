# Agent Design Language (ADL)

**Version:** v0.3  
**Status:** Active v0.3 runtime with deterministic execution, v0.3 fork/join support, and hardened tooling gates

ADL is a schema-validated language and runtime for defining and executing
agent workflows with deterministic resolution and clear failure modes.

[![swarm-ci (main)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml/badge.svg?branch=main&event=push)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml)
[![coverage](https://codecov.io/gh/danielbaustin/agent-design-language/graph/badge.svg?branch=main)](https://app.codecov.io/gh/danielbaustin/agent-design-language/tree/main)
[![swarm-coverage-gate (main)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml/badge.svg?branch=main&event=push)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml)
![License](https://img.shields.io/badge/license-Apache--2.0-blue)
![MSRV](https://img.shields.io/badge/MSRV-1.74%2B-blue)

Status badges above are for `main` branch workflow health, not per-PR checks.
## swarm

`swarm` is a small, conservative reference runtime for **Agent Design Language (ADL)**.

For historical context, see `RELEASE_NOTES_v0.2.md`.
For the official v0.3 milestone summary, see `RELEASE_NOTES_v0.3.md`.
This README reflects the current v0.3 runtime.

It is intentionally *compiler-like* in how it processes ADL documents:

1. **Parse** an ADL YAML document into a typed in-memory model.
2. **Validate** the document against a JSON Schema with crisp, path-specific errors.
3. **Resolve** references deterministically (run → workflow → steps → task → agent → provider).
4. **Materialize** deterministic artifacts (execution plan, assembled prompts).
5. **Execute** deterministic workflows (sequential and v0.3 deterministic fork/join execution), with optional tracing.

Provider execution, tracing, contracts, and repair policies are being added incrementally.

---

## v0.3 Shipped Capabilities

- Deterministic workflow execution with stable plan/trace semantics
- Deterministic v0.3 fork/join execution (`workflow.kind: concurrent`), executed single-threaded in declared step order
- Step-level failure controls (`on_error: fail|continue`, `retry.max_attempts`)
- Remote HTTP provider MVP with explicit failure behavior
- Persistent run state artifacts under `.adl/runs/<run_id>/` for auditability (`run.json`, `steps.json`)
- CI-aligned quality gate (`fmt`, `clippy -D warnings`, `test`)

---

## Fork/Join Mental Model (v0.3)

- **Fork**: declare branch steps under `workflow.kind: concurrent`.
- **Execution**: steps run sequentially in deterministic declared order (no true runtime parallelism yet).
- **Join**: consume branch outputs via `@state:<save_as_key>` and run only when required inputs are available.

---

## Current Status (v0.3)

**Implemented**

- Load and validate ADL YAML (schema + semantic validation)
- Deterministic resolution of run / workflow / steps / agents / tasks / providers
- Deterministic prompt assembly with precedence:
  - `step.prompt` → `task.prompt` → `agent.prompt`
- File-backed inputs with safety checks (size, encoding, paths)
- Sequential workflow execution
- Step-level error policy: `on_error: fail|continue`
- Deterministic retries: `retry.max_attempts` (no backoff)
- v0.3 deterministic fork/join execution (`workflow.kind: concurrent`), single-threaded in declared step order
- Join input wiring via `@state:<save_as_key>`
- Local Ollama provider (real binary or test mock)
- Remote HTTP provider (blocking JSON request/response)
- Deterministic tracing (`--trace`)
- CLI smoke tests and schema tests

**Explicitly deferred**

- Parallel workflow execution (true concurrency; v0.4 target)
- Multi-run documents
- Provider retries / contracts / repair policies

---

## 5‑Minute Demo

From the `swarm` directory:

```bash
# Happy path: one obvious first command
cargo run -q -- examples/v0-3-concurrency-fork-join.adl.yaml --print-plan

# Optional: verify on_error/retry semantics
cargo run -q -- examples/v0-3-on-error-retry.adl.yaml --print-plan

# Optional: verify remote provider wiring
cargo run -q -- examples/v0-3-remote-http-provider.adl.yaml --print-plan
```

Expected output includes deterministic step ordering and resolved provider bindings.
Using `-q` keeps demo output focused on the ADL plan rather than Cargo build noise.

For real `--run` execution, configure provider runtime dependencies (for example local Ollama and any required auth env vars).

For additional runnable examples, see `examples/README.md`.

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

## Run State Artifacts

When running with `--run`, `swarm` writes deterministic run state files under:

```bash
.adl/runs/<run_id>/
```

Files:
- `run.json`:
  - `run_id`
  - `workflow_id`
  - `version`
  - `status` (`success` or `failure`)
  - `start_time_ms`, `end_time_ms`, `duration_ms`
- `steps.json` (stable step order):
  - `step_id`
  - `agent_id`
  - `provider_id`
  - `status` (`success`, `failure`, `not_run`)
  - `output_artifact_path` (when applicable)

This is additive and does not replace existing stdout summaries.

---

## Remote Provider (HTTP MVP)

`swarm` supports a minimal remote provider via `type: "http"` for deterministic,
blocking prompt completion over HTTP.

Expected request/response contract:
- Request: `POST <endpoint>` with JSON body `{"prompt":"..."}`
- Response: JSON object containing string field `output`

Example (see `examples/v0-3-remote-http-provider.adl.yaml`):

```yaml
providers:
  remote_http:
    type: "http"
    config:
      endpoint: "http://127.0.0.1:8787/complete"
      timeout_secs: 10
      auth:
        type: bearer
        env: SWARM_REMOTE_BEARER_TOKEN
```

Run it with:

```bash
cargo run -- examples/v0-3-remote-http-provider.adl.yaml --print-plan
cargo run -- examples/v0-3-remote-http-provider.adl.yaml --run
```

Failure behavior is explicit:
- Missing endpoint -> config error
- Missing auth env var -> config error naming the env var
- Non-200 response -> runtime error with status + body snippet
- Timeout -> runtime error with timeout guidance

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

Note: this JSON file is **DRAFT / not authoritative** and may lag the runtime.
The runtime’s authoritative schema is generated from the Rust structs in `src/adl.rs`.

Schema tests live in:

```
tests/schema_tests.rs
```

Example validation documents live under:

```
examples/
```

Legacy examples (e.g. `adl-0.1.yaml`) remain for regression testing, but the runtime behavior described here reflects v0.3.

The schema/runtime behavior described here is aligned with current **v0.3** support.

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

As of v0.3:

- **Overall line coverage:** enforced by CI gate (see coverage badge above)
- **All critical paths covered:**
  - Schema validation (strict + loose modes)
  - ADL parsing and semantic validation
  - Deterministic resolution
  - Prompt assembly and hashing
  - CLI behavior and error handling
  - Provider execution (mocked and real)
- Lower coverage areas (e.g. some execution branches) are intentional and documented, not accidental gaps.

Coverage percentage may fluctuate as new features are added; the CI gate ensures regressions are caught.

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

- **Line coverage > function coverage** for v0.3  
  (many small helper functions are intentionally exercised indirectly)
- No “coverage theater”:
  - No dummy tests
  - No blanket `#[allow(dead_code)]` without justification
- Coverage is used as a **design signal**, not a vanity metric

Contributors are expected to keep overall line coverage **≥ 85%**, and ideally improve it with each change.

Badge note:
- `coverage` is the live Codecov percentage for the `swarm` upload.
- `swarm-coverage-gate` reflects whether the CI workflow coverage gate passes. It is a status badge, not a live percentage badge.

---

## License

Apache-2.0
