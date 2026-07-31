# ADR 0056: C-SDLC v2 Sole Lifecycle Authority

- Status: Accepted
- Date: 2026-07-30
- Accepted in: v0.91.8
- Related issues: #5358, #5541
- Related ADRs: ADR 0024, ADR 0028, ADR 0029, ADR 0033, ADR 0037, ADR 0044, ADR 0046
- Supersedes: ADR 0029 for operational generation authority
- Source evidence:
  - `docs/architecture/CSDLC_V2_CLEAN_ROOM_ARCHITECTURE.md`
  - `docs/architecture/csdlc-v2/gate10d2/DESIGN.md`
  - `csdlc-v2/operator/generation-selector.json`
  - `csdlc-v2/operator/coexistence.json`
  - `csdlc-v2/operator/SKILLS.md`
  - `docs/milestones/v0.91.8/features/CSDLC_V2_ACCEPTANCE_v0.91.8.md`
  - merge commit `fc75f4fc6`

## Context

The prior C-SDLC surface mixed shell wrappers, prompt wrappers, lifecycle state
mutation, GitHub transport, and compatibility paths. Multiple command
authorities created drift, rebuild cost, and ambiguous recovery behavior.

v0.91.8 completed the independent Rust v2 lifecycle and the reviewed v1
command-surface sunset.

## Decision

C-SDLC v2 is the sole operational authority for C-SDLC lifecycle work.

- `csdlc-install resolve` reads the tracked generation selector.
- Typed Rust binaries own initialization, binding, editing, validation, review,
  publication, GitHub state, merge, recovery, and closeout.
- Operator skills delegate to typed argv contracts and do not mutate Markdown
  or lifecycle state directly.
- Stable binaries are installed under `.adl/bin/csdlc-v2/`.
- The `v1_sunset` inventory forbids restoration of retired v1 command surfaces.
- Session ownership and protected-path collision handling remain lifecycle
  invariants, not an alternate command authority.

## Consequences

- Lifecycle and GitHub behavior have one typed implementation boundary.
- Missing operations are v2 tooling defects, not reasons to revive wrappers.
- Exact-revision review and publication truth can be validated consistently.
- Recovery must remain possible without hand-editing tracked state.

## Alternatives Considered

### Keep v1 wrappers as a fallback

Rejected. Parallel authorities recreate drift and make failures host-dependent.

### Let skills edit lifecycle files directly

Rejected. Skills are routing contracts, not state engines.

## Validation Notes

Validate selector resolution, forbidden-v1 inventory, stable binary
provenance, all typed lifecycle transitions, GitHub issue/PR operations,
conflict recovery, exact-head review, merge, and terminal reconciliation.

## Non-Claims

- This ADR does not make every lifecycle gate necessary forever.
- This ADR does not permit manual state edits when a typed operation is absent.

