# v0.3 Concurrency Fork/Join Example

This example runs in v0.3 using deterministic sequential fork/join execution.

File:
- `swarm/examples/v0-3-concurrency-fork-join.adl.yaml`

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
- Runtime parallelism is intentionally deferred to a later version.
