# v0.3 Concurrency Fork/Join Demo (Plan-Only)

This demo is intentionally plan-only in v0.3. Parsing and planning are supported;
concurrent runtime execution remains intentionally unimplemented.

File:
- `swarm/examples/v0-3-concurrency-fork-join.adl.yaml`

## One-obvious command

From repo root:

```bash
cargo run --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-concurrency-fork-join.adl.yaml --print-plan
```

Expected: plan output that includes `fork.plan`, `fork.branch.alpha`,
`fork.branch.beta`, and `fork.join`.

If you run with `--run`, expect an explicit "not implemented yet for ADL v0.3"
error.

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
- Branch execution order is deterministic by branch key (`alpha`, then `beta`).
- This example defines ordering contract; runtime parallelism is intentionally deferred.
