# v0.2 Coordinator Example (Agents SDK–Style)

This example mirrors the structure of the OpenAI Agents SDK demo (Coordinator + specialists),
but expressed declaratively in ADL:

- A coordinator agent defines the brief and performs final review.
- Specialist agents (researcher, analyst, writer) produce intermediate artifacts.
- Artifacts are passed explicitly via named inputs and `save_as` fields.

## Mapping to the SDK demo

- **Coordinator agent** → `coordinator` steps (`coordinator-brief`, `final`).
- **Specialists** → `researcher`, `analyst`, `writer` steps.
- **Artifacts** → `brief`, `research_notes`, `analysis`, `draft`, `final_output`.

## Why ADL is deterministic and auditable

- All inputs are explicit; no implicit memory or hidden state.
- The workflow is a fixed, sequential plan with named steps.
- `--trace` emits a stable, machine-friendly event stream for review.

## Run

From `swarm/`:

```bash
cargo run -- examples/v0-2-coordinator-agents-sdk.adl.yaml --run --trace
```

## Note on artifact wiring

The current runtime keeps handoffs explicit by listing inputs directly.
`save_as` names the intended artifact for future wiring while preserving
fully deterministic inputs today.
