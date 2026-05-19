# Google Workspace CMS Bridge

## Metadata

- Feature Name: Google Workspace CMS Bridge
- Milestone Target: `v0.91.2`
- Status: in_flight
- Planned WP Home: WP-08, WP-09, #3091, #3092, #3093
- Source Docs:
  - `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_cms_bridge_demo_packet.md`
  - `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_revision_mismatch_and_authority_rules.md`
  - `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/gws_live_safety_runbook.md`
  - `.adl/docs/TBD/google_workspace_cms/GWS_TOOLING_DEPENDENCY_AND_SEQUENCING.md`
  - `.adl/docs/TBD/google_workspace_cms/RUST_NATIVE_GWS_ADAPTER_PLAN.md`
- Proof Modes: fixture demo, adapter boundary, live bounded execution, review

## Purpose

Build a bounded bridge for draft planning/review docs, comments, content cards,
and promotion packets while preserving Git-backed canonical repo truth.

## Scope

In scope:

- Workspace content-card and promotion-packet demo.
- Fixture mode and live-gated mode boundary.
- Revision mismatch handling.
- Rust-native fixture-first CMS capability surface for inventory, snapshot,
  promotion, preview, and bounded apply flows.
- Live bounded `gws` execution surface for one explicit folder/doc/sheet scope.
- Live bounded content-card preview/apply roundtrip contract with revision-anchor match enforcement and promotion-packet handoff.

Out of scope:

- Workspace as canonical source of truth.
- Silent repo edits from Drive state.
- Live secrets in fixture validation.
- Ambient live Workspace authority.

## Acceptance Criteria

- Demo stops before silent canonical repo edits.
- Git/Workspace revision mismatch is recorded as a first-class risk.
- Adapter boundary preserves ADL tool/ACC authority semantics.
- Live `gws` execution classifies missing auth, missing scope, or unavailable
  tooling as skipped rather than silently failing.
- Live content-card mutation stops before apply when the recorded revision
  anchor no longer matches the bounded Workspace source or when the bounded
  document/content-card binding is inconsistent.

## Proving Surface

- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_cms_snapshot.json`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_cms_bridge_demo_packet.md`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_promotion_packet.md`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_revision_mismatch_and_authority_rules.md`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_management_report.md`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_demo_manifest.json`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_tool_capability_trace.json`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/rust_native_gws_adapter_boundary_report.json`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/gws_live_safety_package_report.json`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/gws_live_safety_runbook.md`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/gws_live_capability_execution_report.json`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/gws_live_capability_execution_snapshot.json`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/gws_live_content_card_roundtrip_report.json`

## Non-Claims

- `WP-08` does not claim Google Workspace is canonical repo truth.
- `WP-08` does not claim live `gws` execution is required for fixture proof.
- `WP-08` does not authorize direct tracked repo edits from Workspace state.
- `WP-09` does not claim live authenticated Workspace writes are enabled by
  default.
- `#3091` does not claim live Workspace writes or broad ambient account access.
- The live-safety package does not authorize ambient broad Workspace authority.
- `#3093` does not authorize silent content promotion into tracked repository
  files.
- The tracked `#3093` artifact may remain dry-run when live auth or scopes are
  unavailable; that still proves the bounded command and stop-boundary
  contract, not successful live mutation.
