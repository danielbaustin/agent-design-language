# Demo Planning — ADL v0.75

## Purpose

This document defines the demonstrations used to validate and communicate the core capabilities of **ADL v0.75**.

Demos serve three purposes:

1. **Validation** — ensure the runtime behaves as expected.
2. **Review support** — allow external reviewers to reproduce behavior.
3. **Communication** — demonstrate milestone progress to the community.

v0.75 demos focus on:

- Deterministic execution and replay
- Learning artifact export
- ObsMem integration boundary
- Hierarchical planner example

---

# Demo Inventory

## D‑10 Deterministic Run + Replay

### Goal
Demonstrate that an ADL run can be reproduced deterministically using exported artifacts.

### Scenario
A simple multi‑step agent workflow runs locally and produces:

- run trace
- artifact bundle
- deterministic replay

### Expected Output

Artifacts produced:

```
run_summary.json
trace.json
artifacts/
```

Replay should reproduce the same execution outcome and trace structure.

### Validation

Reviewer verifies:

- identical outputs between runs
- replay produces the same trace structure
- no nondeterministic state leakage

---

## D‑11 Learning Artifact Export

### Goal
Demonstrate the **learning export bundle** introduced in v0.7 and finalized in v0.75.

### Scenario

An ADL job produces a structured export bundle containing:

- trace data
- artifact metadata
- scoring information
- suggestions surface

### Bundle Structure (Example)

```
learning_bundle/
    run_summary.json
    trace.json
    artifacts/
    suggestions.json
```

### Validation

Reviewer verifies:

- bundle contains no secrets
- artifacts are structured for downstream indexing
- bundle can be consumed by external learning systems

---

## D‑12 Hierarchical Planner Example

### Goal

Demonstrate ADL’s ability to express a **hierarchical planning agent**.

This demo was inspired by the hierarchical planner architecture article referenced during v0.7 development.

### Scenario

A planner agent decomposes a task into subtasks and executes them using structured agents.

Example execution model:

User request
→ Planner Agent
→ Task decomposition
→ Execution agents
→ Aggregated result

### Implementation

Example job definition:

```
examples/v0-7-hierarchical-planner.adl.yaml
```

### Validation

Reviewer verifies:

- planner produces structured subtasks
- execution tasks run sequentially or hierarchically
- trace reflects both planning and execution stages

---

## D‑13 Planner Artifact Contract (Plan Export)

### Goal

Elevate the hierarchical planner demo so that the **plan itself becomes a first‑class artifact** that can be inspected, diffed, replayed, and indexed by learning systems.

Instead of the planner being only runtime behavior, the planning phase will emit a canonical artifact describing the execution plan.

### Scenario

Execution becomes a two‑phase process:

1. **PLAN phase** – the planner decomposes the task and emits a structured plan artifact.
2. **EXECUTE phase** – the runtime executes that plan and produces the normal run artifacts.

### Expected Artifact

The planner emits a deterministic plan artifact such as:

```
artifacts/plan_v1.json
```

Example conceptual structure:

```
{
  "plan_id": "stable-hash",
  "goal": "original user task",
  "steps": [
    {
      "step_id": "S1",
      "kind": "subtask",
      "deps": [],
      "inputs": { }
    }
  ],
  "policy": {
    "max_depth": 5,
    "max_steps": 50
  }
}
```

### Validation

Reviewer verifies:

- planner produces a deterministic `plan_v1.json`
- identical inputs produce identical plan hashes
- the plan artifact appears in the learning export bundle

### Rationale

Making the plan a stable artifact enables:

- deterministic planning verification
- diffable plans across runs
- security review of planned actions
- indexing by ObsMem or other learning systems

This also prepares ADL for future **authoring surfaces and planning introspection tools**.

---

# Demo Execution Procedure

Before running demos:

- Use the runtime workspace directory (the directory that contains `Cargo.toml` for the ADL runtime).
- Commands below assume you are in that runtime workspace directory.

Build + test:

```
cargo build --workspace
cargo test --workspace
```

Run the hierarchical planner example (execute + trace):

```
# Print the resolved plan (dry run)
cargo run --bin adl -- examples/v0-7-hierarchical-planner.adl.yaml --print-plan --allow-unsigned

# Execute the workflow and emit trace artifacts
cargo run --bin adl -- examples/v0-7-hierarchical-planner.adl.yaml --run --trace --allow-unsigned
```

Note: ADL enforces workflow signatures by default. The example workflow used in this demo is intentionally unsigned, so the `--allow-unsigned` flag is required when running locally for development and review purposes.

Verify:

- run artifacts are written under `.adl/runs/...` (see `--help` for defaults)
- a `trace.json` is produced when `--trace` is used
- learning export bundle commands (D-11) succeed and produce deterministic, sanitized output

---

# Demo Requirements for Release

The following must be true before the v0.75 release:

- [ ] All demos run successfully on a clean checkout
- [ ] Example files live under `examples/` in the runtime workspace
- [ ] Demo artifacts are reproducible
- [ ] Documentation references correct example paths

---

# Reviewer Guidance

External reviewers should be able to:

1. Clone repository
2. Build runtime
3. Run demo examples
4. Inspect generated artifacts

Expected review time: **15–30 minutes**.

---

# Future Demo Expansion (v0.8)

Planned demonstrations for the next milestone:

- Gödel agent reasoning loop
- Authoring surface (structured prompt → ADL job)
- expanded multi‑agent coordination examples
