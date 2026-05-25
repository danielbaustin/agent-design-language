# SRP/SOR ObsMem Handoff

## Status

Proven `v0.91.3` feature under `WP-07` / `#3205`.

## Purpose

Define how C-SDLC review results and outcome truth become memory input without
turning unreviewed local notes into canonical knowledge.

The first slice shows the boundary: `SRP` contains review instructions,
findings, dispositions, and residual risk; `SOR` contains actual outcome truth.
Together they provide a memory handoff shape that `v0.91.4` can harden into
tracked ingestion.

For `v0.91.3`, exact reviewer-facing source provenance is now anchored to the
tracked `WP-05` card bundle under
`docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3203-evidence-bundle-proof/`, while
supporting evidence and merge-readiness artifacts remain companion citations.
The promoted snapshots may still preserve bounded local derivation references,
so this feature proves durable source anchoring rather than a fully standalone
tracked-workflow migration.

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

## Proof Surface

Tracked first-slice proof:

- `docs/milestones/v0.91.3/review/obsmem_handoff/ct_demo_001_obsmem_handoff.json`
- `docs/milestones/v0.91.3/review/obsmem_handoff/ct_demo_001_obsmem_handoff.md`
- `docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3203-evidence-bundle-proof/cards/srp.md`
- `docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3203-evidence-bundle-proof/cards/sor.md`
- `adl/tools/validate_obsmem_handoff_packet.py`
- `adl/tools/test_obsmem_handoff_packet.sh`

This is a tracked handoff contract, not a live ObsMem backend integration.

## Non-Goals

- This feature does not complete full ObsMem write/read integration.
- This feature does not ingest untracked local artifacts as canonical memory.
- This feature does not treat unresolved review findings as completed truth.
