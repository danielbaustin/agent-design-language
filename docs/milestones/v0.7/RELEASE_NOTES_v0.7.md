# ADL v0.7 Release Notes (Living Draft)

## Metadata
- Product: ADL (Agent Design Language)
- Milestone: `v0.7`
- Version: `v0.7.x (release train)`
- Status: Draft (updated as PRs merge)
- Date: 2026-02-24
- Release manager: Daniel Austin
- Tag: Not yet tagged

---

## How to Read This Document

- This is a **living draft** during development. It becomes the final GitHub Release text at tag time (WP-16).
- **Do not treat planned work as shipped.** Items are marked as:
  - **Shipped**: merged to `main` and included in the tagged release.
  - **In progress**: actively being implemented/reviewed.
  - **Planned**: accepted for v0.7 but not yet merged.

---

## Summary

v0.7 is a two-phase release train:

- **v0.7.0 (Foundation)** hardens ADL’s runtime for deterministic, replayable multi-agent execution with security/trust boundaries and paper-driven delegation patterns.
- **v0.7.x minors (Learning train)** add overlay-based, opt-in learning features (observe → score → suggest → apply → export) without depending on ObsMem (deferred to v0.8).

Core principle: **no silent drift**. Adaptive behavior must be opt-in, artifacted, auditable, and reversible.

---

## Highlights

As of 2026-02-24: **Not yet shipped** (milestone docs bootstrap in progress).

---

## v0.7.0 — Foundation Release

### What’s New

As of 2026-02-24: **Not yet shipped**.

Planned areas for v0.7.0 (do not claim shipped until merged/tagged):
- Security envelope + trust model hardening (EPIC-E #429)
  - Sandbox hardening: prevent symlink-based escapes (#472)
  - Remote execution security envelope (#370)
  - Signing trust policy requirements (#371)
  - Remote request signing (#386)
- Delegation runtime (paper-driven / DeepMind-style patterns) (EPIC-B #413)
- Scheduler policy surface (per-workflow concurrency) (#369)
- Runtime resilience + checkpointing surfaces (EPIC-F #430)
- Canonical execution path cleanup (execute.rs deduplication) (#383)
- Cleanup + deferred hard systems work (EPIC-D #415)

### Upgrade Notes

- No v0.7.0 tag exists yet.
- When v0.7.0 ships, upgrade notes will include any CLI/env var changes and any migration steps.

### Known Limitations

- Learning loop features are **not part of v0.7.0** (they land in v0.7.x minors).
- ObsMem integration is deferred to **v0.8**.

---

## v0.7.x — Learning Train (Minors)

### What’s New

As of 2026-02-24: **Not yet shipped**.

Planned sequence (overlay-based, opt-in; no workflow YAML mutation):
1) Observe: `run_summary.json`
2) Score: scoring hooks
3) Suggest: explainable `suggestions.json`
4) Apply: deterministic overlays (opt-in)
5) Export: dataset export

### Hard Constraints (must hold for every minor)

- Overlay-based only; **no workflow YAML mutation**.
- Opt-in only; **no silent auto-promotion**.
- Artifacts are versioned and schema-validated (`deny_unknown_fields`).
- Learning surfaces remain independent of ObsMem (ObsMem is v0.8).

---

## Late v0.7 — Runtime Identity Migration (Do Last)

As of 2026-02-24: **Not yet shipped**.

- WP-12 / EPIC-H (#336 / #479) renames runtime identity late in v0.7:
  - Crate/package + binaries become `adl`
  - Keep the `swarm/` directory path stable in v0.7
  - One-release compatibility window:
    - legacy `swarm` / `swarm-remote` entrypoints remain as shims with deprecation warnings
    - legacy `SWARM_...` env vars continue to work with deprecation warnings; canonical env vars become `ADL_...`

---

## Validation Notes

At ship time (WP-16), the release must be supported by:
- `cargo fmt --all` (pass)
- `cargo clippy --all-targets -- -D warnings` (pass)
- `cargo test` (pass)
- CI green on merge target
- Demo matrix and runnable demos validated (WP-13 #474)
- Coverage/quality gate completed with documented exclusions (WP-14 #475)

---

## What’s Next (v0.8 preview)

These are explicitly out of scope for v0.7:
- ObsMem integration (memory-backed learning)
- Distributed/cluster execution (#339)
- Durable checkpoint / recovery engine (#340)
- Gödel agent promotion/evaluation harness expansion

---

## Exit Criteria

- Release notes reflect **only shipped behavior** at tag time.
- Planned items remain clearly marked until shipped.
- Final text is ready to paste into GitHub Release UI without further editing.
