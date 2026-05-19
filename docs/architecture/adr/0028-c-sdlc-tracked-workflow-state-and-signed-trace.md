# ADR 0028 Candidate: C-SDLC Tracked Workflow State And Signed Trace Boundary

- Status: Candidate
- Target milestone: v0.91.2 planning, v0.91.3/v0.91.4 implementation path
- Related issues: #3099, #3100, #3120, #3124
- Related ADRs: ADR 0018, ADR 0024 after acceptance

## Context

C-SDLC is a general model for governed software development with cognitive
agents. ADL will implement it first for its own development workflow, but the
idea is not ADL-only: other projects can implement C-SDLC with their own tools
if they preserve the same audit and lifecycle principles.

The current ADL workflow exposes the problem C-SDLC is meant to solve. Durable
lifecycle truth has often lived in ignored local `.adl` records. That startup
shortcut helped the project move quickly while the workflow was immature, but
it now creates repeated truth drift and makes governed software development
harder to audit.

C-SDLC is intended to remodel software development for agents. If it is to be
credible, its lifecycle records must be public, tracked, inspectable,
auditable, and useful as memory. The system cannot depend on "the real state
was on one machine."

For ADL's implementation, C-SDLC has one canonical direction for durable
workflow truth: tracked Git state plus signed trace evidence and ObsMem
ingestion. Other implementations may choose different storage mechanics, but
must preserve public inspectability, auditability, and durable evidence.

## Decision

ADL should implement C-SDLC by making durable workflow truth tracked in Git by
the end of v0.91.4.

Durable C-SDLC records include issue cards, sprint state, closeout artifacts,
review findings, proof packets, promoted traces, signed trace bundles, and
ObsMem ingestion surfaces derived from tracked evidence.

Local `.adl` state should shrink toward ephemeral execution cache and temporary
helper files. Durable C-SDLC truth should live in tracked Git state, not in
ignored local-only records.

Signed trace proof should not be deferred until full trace-query work. ADL's
C-SDLC implementation needs a minimal signed trace slice before it becomes
default operation.

## Requirements

- Track all durable workflow records needed for governance, review, closeout,
  release evidence, or ObsMem ingestion in ADL's implementation.
- Keep genuinely ephemeral caches, secrets, temporary helper files, and
  non-durable scratch untracked.
- Preserve the canonical lifecycle `SIP -> STP -> SPP -> SRP -> SOR`.
- Add a minimal signed trace bundle for durable C-SDLC runs:
  - trace event stream or trace manifest
  - digest manifest for trace and key proof artifacts
  - signature record
  - verification command and expected result
  - SOR/SRP/closeout linkage
  - ObsMem ingestion linkage
- Complete ADL's tracked workflow-state migration by the end of v0.91.4.

## Consequences

### Positive

- Makes ADL's C-SDLC implementation auditable by future agents and humans.
- Reduces truth drift between local cards, GitHub, PRs, and release evidence.
- Gives ObsMem higher-quality governed memory inputs.
- Makes signed trace part of the C-SDLC proof story early enough to matter.

### Negative

- Tracked workflow records will add PR noise and merge friction.
- Tooling must classify durable versus ephemeral artifacts carefully.
- The migration must be staged to avoid a giant historical cleanup burst.

## Migration Path

1. Define the tracked workflow home and artifact classification.
2. Track new durable C-SDLC records first.
3. Normalize the active milestone's open and recently closed workflow records.
4. Add the minimal signed trace slice.
5. Complete all durable C-SDLC tracking by the end of v0.91.4.
6. Backfill older history selectively by governance and release-evidence value.

## Alternatives Considered

### Keep `.adl` mostly ignored

This preserves low-noise local execution, but it perpetuates truth drift and
weak auditability.

### Wait for full trace query before signed traces

Full TQL can wait. A minimal signed trace proof slice cannot, because ADL's
C-SDLC governance needs durable signed evidence.

## Validation Notes

This candidate should be reviewed against:

- the C-SDLC tracked workflow state migration plan from 2026-05-19
- the workflow state home decision memo from 2026-05-19, as superseded for
  C-SDLC by the tracked-state direction
- root `AGENTS.md`
- the C-SDLC first-slice and completion planning issues
- signed trace architecture and existing signing ADRs

## Non-Claims

- This ADR does not implement the migration.
- This ADR does not claim C-SDLC can only be implemented by ADL.
- This ADR does not require full TQL before C-SDLC adoption.
- This ADR does not require tracking secrets, temporary caches, or scratch
  material that is not durable workflow truth.
