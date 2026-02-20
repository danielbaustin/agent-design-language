# Swarm Examples

Run any example from the `swarm/` directory:

```bash
cargo run -- examples/<file> --run --trace
```

## v0.2 happy-path examples

- `v0-2-multi-step-basic.adl.yaml`
- `v0-2-multi-step-file-input.adl.yaml`
- `v0-2-coordinator-agents-sdk.adl.yaml` (see `v0-2-coordinator-agents-sdk.md`)

Expected: two steps execute in order and print outputs for each step.

Coordinator demo (writes HTML artifact):

```bash
cargo run -- examples/v0-2-coordinator-agents-sdk.adl.yaml
```

## v0.3 flagship demo

Run the deterministic artifact demo (single command):

```bash
cargo run -- demo demo-a-say-mcp --run --trace --open
```

Artifacts are written to:

```bash
out/demo-a-say-mcp/
```

Reference ADL doc for the demo scenario:
- `v0-3-demo-a-say-mcp.adl.yaml`

## v0.2 failure-mode examples

- `v0-2-failure-unknown-field.adl.yaml`
  - Expected: non-zero exit; error mentions the unknown field (e.g., `modell`).
- `v0-2-failure-unknown-state-ref.adl.yaml`
  - Expected: non-zero exit; error indicates missing input bindings (e.g., `summary_2`).

## v0.3 concurrency examples

- `v0-3-concurrency-fork-join.adl.yaml`
  - deterministic fork/join sequence contract (single-threaded runtime order)
  - clear branch/join artifacts: `fork/alpha.txt`, `fork/beta.txt`, `fork/join.txt`
  - see `v0-3-concurrency-fork-join.md` for mental model + deterministic trace ordering
- `v0-3-fork-join-seq-run.adl.yaml`
  - runnable v0.3 sequential fork/join execution
  - join input wiring via `@state:<save_as_key>`

Quick run from repo root:

```bash
cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-concurrency-fork-join.adl.yaml --run --trace --out out
```

## v0.3 on_error + retry example

- `v0-3-on-error-retry.adl.yaml`
- See `v0-3-on-error-retry.md` for deterministic failure-policy behavior.

## v0.3 remote provider demo

- `v0-3-remote-http-provider.adl.yaml`
- `v0-3-remote-provider-demo.adl.yaml` (compat alias)
- See `v0-3-remote-http-provider.md` for setup and expected behavior.

From repo root:

```bash
cargo run --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-remote-http-provider.adl.yaml --print-plan
```

## v0.5 primitives baseline

- `v0-5-primitives-minimal.adl.yaml`
  - defines all six primitives (`providers`, `tools`, `agents`, `tasks`, `workflows`, `run`)
  - demonstrates explicit `workflow_ref` and task references in steps
  - inline `run.workflow` is legacy-compatible but must not coexist with `workflow_ref`
  - when multiple providers exist, provider selection must be explicit
  - demonstrates `agent_ref` resolution from task

From repo root:

```bash
cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-5-primitives-minimal.adl.yaml --print-plan
```

## v0.5 pattern compiler examples

PatternSchema v0.1 compiles patterns into deterministic ExecutionPlan nodes with `p::<pattern_id>::...` step IDs.

Rules:
- task symbols in pattern `steps` must match task IDs in `tasks` (missing symbols fail with a clear validation error)
- fork branches are compiled in lexicographic `branch.id` order for stable plans across declaration order variants

- `v0-5-pattern-linear.adl.yaml`
- `v0-5-pattern-fork-join.adl.yaml`

Quick checks from repo root:

```bash
cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-5-pattern-linear.adl.yaml --print-plan
cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-5-pattern-fork-join.adl.yaml --print-plan
```

## v0.5 remote execution MVP example

- `v0-5-remote-execution-mvp.adl.yaml`
  - mixed placement in one workflow: local -> remote -> local
  - runner stays scheduler; remote executes one fully-resolved step via `/v1/execute`

Start local remote executor:

```bash
cargo run -q --manifest-path swarm/Cargo.toml --bin swarm-remote -- 127.0.0.1:8787
```

Then run the mixed-placement example from repo root:

```bash
SWARM_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh \
cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-5-remote-execution-mvp.adl.yaml --run --trace
```
