# Release Notes: ADL v0.3.0

ADL v0.3.0 establishes the deterministic execution foundation required for true runtime concurrency in future releases.

This release introduces a plan-only fork/join model, a hardened Remote HTTP provider MVP, polished demo UX, stabilized tooling, and cleaned CI workflows.

v0.3 is focused on discipline: determinism, explicit semantics, and reproducible execution.

---

## Highlights

- Deterministic fork/join model (plan-only; sequential runtime execution).
- Remote HTTP provider MVP hardened with explicit timeout, retry, and HTTP error semantics.
- Demo UX improvements emphasizing quiet output and one obvious first command.
- Tooling and git hygiene stabilization for `pr.sh`, worktree-safe flows, and branch reliability.
- CI and coverage workflow hardening with stabilized generation and clearer gating.

---

## What’s New in Detail

### Concurrency model (deterministic, not parallel yet)

- `workflow.kind: concurrent` is accepted in v0.3 and resolved deterministically.
- Fork branches execute sequentially in declared order at runtime.
- Join steps may consume fork outputs via state.
- Runtime multithreading is intentionally deferred; v0.3 establishes deterministic semantics first.

### Provider abstraction improvements

- Remote HTTP provider MVP supports a single configured HTTPS endpoint.
- Deterministic error surfaces for missing configuration, timeout, and non-2xx responses.
- Local and remote provider resolution behavior aligned with deterministic runtime validation.

### Retry semantics

- Step-level retry policy via `retry.max_attempts` implemented deterministically (no backoff or jitter).
- `on_error: fail|continue` honored for workflow control behavior.
- Retry and failure paths covered with execution tests.

### Trace and artifact improvements

- Deterministic trace behavior preserved across v0.3 flows.
- Run-state artifacts persisted under `.adl/runs/<run_id>/` including `run.json` and `steps.json`.
- Artifact shape is stable and intended for auditability and reproducibility.

### Documentation updates

- READMEs updated to reflect active v0.3 behavior and examples.
- Quickstart prioritizes one clear, low-friction command path.
- Badge semantics clarified to distinguish `main` status from per-PR checks.

---

## Upgrade Notes

- v0.3 documents are accepted by schema and runtime validation.
- No breaking runtime behavior changes were introduced from v0.2 deterministic guarantees.
- Deterministic behavior remains a core invariant across planning, resolution, and execution.

---

## Known Limitations

- Fork execution remains sequential at runtime.
- No runtime thread pool or true parallel scheduler is included in v0.3.
- Remote provider remains MVP scope (single endpoint model).

These constraints are intentional to validate deterministic semantics before introducing parallel execution.

---

## What’s Next (v0.4 Preview)

- True runtime concurrency for eligible fork branches.
- Execution scheduling layer and concurrency-safe state coordination.
- Expanded provider orchestration beyond current MVP constraints.
- Deterministic parallel trace semantics suitable for replay and debugging.

---

## Validation Notes

- This release reflects test-validated behavior in the v0.3 runtime and CI workflows.
- It intentionally avoids claiming unshipped runtime parallelism.
