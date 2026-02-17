# Release Notes: ADL v0.3.0

ADL v0.3.0 delivers deterministic fork/join (plan-only) concurrency, a hardened Remote HTTP provider MVP, polished demo UX, stabilized tooling, and hardened CI coverage workflows.

This release moves ADL from proof-of-concept orchestration toward a disciplined deterministic execution engine foundation that can support true runtime concurrency in v0.4.

## Highlights

- Deterministic fork/join execution model for v0.3 workflows, with plan and run ordering fixed by declared step order.
- Remote HTTP provider MVP hardened with explicit timeout, retry, and HTTP error behavior.
- Demo UX polish emphasizing quiet output and one obvious first command.
- Tooling and git hygiene improvements for `pr.sh` workflows, worktree-safe flows, and branch/PR reliability.
- CI and coverage hardening, including stabilized coverage generation and clearer upload gating behavior.

## What's New in Detail

### Concurrency model (deterministic, not parallel yet)

- `workflow.kind: concurrent` is accepted in v0.3 and resolved deterministically.
- Fork branches execute sequentially in declared order at runtime.
- Join semantics are supported through explicit state wiring (`@state:<key>`).
- Runtime multithreading is intentionally deferred; this release is about deterministic semantics first.

### Provider abstraction improvements

- Remote HTTP provider MVP supports a single configured endpoint with explicit request/response contract.
- Provider behavior includes deterministic error surfaces for missing config, missing auth env, timeout, and non-200 responses.
- Local provider and remote provider plan resolution behavior is aligned with deterministic runtime validation.

### Retry semantics

- Step-level retry policy via `retry.max_attempts` is implemented deterministically (no backoff/jitter).
- `on_error: fail|continue` is honored for workflow control behavior.
- Retry and failure semantics are covered with execution tests for success-on-retry and retry exhaustion paths.

### Trace and artifact improvements

- Deterministic trace behavior remains intact across v0.3 flows.
- Run-state artifacts are persisted under `.adl/runs/<run_id>/` including `run.json` and `steps.json`.
- Artifact shape is stable and intended for auditability and reproducibility.

### Documentation updates

- READMEs were updated to reflect active v0.3 behavior and examples.
- Quickstart flow prioritizes one clear, low-friction command path.
- Badge semantics were clarified to distinguish `main` status from per-PR checks.

## Upgrade Notes

- v0.3 documents are accepted by schema and runtime validation.
- No breaking runtime behavior changes were introduced from v0.2 deterministic execution guarantees.
- Deterministic behavior remains a core invariant across planning, resolution, and execution paths.

## Known Limitations

- Fork execution remains sequential at runtime.
- No runtime thread pool or true parallel scheduler is included in v0.3.
- Remote provider remains MVP scope (single endpoint model, minimal orchestration features).

## What's Next (v0.4 Preview)

- True runtime concurrency for eligible fork branches.
- Execution scheduling layer and concurrency-safe state coordination.
- Expanded provider orchestration features beyond current MVP constraints.
- Deterministic parallel trace semantics suitable for replay and debugging.

## Validation Notes

- This release note reflects test-validated behavior in the v0.3 runtime and CI workflows.
- It intentionally avoids claiming unshipped runtime parallelism.
