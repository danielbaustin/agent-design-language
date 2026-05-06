# Google Workspace CMS Bridge

## Metadata

- Feature Name: Google Workspace CMS Bridge
- Milestone Target: `v0.91.2`
- Status: planned
- Planned WP Home: WP-08 and WP-09
- Source Docs: `.adl/docs/TBD/google_workspace_cms/`
- Proof Modes: fixture demo, adapter boundary, review

## Purpose

Build a bounded bridge for draft planning/review docs, comments, content cards,
and promotion packets while preserving Git-backed canonical repo truth.

## Scope

In scope:

- Workspace content-card and promotion-packet demo.
- Fixture mode and live-gated mode boundary.
- Revision mismatch handling.
- Rust-native adapter feasibility and typed boundary.

Out of scope:

- Workspace as canonical source of truth.
- Silent repo edits from Drive state.
- Live secrets in fixture validation.

## Acceptance Criteria

- Demo stops before silent canonical repo edits.
- Git/Workspace revision mismatch is recorded as a first-class risk.
- Adapter boundary preserves ADL tool/ACC authority semantics.
