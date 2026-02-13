# Release Notes: ADL v0.2

ADL v0.2 expands the reference runtime and examples while preserving the deterministic execution model from v0.1.

## What shipped

- Multi-step v0.2 example suite in `examples/v0-2-multi-step-basic.adl.yaml`, `examples/v0-2-multi-step-file-input.adl.yaml`, and `examples/v0-2-coordinator-agents-sdk.adl.yaml`.
- Failure-mode examples with deterministic, testable errors in `examples/v0-2-failure-unknown-field.adl.yaml` and `examples/v0-2-failure-unknown-state-ref.adl.yaml`.
- Stricter validation and clearer failure behavior for invalid schemas and unresolved inputs.
- Trace UX improvements for v0.2 runs (human-readable timestamp/duration fields in trace output).
- Provider safety improvements, including bounded subprocess behavior and timeout validation.

## What is deferred to v0.3+

- Concurrency primitives and parallel workflow execution.
- DAG scheduling and dynamic branching semantics.
- Streaming/tool execution semantics beyond current v0.2 scope.

## Flagship demo (copy/paste)

Run from the `swarm/` directory.

Inspect the coordinator plan (no provider call required):

```bash
cargo run -- examples/v0-2-coordinator-agents-sdk.adl.yaml --print-plan
```

Run the coordinator workflow (requires local Ollama provider setup):

```bash
cargo run -- examples/v0-2-coordinator-agents-sdk.adl.yaml --run --trace
```

If local provider setup is not ready yet, use the plan command above as the recommended quickstart and track demo UX polish in the following issues:
- https://github.com/danielbaustin/agent-design-language/issues/60
- https://github.com/danielbaustin/agent-design-language/issues/71

## More examples

- Examples index: `examples/README.md`
- Coordinator walkthrough: `examples/v0-2-coordinator-agents-sdk.md`

## Key docs

- v0.2 schema extensions: `../adl-spec/ADL_v0.2_Schema_Extensions.md`
- ADL spec index: `../adl-spec/spec/README.md`
- Provider implementation reference: `src/provider.rs`
