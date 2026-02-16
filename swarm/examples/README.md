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
  - deterministic fork/join sequence contract
- `v0-3-fork-join-seq-run.adl.yaml`
  - runnable v0.3 sequential fork/join execution
  - join input wiring via `@state:<save_as_key>`
- See `v0-3-concurrency-fork-join.md` for expected deterministic trace ordering.

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
