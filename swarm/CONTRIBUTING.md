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
```

---

## Workflow (Source of Truth: `swarm/CODEX_PLAYBOOK.md`)

For the full collaboration workflow (including input/output cards and `swarm/tools/pr.sh`),
use `swarm/CODEX_PLAYBOOK.md` as the canonical guide. This file keeps a lightweight summary
to avoid drift.

Canonical loop:

```
start → cards → execute → review → finish → merge → cleanup
```

Key points:
- Input/output cards are **local-only** trace artifacts stored under `.adl/cards/` (not committed).
- Templates live under `.adl/templates/` (versioned).
- Tasks can be non-code; the same card-based trace applies.

## Fast path (copy/paste)

```bash
swarm/tools/pr.sh start <issue>
swarm/tools/pr.sh cards <issue>
# do the work + tests
swarm/tools/pr.sh finish <issue> --title "swarm: <short description>" \
  -f .adl/cards/issue-####__input__v0.2.md \
  --receipt .adl/cards/issue-####__output__v0.2.md
```

## Recovery (common pitfalls)

- **Wrong branch:** `git switch main` → `swarm/tools/pr.sh start <issue>`
- **Finish after manual commit:** `swarm/tools/pr.sh finish ...` still works; it will commit staged changes.
- **Issue vs PR number confusion:** always use the **issue** number for cards/branches.
