# v0.90 Repo Visibility Prototype

## Status

WP-12 prototype for `#2031`.

## Purpose

This directory is the bounded repo-visibility proof packet for `v0.90`.

The goal is not to build a full semantic repo index. The goal is to prove that
ADL can make one milestone slice reviewer-visible by naming the authoritative
docs, implementation surfaces, tests, demos, review surfaces, issue records,
planning boundaries, and known gaps in one stable place.

## Prototype Slice

This first pass maps the `v0.90` long-lived runtime slice because it is the
milestone's central implementation thesis and it already has enough tracked
evidence to be useful:

- supervisor and heartbeat
- cycle artifact contract
- state and continuity handles
- operator control and safety
- stock-league proof path

## Files

- `CANONICAL_SOURCE_MANIFEST_v0.90.yaml`: machine-readable source manifest for
  the selected slice.
- `CODE_DOC_DEMO_LINKAGE_REPORT_v0.90.md`: reviewer-readable linkage report
  connecting docs, code, tests, demos, review surfaces, issues, and gaps.

## Truth Boundaries

- Tracked milestone docs are the public source of truth for the milestone.
- Local `.adl` planning notes are not public release truth.
- Ideas-lane docs provide background or later-band context, not shipped
  implementation claims.
- Retired material is historical provenance only.
- Missing links are recorded as gaps, not inferred from aspiration.
