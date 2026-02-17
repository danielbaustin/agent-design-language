# v0.3 Concurrency Fork/Join Example

This example runs in v0.3 using deterministic sequential fork/join execution.

File:
- `swarm/examples/v0-3-concurrency-fork-join.adl.yaml`

## Commands

From repo root:

```bash
cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-concurrency-fork-join.adl.yaml --print-plan
cargo run -q --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-concurrency-fork-join.adl.yaml --run --trace --out out
```

## Mental Model (v0.3)

- **Fork**: branches are declared as steps in a `workflow.kind: concurrent` workflow.
- **Branch execution**: runtime is still single-threaded; branch steps run in deterministic declared order.
- **Join**: join step consumes saved branch outputs via `@state:<save_as_key>` and runs only after required inputs are available.

## Artifacts

Run outputs are deterministic and easy to inspect:

- `out/fork/alpha.txt`
- `out/fork/beta.txt`
- `out/fork/join.txt`

Run metadata is written under:

- `.adl/runs/<run_id>/run.json`
- `.adl/runs/<run_id>/steps.json`

## Expected Deterministic Trace Ordering

Expected high-level event order:

1. `StepStarted(fork.plan)`
2. `PromptAssembled(fork.plan, hash)`
3. `StepFinished(fork.plan, success)`
4. `StepStarted(fork.branch.alpha)`
5. `PromptAssembled(fork.branch.alpha, hash)`
6. `StepFinished(fork.branch.alpha, success)`
7. `StepStarted(fork.branch.beta)`
8. `PromptAssembled(fork.branch.beta, hash)`
9. `StepFinished(fork.branch.beta, success)`
10. `StepStarted(fork.join)`
11. `PromptAssembled(fork.join, hash)`
12. `StepFinished(fork.join, success)`
13. `RunFinished(success)`

Notes:
- Branch execution order is deterministic by declared step order (`alpha`, then `beta` in this file).
- Join uses explicit state inputs (`alpha`, `beta`) saved by upstream branch steps.
- Runtime parallelism is intentionally deferred to a later version.
