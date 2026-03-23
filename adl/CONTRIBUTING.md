# Contributing to Swarm (ADL Reference Runtime)

Swarm is the **reference runtime + CLI** for Agent Design Language (ADL).

All contribution workflow, governance, and repository-wide policies are defined in the root:

- `../CONTRIBUTING.md`

This file exists only to clarify Swarm-specific expectations and to prevent process drift.

---

## Before You Start

Please read:

- `../CONTRIBUTING.md` (canonical workflow + governance)
- `../docs/codex_playbook.md` (card-based PR workflow)
- `../docs/design_goals.md` (stable principles)

---

## Swarm-Specific Expectations

In addition to the root contribution rules:

- Tests must remain **hermetic** (no real network calls, no real providers).
- Runtime changes must not alter deterministic planning semantics.
- Changes affecting ADL meaning must be proposed in `/adl-spec` first.
- Coverage discipline (>=80% per file or documented exception) applies.

If unsure whether a change affects language semantics vs runtime implementation, open an issue first.

---

## Quick Local Validation (from `adl/`)

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

---

Swarm defers to the root `CONTRIBUTING.md` for all process, workflow, and governance rules.
