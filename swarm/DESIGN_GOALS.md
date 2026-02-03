# Agent Design Language (ADL) — Design Goals

This document defines the guiding principles behind **Agent Design Language (ADL)** and its reference Rust runtime, `swarm`.

ADL is designed as a *language*, not a framework: a small, explicit, and deterministic way to describe agent workflows that can be parsed, validated, reasoned about, and executed by multiple runtimes.

---

## 1. Determinism First

ADL is deterministic by design.

Given the same ADL document and inputs:

- Resolution produces the same execution plan
- Prompt assembly produces identical text
- Tracing produces reproducible event sequences

There is no implicit execution, background mutation, or runtime-dependent behavior during planning.

Execution engines may be adaptive; **planning is not**.

---

## 2. Compiler-Inspired Architecture

ADL follows a compiler-style pipeline:

1. **Parse** — Load ADL YAML into a structured model
2. **Validate** — Ensure all references are well-formed
3. **Resolve** — Bind run → workflow → steps → tasks → agents → providers
4. **Materialize** — Produce deterministic artifacts:
   - Execution plans
   - Assembled prompts
   - Trace events

Each phase is explicit and independently testable.

---

## 3. Separation of Concerns

ADL strictly separates:

- **Specification** (YAML documents)
- **Resolution** (reference binding and validation)
- **Prompt Assembly** (deterministic text construction)
- **Execution** (providers, tools, retries — future work)

This ensures:
- Clear failure modes
- Predictable behavior
- Minimal coupling between components

---

## 4. Explicitness Over Magic

ADL avoids hidden or heuristic behavior.

There is no:
- Implicit agent selection
- Implicit tool invocation
- Hidden system prompts
- Automatic retries or self-repair

Instead, ADL favors:
- Explicit agent, task, and provider references
- Structured prompt sections
- Explicit inputs and guards
- Fail-fast validation with clear errors

If something happens, it must be visible in the document or the trace.

---

## 5. Testability as a First-Class Constraint

Everything ADL produces is testable **without calling a model**:

- Resolved execution plans
- Prompt text
- Trace events
- CLI behavior

This enables:
- CI without API keys
- Deterministic regression tests
- Safe refactoring of runtimes

The reference runtime intentionally tests language semantics, not provider behavior.

---

## 6. Small Core, Composable Extensions

The ADL core is intentionally minimal:

- Agents
- Tasks
- Workflows
- Prompts
- Inputs
- Tracing

Advanced features such as:
- Tool invocation
- Contracts and guarantees
- Repair policies
- MCP integration
- Remote providers

are designed to layer *on top* of the core without changing its semantics.

---

## 7. Language, Not Framework

ADL is a language definition, not a framework API.

- YAML is the interface boundary
- Runtimes may be written in Rust, Python, or other languages
- Users should not need to write code to use ADL

This allows ADL to be:
- Embedded in CI pipelines
- Used on edge devices
- Evaluated statically
- Implemented by multiple independent runtimes

---

## 8. Conservative by Default

The reference runtime (`swarm`) intentionally prioritizes:

- Strict validation
- Clear error messages
- Deterministic output
- Clean `fmt + clippy` builds with `-D warnings`

Other runtimes may choose to be more permissive; the reference implementation defines the baseline semantics.

---

## 9. Observability via Tracing

ADL includes first-class support for tracing:

- Step lifecycle events
- Prompt assembly visibility
- Deterministic timestamps and ordering

Tracing is designed to support:
- Debugging
- Auditing
- Reproducibility
- Evaluation tooling

---

## Non-Goals

ADL is explicitly **not**:

- An autonomous agent framework
- A prompt auto-tuning system
- A replacement for application code
- A reasoning engine or planner

Those behaviors belong *above* ADL, not inside it.

---

## Summary

ADL is designed to be:

- Deterministic
- Explicit
- Testable
- Minimal
- Composable
- Runtime-agnostic

The goal is not to automate intelligence, but to **make agent systems legible, inspectable, and reliable**.

---

## Next Steps

With the core stabilized, the next milestones are:

1. Canonical example ADL documents
2. Tool invocation semantics
3. Provider abstraction (local + remote)
4. Python reference runtime
5. Formal specification versioning
