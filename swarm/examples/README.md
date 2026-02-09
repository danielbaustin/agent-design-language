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
cargo run -- examples/v0-2-coordinator-agents-sdk.adl.yaml --run --trace --out ./out --quiet --open
open ./out/index.html
```

## v0.2 failure-mode examples

- `v0-2-failure-unknown-field.adl.yaml`
  - Expected: non-zero exit; error mentions the unknown field (e.g., `modell`).
- `v0-2-failure-unknown-state-ref.adl.yaml`
  - Expected: non-zero exit; error indicates missing input bindings (e.g., `summary_2`).
