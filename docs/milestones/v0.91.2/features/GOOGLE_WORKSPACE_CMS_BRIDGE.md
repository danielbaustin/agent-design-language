# Google Workspace CMS Bridge

## Metadata

- Feature Name: Google Workspace CMS Bridge
- Milestone Target: `v0.91.2`
- Status: implemented baseline plus active hardening
- Planned WP Home: WP-08, WP-09, #3091, #3092, #3093, #3094
- Source Docs:
  - `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_cms_bridge_demo_packet.md`
  - `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/workspace_revision_mismatch_and_authority_rules.md`
  - `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/gws_live_safety_runbook.md`
  - `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/codefriend_gws_operational_package.md`
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
- Project-ready operational package for reusing the bridge on future CodeFriend/ADL projects.

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
- A future project can adopt the bridge without guessing about setup, scope,
  GitHub boundaries, or proof-packet expectations.

## Current State

The bounded GWS bridge is now real in `v0.91.2`:

- `WP-08` landed the bounded Workspace CMS demo packet and canonical-repo
  boundary rules.
- `WP-09` landed the typed Rust-native adapter boundary for fixture-backed
  inventory, snapshot, preview, promotion, and bounded apply flows.
- `#3092` landed the live safety package, auth/scope runbook, and safe-defaults
  operator posture.
- `#3091` landed the bounded live Drive/Doc/Sheet execution slice.
- `#3093` landed the bounded content-card roundtrip contract with revision-anchor
  enforcement and promotion handoff discipline.
- `#3094` landed the reusable operational package for future CodeFriend/ADL
  projects.

What is now proven:

- Google Workspace can be used as bounded draft/review/content-card
  infrastructure.
- GitHub remains canonical truth for tracked repository state.
- Live bounded reads and bounded content-card mutation flows can be expressed
  with explicit auth, scope, revision, and stop-boundary semantics.
- A future project can onboard to this bridge with stable setup and proof
  surfaces.

What is still intentionally not claimed:

- Google Workspace is not canonical repo truth.
- Broad ambient Workspace authority is not allowed.
- Silent sync from Workspace into tracked repository files is not allowed.
- This milestone does not claim general enterprise Workspace administration.

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
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/codefriend_gws_operational_package.md`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/gws_project_setup_and_onboarding.md`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/gws_safe_defaults_and_scope_checklist.md`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/gws_project_workflow_template.md`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/codefriend_gws_git_workspace_boundary_runbook.md`
- `docs/milestones/v0.91.2/review/google_workspace_cms_bridge/gws_reusable_proof_packet_template.md`

## Migration Plan: `.adl` To GWS

The intended migration is not "move ADL into Workspace." It is "move selected
draft/review collaboration loops into bounded Workspace surfaces while keeping
GitHub and tracked repo files canonical."

Phase 1: Local `.adl` remains canonical for issue execution records.

- `STP`, `SIP`, `SPP`, `SRP`, and `SOR` remain local workflow records under
  `.adl/`.
- Sprint state, closeout truth, and task-bundle cards stay local unless and
  until ADL explicitly defines a tracked publication surface for them.
- Workspace is optional in this phase and should not be required for normal
  issue execution.

Phase 2: Workspace becomes the bounded collaboration layer.

- Use GWS Docs/Sheets as draft-review/content-card working space.
- Use the typed adapter and live safety package to inventory, snapshot,
  preview, and roundtrip bounded content-card updates.
- Require explicit scope binding, explicit live mode, and explicit write
  approval for live mutation.

Phase 3: Promotion back to canonical truth.

- Workspace outputs become promotion inputs, not canonical repository state.
- GitHub issue/PR flow remains the promotion boundary for tracked docs/code.
- Revision-anchor and doc-binding checks must pass before bounded live write
  flows continue.
- Promotion packets and proof artifacts should be captured under
  `docs/milestones/.../review/google_workspace_cms_bridge/` or a future
  project-local tracked proof home.

Phase 4: Routine project use.

- New CodeFriend/ADL projects should start from the operational package and
  safe-defaults checklist rather than rebuilding the bridge ad hoc.
- Use dry-run first, then bounded execute mode only when auth, scope, and write
  approval are explicit.
- Keep `.adl` for local lifecycle truth and GitHub for canonical tracked truth;
  GWS fills the collaboration gap between them.

This means the migration target is a three-layer model:

1. `.adl` for local workflow truth
2. GWS for bounded draft/review/content-card collaboration
3. GitHub + tracked repo files for canonical promoted truth

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
- `#3094` does not claim broad enterprise Google Workspace operations or
  canonical Git replacement.
- The tracked `#3093` artifact may remain dry-run when live auth or scopes are
  unavailable; that still proves the bounded command and stop-boundary
  contract, not successful live mutation.
- The migration plan does not claim `.adl` issue cards are disappearing in
  `v0.91.2`; it defines how GWS fits around them without replacing them.
