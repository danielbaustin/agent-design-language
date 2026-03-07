# ADL v0.7 Release Notes (Historical Record)

## Metadata
- Product: ADL (Agent Design Language)
- Milestone: `v0.7`
- Version: `v0.7.x (release train)`
- Status: Released
- Date: 2026-02-24 (draft), status reconciled 2026-03-06
- Release manager: Daniel Austin
- Tag: `v0.7.0`

---

## How to Read This Document

- This file is retained as the historical release-train record for v0.7.
- Draft/planned phrasing below is preserved for historical context from the pre-release period.
- Current project state: v0.7 is released; active development milestone is v0.75.

---

## Summary

v0.7 is a two-phase release train:

- **v0.7.0 (Foundation)** hardens ADL’s runtime for deterministic, replayable multi-agent execution with security/trust boundaries and paper-driven delegation patterns.
- **v0.7.x minors (Learning train)** add overlay-based, opt-in learning features (observe → score → suggest → apply → export) without depending on ObsMem (moved to v0.75 planning).

Core principle: **no silent drift**. Adaptive behavior must be opt-in, artifacted, auditable, and reversible.

---

## Highlights

v0.7 has shipped (tag: `v0.7.0`).

---

## v0.7.0 — Foundation Release

### What’s New

The following areas were delivered as part of the v0.7 release train:
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

- v0.7.0 is tagged and released.
- Upgrade notes include CLI/env var migration details for the runtime identity compatibility window.

### Known Limitations

- Learning loop features are **not part of v0.7.0** (they land in v0.7.x minors).
- ObsMem integration is deferred to **v0.75 planning**.

---

## v0.7.x — Learning Train (Minors)

### What’s New

Learning-train scope is recorded here as release-train history.

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
- Learning surfaces remain independent of ObsMem (ObsMem is v0.75 planning).

---

## Late v0.7 — Runtime Identity Migration (Do Last)

Status: shipped during late v0.7 release-train execution.

- WP-12 / EPIC-H (#336 / #479) renames runtime identity late in v0.7:
  - Canonical crate/package/lib identity becomes `adl`
  - Canonical binaries are `adl` and `adl-remote`
  - Keep the `swarm/` directory path stable in v0.7
  - One-release compatibility window:
    - legacy `swarm` / `swarm-remote` entrypoints remain as shims with deprecation warnings
    - legacy `SWARM_...` env vars continue to work with deprecation warnings; canonical env vars become `ADL_...`

---

## Validation Notes

At release time (WP-16), the release was required to be supported by:
- `cargo fmt --all` (pass)
- `cargo clippy --all-targets -- -D warnings` (pass)
- `cargo test` (pass)
- CI green on merge target
- Demo matrix and runnable demos validated (WP-13 #474)
- Coverage/quality gate completed with documented exclusions (WP-14 #475)

---

## What’s Next (v0.75/v0.8/v0.85+ preview)

These are explicitly out of scope for v0.7:
- v0.75: ObsMem v1 integration and deterministic substrate follow-ons (EPIC-A + EPIC-B)
- v0.8: Gödel + authoring work (EPIC-C + EPIC-D)
- v0.85/v0.9: Distributed/cluster execution (#339) and durable checkpoint / recovery engine (#340)

---

## Exit Criteria

- Release notes reflect **only shipped behavior** at tag time.
- Planned items remain clearly marked until shipped.
- Final text is ready to paste into GitHub Release UI without further editing.
