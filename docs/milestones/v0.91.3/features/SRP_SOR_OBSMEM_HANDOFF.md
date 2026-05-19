# SRP/SOR ObsMem Handoff

## Status

Planned `v0.91.3` feature.

## Purpose

Define how C-SDLC review results and outcome truth become memory input without
turning unreviewed local notes into canonical knowledge.

The first slice should show the boundary: `SRP` contains review instructions,
findings, dispositions, and residual risk; `SOR` contains actual outcome truth.
Together they provide a memory handoff shape that `v0.91.4` can harden into
tracked ingestion.

## Scope

The first slice must define:

- which `SRP` fields are eligible for memory handoff
- which `SOR` fields are eligible for memory handoff
- how the handoff references issue, PR, branch, evidence, and review truth
- how skipped, failed, or deferred states are represented
- what remains outside memory until verified

## Acceptance Criteria

- The first proof emits a memory handoff fixture or tracked handoff record.
- The handoff is derived from final `SRP` and `SOR` truth, not stale draft
  cards.
- The handoff distinguishes facts, review judgments, residual risks, and
  follow-on work.
- The handoff is repo-relative and ready for `v0.91.4` tracked ObsMem
  ingestion.

## Non-Goals

- This feature does not complete full ObsMem write/read integration.
- This feature does not ingest untracked local artifacts as canonical memory.
- This feature does not treat unresolved review findings as completed truth.
