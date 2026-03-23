# ADL Runtime (`adl/`)

ADL runtime is the reference Rust runtime for **Agent Design Language (ADL)**. It compiles schema-validated ADL documents into a deterministic ExecutionPlan and executes them with explicit concurrency, failure, retry, signing, and (minimal) remote execution semantics.

ADL runtime prioritizes determinism and inspectability. Every run emits stable artifacts under `.adl/runs/<run_id>/` to support replay, debugging, and post-mortem analysis.

[![adl-ci (main)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml/badge.svg?branch=main&event=push)](https://github.com/danielbaustin/agent-design-language/actions/workflows/ci.yaml)
[![coverage](https://codecov.io/gh/danielbaustin/agent-design-language/graph/badge.svg?branch=main)](https://app.codecov.io/gh/danielbaustin/agent-design-language/tree/main)
![License](https://img.shields.io/badge/license-Apache--2.0-blue)
![MSRV](https://img.shields.io/badge/MSRV-1.74%2B-blue)

Badge note:
- Status badges above reflect `main` branch workflow health.
- Coverage is generated via `cargo llvm-cov` in CI and uploaded to Codecov.
- CI enforces a coverage gate; the Codecov upload is informational.

## Status

Latest tagged runtime release: **v0.7.0**

Current main-branch crate version: **0.8.0**

This README reflects the current `main`-branch runtime surfaces for the active **v0.8** milestone. Treat it as unreleased development documentation layered on top of the latest tagged `v0.7.0` release.

## v0.7 Naming Migration (Compatibility Window)

- Canonical binaries are `adl` and `adl-remote`.
- Legacy compatibility shim binaries were introduced during v0.7 and remain available on `main` during the compatibility window with deprecation warnings.
- Canonical env vars use `ADL_*`; legacy compatibility env vars remain supported during the compatibility window with deprecation warnings.

## Features by Release

### v0.8 (Current Main Branch, Unreleased)

The current `main` branch carries the `0.8.0` crate version and includes bounded v0.8 runtime/demo surfaces on top of the shipped v0.7 base.

### v0.7.0 (Latest Tagged Release)

* ExecutionPlan-driven runtime execution
* Deterministic sequential + concurrent fork/join semantics
* Canonical concurrent ready-step ordering (lexicographic by `step_id`)
* Deterministic join barrier semantics
* Bounded parallelism enforcement via `run.defaults.max_concurrency` (default: 4; must be `>= 1`)
* Step-level failure controls (`on_error: fail|continue`, deterministic `retry.max_attempts`, no backoff)
* Streaming trace events (observational) with human-readable timestamps + progress banners
* Pattern compiler (`linear`, `fork_join`) with deterministic canonical IDs
* Provider profile registry (predefined profiles) with placeholder endpoint guardrails
* Signing and verification CLI (`keygen`, `sign`, `verify`) with unsigned-run rejection by default for `--run`
* Remote execution MVP (`/v1/health`, `/v1/execute`) where scheduler ownership remains local
* HITL pause/resume (step-boundary-only) with deterministic, versioned, atomic pause state
* CI-aligned quality gate (`fmt`, `clippy -D warnings`, `test`, coverage gate)

### v0.5

* Full primitives support (agents, tasks, providers, workflows)
* Deterministic plan-only mode
* Signing canonicalization groundwork

### v0.4

* Deterministic, no-network demo harness (`adl/tools/demo_v0_4.sh`)
* Stable artifact emission

### v0.3

* Fork/join planning semantics
* Concurrency planning model
* Plan printing + deterministic ID normalization

## Documentation Map

- Root repo README: `../README.md`
- Canonical demo index: `../demos/README.md`
- v0.8 milestone docs: `../docs/milestones/v0.8/`
- v0.7 milestone docs: `../docs/milestones/v0.7/`
- ADRs: `../docs/adr/`
- Runnable demos:
  - `../docs/milestones/v0.7/DEMOS_v0.7.md` (latest tagged release)
  - `../docs/milestones/v0.8/DEMOS_V0.8.md` (active milestone review packet)
- Crate-local fixture inventory: `examples/README.md`

## How Swarm Processes ADL (Compiler-like Pipeline)

Swarm processes ADL documents in a conservative, compiler-like pipeline:

1. **Parse** an ADL YAML document into a typed in-memory model.
2. **Validate** the document against a JSON Schema with crisp, path-specific errors.
3. **Resolve** references deterministically (run → workflow → steps → task → agent → provider).
4. **Materialize** deterministic artifacts (execution plan, assembled prompts).
5. **Execute** deterministic workflows (sequential and bounded concurrent execution), with optional tracing.

Provider execution, tracing, contracts, and repair policies are being added incrementally.

---

## Fork/Join Mental Model

- **Fork**: declare branch steps under `workflow.kind: concurrent`.
- **Execution**: ready fork steps execute with bounded parallelism and deterministic lexicographic step-id ordering.
- **Join**: consume branch outputs via `@state:<save_as_key>` and run only when required inputs are available.

---

## 5‑Minute Demo

From the `adl` directory:

```bash
# Happy path: v0.7 primitive schema baseline
cargo run -q --bin adl -- examples/v0-5-primitives-minimal.adl.yaml --print-plan

# Optional: verify pattern compiler canonical IDs
cargo run -q --bin adl -- examples/v0-5-pattern-fork-join.adl.yaml --print-plan

# Optional: verify remote execution wiring (requires local adl-remote server)
cargo run -q --bin adl -- examples/v0-5-remote-execution-mvp.adl.yaml --print-plan
```

Expected output includes deterministic step ordering and resolved provider bindings.
Using `-q` keeps demo output focused on the ADL plan rather than Cargo build noise.

For real `--run` execution, configure provider runtime dependencies (for example local Ollama and any required auth env vars).

For additional runnable examples, start with `../demos/README.md`.
Use `examples/README.md` when you specifically want the full crate-local fixture inventory.

---

## CLI

```bash
cargo run -q --bin adl -- <path-to-adl.yaml> [OPTIONS]
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

## Signing Quickstart (v0.7)

For v0.7 workflows, signature enforcement is enabled by default for `--run`.

```bash
# 1) generate local dev keys
cargo run -q --bin adl -- keygen --out-dir ./.keys

# 2) sign a workflow
cargo run -q --bin adl -- sign examples/v0-5-pattern-linear.adl.yaml \
  --key ./.keys/ed25519-private.b64 \
  --out /tmp/signed.adl.yaml

# 3) verify signature
cargo run -q --bin adl -- verify /tmp/signed.adl.yaml --key ./.keys/ed25519-public.b64

# 4) run signed workflow (no override needed)
cargo run -q --bin adl -- /tmp/signed.adl.yaml --run
```

Dev-only bypass:

```bash
cargo run -q --bin adl -- examples/v0-5-pattern-linear.adl.yaml --run --allow-unsigned
```

---

## Run State Artifacts

When running with `--run`, `adl` writes deterministic run state files under:

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

`adl` supports a minimal remote provider via `type: "http"` for deterministic,
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
        env: ADL_REMOTE_BEARER_TOKEN
```

Run it with:

```bash
cargo run -q --bin adl -- examples/v0-3-remote-http-provider.adl.yaml --print-plan
cargo run -q --bin adl -- examples/v0-3-remote-http-provider.adl.yaml --run
```

Failure behavior is explicit:
- Missing endpoint -> config error
- Missing auth env var -> config error naming the env var
- Non-200 response -> runtime error with status + body snippet
- Timeout -> runtime error with timeout guidance

## Remote Execution MVP (v0.7 Placement)

In addition to HTTP providers, v0.7 includes a minimal remote execution protocol for
step placement:

- `GET /v1/health`
- `POST /v1/execute` (single fully-resolved step payload)

Design boundary:
- Scheduler ownership stays local in `adl`.
- Remote server executes exactly one resolved step and returns `{ok,result,error}`.
- Remote server does not compile/plan/schedule workflows.

Limits:
- Request payloads over 5 MiB are rejected (`413`).
- No authn/authz in the current MVP (deferred).

### Security Model / Threat Model (v0.7)

This remote execution path is an MVP transport boundary, not a hardened public
service. Treat it as trusted-network infrastructure only.

Threat-model assumptions:
- `adl-remote` runs on localhost or a tightly controlled private network.
- The caller and remote endpoint are operated by the same trusted team.
- Network peers are trusted or isolated by external controls.

Current protections:
- Request-size guard: payloads larger than 5 MiB are rejected (`413`).
- Placement boundary: scheduler/planner remain local; remote executes exactly
  one fully resolved step from `POST /v1/execute`.
- Timeout/error handling: transport timeout/unreachable/bad-status/invalid-json
  failures are surfaced with explicit stable error categories.

Known gaps / risks (v0.7):
- No request signing for remote payloads.
- No built-in authentication/authorization.
- Unsafe to expose directly on a public interface.

Operational guidance:
- Bind to loopback for local development:
  - `cargo run -q --bin adl-remote -- 127.0.0.1:8787`
- If cross-host is required, prefer private networking plus an authenticated
  tunnel (for example SSH tunnel or VPN), and restrict ingress with firewall
  rules to trusted sources only.
- Log and monitor remote-server lifecycle/events and non-2xx responses so
  misconfiguration or abuse is visible quickly.

Forward-looking hardening work (v0.7):
- Remote execution security envelope: https://github.com/danielbaustin/agent-design-language/issues/370
- Signing trust policy tightening: https://github.com/danielbaustin/agent-design-language/issues/371

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

Legacy examples (e.g. `adl-0.1.yaml`) remain for regression testing, but the runtime behavior described here reflects v0.7.

The schema/runtime behavior described here is aligned with current **v0.7** support.

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

`adl` enforces a **high bar for test coverage**, especially for core compiler-like behavior (parsing, validation, resolution, and execution).

As of v0.7:

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

- **Line coverage > function coverage** for v0.7  
  (many small helper functions are intentionally exercised indirectly)
- No “coverage theater”:
  - No dummy tests
  - No blanket `#[allow(dead_code)]` without justification
- Coverage is used as a **design signal**, not a vanity metric

Contributors are expected to keep overall line coverage **≥ 85%**, and ideally improve it with each change.

Badge note:
- `coverage` is the live Codecov percentage for the `adl` upload.
- `adl-coverage-gate` reflects whether the CI workflow coverage gate passes. It is a status badge, not a live percentage badge.

---

## License

Apache-2.0
