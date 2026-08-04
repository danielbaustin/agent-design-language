# Contributing to Agent Design Language (ADL)

Thanks for contributing to ADL.

ADL is structured as a language + reference runtime. This document defines the **canonical contribution workflow and governance model for the entire repository**.

If a directory contains its own `CONTRIBUTING.md`, it must defer to this file.

---

## Repository Structure (High-Level)

- `/adl-spec` — Language semantics and schema definitions
- `/adl` — Reference Rust runtime + CLI
- `/docs` — Milestone docs, ADRs, and release notes

**Rule of thumb:**
- If a change affects ADL *meaning* (semantics, versioning, schema intent), propose it in `/adl-spec` first.
- If a change affects *how ADL executes* (performance, ergonomics, CLI behavior, provider wiring), it belongs in `/adl`.

---

## Canonical Workflow

Source-of-truth quick links:
- `docs/codex_playbook.md`
- `AGENTS.md`
- `csdlc-v2/operator/skills/`

Workflow loop:

```
start → cards → execute → review → finish → merge → cleanup
```

Card semantics:
- Input/output cards are **local-only trace artifacts** under `.adl/cards/` (not committed).
- Templates live under `docs/templates/` (versioned).
- Tasks can be non-code; the same card-based trace applies.

Lifecycle entrypoints:

```bash
.adl/bin/csdlc-v2/csdlc-install resolve --repo . --issue <issue>
.adl/bin/csdlc-v2/csdlc-init --root <worktree> --request <bootstrap-request.json>
.adl/bin/csdlc-v2/csdlc-doctor --repo <repo> --issue <issue>
.adl/bin/csdlc-v2/csdlc-bind --root <worktree> --request <bind-request.json>
```

Continue through the typed `csdlc-validate`, `csdlc-review`, and
`csdlc-publish` skills and request contracts. Tracked edits begin only after
binding and creation of the issue-bound session goal.

---

## Determinism and Design Constraints

ADL optimizes for:

- Determinism (resolution, planning, ordering semantics)
- Traceability (observable, reproducible runs)
- Schema discipline (explicit versioning, no implicit behavior)
- Small, auditable diffs

Changes must preserve deterministic semantics unless explicitly version-gated.

---

## Testing and Coverage Discipline

Typical local validation from `adl/`:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Coverage discipline (v0.6+):

- >=80% coverage per file
- Exceptions require an owner + linked issue
- New logic paths must include tests
- No “coverage-only churn”

Tests must remain hermetic (no real network calls, no real providers).

---

## Documentation Responsibilities

- Root `README.md` is the repo entrypoint.
- `adl/README.md` is the runtime entrypoint.
- Milestone work updates `docs/milestones/<version>/`.
- Architectural decisions must be captured under `docs/adr/`.

Do not duplicate narrative across README files. Prefer link-outs to canonical locations.

---

## Security

See `SECURITY.md` for vulnerability disclosure guidance.

---

## When in Doubt

Open an issue first.

Propose intent clearly.

Keep changes small, deterministic, and reviewable.
