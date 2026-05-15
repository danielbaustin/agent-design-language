# Google Workspace CMS Bridge

## Metadata

- Feature Name: Google Workspace CMS Bridge
- Milestone Target: `v0.91.2`
- Status: in_flight
- Planned WP Home: WP-08, WP-09
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

Out of scope:

- Workspace as canonical source of truth.
- Silent repo edits from Drive state.
- Live secrets in fixture validation.
- Rust-native adapter feasibility and typed boundary.

## Acceptance Criteria

- Demo stops before silent canonical repo edits.
- Git/Workspace revision mismatch is recorded as a first-class risk.
- Adapter boundary preserves ADL tool/ACC authority semantics.

## Proving Surface

- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_cms_snapshot.json`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_cms_bridge_demo_packet.md`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_promotion_packet.md`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_revision_mismatch_and_authority_rules.md`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_management_report.md`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_demo_manifest.json`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_tool_capability_trace.json`

## Non-Claims

- `WP-08` does not claim Google Workspace is canonical repo truth.
- `WP-08` does not claim live `gws` execution is required for fixture proof.
- `WP-08` does not authorize direct tracked repo edits from Workspace state.
