# Public Prompt Records v0.91.5

## Metadata

- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-05-29`
- Owner: ADL maintainers
- Status: `active_wp_01_opening`
- Related issues: `#3472`, `#3473`, `#3474`, `#3475`, `#3476`

## Template Rules

This is a planning feature doc, not a publication approval.

## Purpose

Define the transition from local C-SDLC prompt records to public, reviewable,
redaction-safe prompt packets.

## Context

Prompt cards are durable C-SDLC state. Local `.adl` state also contains
execution cache and historical working files. v0.91.5 must separate public
records from local cruft before cleanup.

## Coverage / Ownership

This feature owns prompt packet export, redaction, validation, reviewer index,
and `.adl` archive/deletion review expectations.

## Overview

The public prompt-record lane should export selected prompt packets, validate
machine-readable shape, redact local/private data, and index them for review.

## Design

- Export prompt packets from tracked or approved local sources.
- Validate structure and unresolved placeholders.
- Run redaction checks before publication.
- Inventory `.adl` state before archive or deletion.
- Require review before destructive cleanup.

## Execution Flow

1. Define/export prompt packets.
2. Inventory `.adl` local state.
3. Pilot packets and reviewer index.
4. Add validation/redaction gates.
5. Close umbrella with disposition truth.

## Determinism and Constraints

Public packet generation must not depend on host paths or hidden local state.

## Integration Points

- [../WBS_v0.91.5.md](../WBS_v0.91.5.md)
- [../MILESTONE_CHECKLIST_v0.91.5.md](../MILESTONE_CHECKLIST_v0.91.5.md)

## Validation

Validation should include prompt packet structure, redaction scan, link checks,
and archive/deletion review checklist.

## Acceptance Criteria

- Public prompt packets are exportable and reviewable.
- Redaction and validation gates exist.
- `.adl` cleanup has review-before-delete disposition.

## Risks

- Local records may contain private or machine-specific content.
- Over-cleanup could lose useful historical evidence.

## Future Work

Future milestones can ingest archived records into ObsMem or publish curated
prompt corpora.

## Notes

This feature does not require all `.adl` history to become tracked repo state.
