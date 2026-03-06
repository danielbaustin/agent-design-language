# ADL Demo: Rust Transpiler / Migration Workflow

## Purpose

This document defines a **flagship ADL demo** showing how deterministic workflows, adaptive execution, verification hooks, and replayable artifacts can be used to safely perform automated code transformations.

The demo demonstrates how ADL can:

- analyze a Rust codebase
- generate a transformation patch
- apply the patch
- verify correctness using Rust tooling
- retry bounded repair strategies when verification fails
- emit deterministic artifacts and evidence for audit and replay

This showcases ADL’s **Adaptive Execution Engine** and **replayable execution substrate**.

---

# Demo Goals

The Rust transpiler demo should prove that ADL can orchestrate complex engineering workflows while remaining:

- deterministic
- auditable
- replayable
- provider‑agnostic

The demo intentionally resembles a real enterprise migration workflow.

Inputs:

- a small Rust repository (fixture crate)
- a migration rule or transformation goal

Outputs:

- transformation patch
- verification results
- evidence report
- trace bundle v2

---

# Why This Demo Matters

Many "AI coding" demos simply generate code.

This demo shows something different:

ADL provides **structured orchestration** around code generation.

Key capabilities demonstrated:

- policy‑gated retries
- deterministic replay
- artifact traceability
- verification‑driven convergence

In other words, the agent **does not just generate code** — it executes a **reliable engineering process**.

---

# Example Transformation

The initial demo should use a deterministic, repeatable migration such as:

## Option A — Error Modernization

Convert ad‑hoc error handling into a typed error enum.

Example transformation:

- detect `Result<T, String>` patterns
- introduce `enum AppError`
- update call sites

Verification:

- `cargo fmt`
- `cargo clippy`
- `cargo test`

## Option B — API Migration

Replace deprecated API usage with a new API.

Example:

- identify deprecated function usage
- rewrite calls
- update imports

Verification ensures the transformation preserves behavior.

---

# Demo Architecture

The demo is implemented as a **single ADL workflow** with clearly defined phases.

The workflow emits artifacts at every stage.

## Phase 1 — Intake

Analyze the repository.

Artifacts produced:

repo_inventory.json

Contains:

- crate metadata
- file list
- Rust toolchain version
- detected patterns relevant to the transformation

---

## Phase 2 — Transformation Plan

The agent generates a transformation plan describing:

- what changes are required
- which files will be modified
- expected compile/test impact

Artifact:

plan.json

This plan is deterministic for the same input repository.

---

## Phase 3 — Patch Generation

The workflow generates a patch representing the transformation.

Artifact:

patchset/

Contains:

- patch.diff
- rationale.md

The patch is hashed and recorded for reproducibility.

---

## Phase 4 — Patch Application

The workflow applies the patch inside the sandbox.

Artifacts:

apply_log.txt

And a snapshot of the modified workspace.

---

## Phase 5 — Verification

Verification hooks are executed.

Verification steps:

cargo fmt

cargo clippy -- -D warnings

cargo test

Artifacts:

verification/

Contains:

fmt.log

clippy.log

test.log

If all checks pass, the workflow proceeds to final reporting.

---

# Adaptive Execution Loop

If verification fails, the workflow enters the **Adaptive Execution loop**.

The loop:

1. Classifies the failure
2. Selects a repair strategy
3. Generates a corrective patch
4. Re‑runs verification

Retry attempts are bounded by policy.

Example strategies:

- compile‑fix
- lint‑fix
- test‑fix

Every attempt is recorded in the trace.

This demonstrates the **Sticktoitiveness / Adaptive Execution Engine**.

---

# Evidence Report

At completion, the workflow generates a deterministic evidence report.

Artifact:

evidence.json

Contains:

- final outcome
- patch hash
- verification results
- attempt history

The report includes citations pointing to verification logs.

---

# Trace Bundle

The entire run produces a **Trace Bundle v2**.

This bundle includes:

- run metadata
- activation logs
- artifacts
- evidence

The bundle can be replayed using the ADL replay engine.

This ensures the demo is:

- deterministic
- auditable
- reproducible

---

# Repository Layout

Recommended demo structure:

swarm/examples/

v0-8-rust-transpiler-demo.adl.yaml

fixtures/rust-demo-crate/

The fixture crate must be:

- small
- deterministic
- compile/test clean before transformation

---

# Demo Success Criteria

The demo is considered complete when:

1. The workflow transforms the fixture crate.

2. The crate compiles successfully after transformation.

3. All tests pass.

4. A deterministic evidence report is produced.

5. A replay from the trace bundle reproduces the same outcome.

---

# What This Demonstrates About ADL

This demo shows that ADL provides a **reliable engineering workflow layer** around AI.

Capabilities demonstrated:

- deterministic orchestration
- verification‑driven workflows
- bounded adaptive retries
- traceable artifact production
- replayable execution

This positions ADL as a framework for **safe autonomous software operations**, not merely prompt-driven code generation.

---

# Relationship to Gödel Self‑Improvement Experiments

The Rust transpiler demo also serves as the **first practical substrate for Gödel‑style self‑improvement experiments in ADL**.

The workflow structure in this demo mirrors the scientific loop used by the planned Gödel agent system:

1. **Observe** – capture run artifacts and execution traces
2. **Hypothesize** – generate a candidate modification (patch or improvement)
3. **Evaluate** – run deterministic verification (tests, lint, compile)
4. **Accept or Reject** – commit only if evaluation criteria pass

In the Rust transpiler demo:

- The **patch generation step** acts as the mutation/hypothesis.
- The **verification phase** acts as the deterministic evaluation function.
- The **Adaptive Execution loop** acts as the bounded search process.

Because all artifacts are captured in the **Trace Bundle v2**, the experiment can be replayed and compared across runs.

This means the same workflow structure can later power:

- Gödel agent proposal evaluation
- policy optimization experiments
- automated repair and improvement loops

The key architectural insight is that **ADL does not require a separate "learning system"**.

Instead, learning and improvement are implemented as **ordinary ADL workflows operating over deterministic artifacts**.

The Rust transpiler demo therefore doubles as:

- a developer‑facing demonstration of ADL orchestration
- a minimal, concrete example of the Gödel experimentation substrate

This dual purpose makes it an ideal flagship demo for the v0.8 roadmap.