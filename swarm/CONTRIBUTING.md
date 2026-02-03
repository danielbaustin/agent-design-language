# Contributing to Swarm (ADL Reference Runtime)

## Scope and layering

This directory (`/swarm`) contains the **reference runtime + CLI** for ADL.

- **Language semantics and evolution live in** `/adl-spec`
- **Runtime implementation details live here** in `/swarm`

**Rule of thumb:** If a change affects ADL *meaning* (semantics, versioning behavior, schema intent), propose it in `/adl-spec` first. If it affects *how Swarm executes* (performance, ergonomics, provider wiring, CLI behavior) it belongs here.

---

## What we optimize for

Swarm is intentionally opinionated:

- **Determinism first** (resolution, prompt assembly, execution order)
- **Traceability** (runs and step lifecycles should be observable)
- **Schema discipline** (schema/fixtures/examples stay aligned)
- **Hermetic tests** (no network, no real providers)
- **Small diffs** and **high auditability**

If you are unsure whether a change preserves these properties, open an issue or propose a plan before making broad edits.

---

## Quick start

From `swarm/`:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test