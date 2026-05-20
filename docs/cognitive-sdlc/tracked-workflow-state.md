# Tracked Workflow State

## Status

Tracked C-SDLC workflow-state policy summary.

The milestone-specific migration plan is
[`docs/milestones/v0.91.4/C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md`](../milestones/v0.91.4/C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md).

## Decision

Durable C-SDLC truth must be tracked in Git.

The local `.adl/` workflow surface was useful as a startup shortcut, but it is
not a sufficient final home for governed workflow records. If an artifact is
needed for governance, review, closeout, release evidence, signed trace proof,
or ObsMem ingestion, it should not remain local-only.

## Canonical Layers

### Git And GitHub

GitHub issues and pull requests remain external coordination, review, CI, and
merge surfaces.

Tracked repo files remain canonical source, docs, durable workflow records, and
evidence surfaces.

### Durable Workflow Records

Durable records should live under:

```text
workflow/c-sdlc/<version>/
```

The planned shape is:

```text
workflow/c-sdlc/<version>/
  issues/
  sprints/
  evidence/
  traces/
```

### Local `.adl/`

Local `.adl/` may remain for:

- execution cache
- local staging
- temporary helper files
- machine-local scratch
- ignored bootstrap compatibility surfaces

It must not be the only authoritative record for durable C-SDLC truth after
the default-operation cutover.

### External Collaboration

Google Workspace or other collaboration systems may be used for scratch,
drafting, or staging. They are optional and non-canonical unless their outputs
are promoted into Git/GitHub.

GWS is not part of C-SDLC.

## Track First

The first durable surfaces to track are:

- `SIP`, `STP`, `SPP`, `SRP`, and `SOR`
- sprint state and sprint closeout
- evidence bundles
- review findings and dispositions
- SOR closeout truth
- signed trace bundles and verification results
- ObsMem ingestion records
- milestone and release evidence

## Completion Bar

C-SDLC cannot be the default ADL software-development path while durable
workflow truth remains local-only.

