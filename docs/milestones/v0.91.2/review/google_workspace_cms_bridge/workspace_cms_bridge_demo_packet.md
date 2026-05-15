# Google Workspace CMS Bridge Demo Packet

## Purpose

Demonstrate one bounded Workspace CMS flow for ADL where draft planning and
review material can be inventoried, classified, and prepared for promotion
without turning Google Workspace into canonical repository truth.

## Inputs

- `.adl/docs/TBD/google_workspace_cms/GWS_CMS_BRIDGE_DEMO_PLAN.md`
- `.adl/docs/TBD/google_workspace_cms/GWS_TOOLING_DEPENDENCY_AND_SEQUENCING.md`
- `.adl/docs/TBD/google_workspace_cms/GOOGLE_DRIVE_DOCS_BRIDGE_ANALYSIS.md`
- `docs/milestones/v0.91.2/features/GOOGLE_WORKSPACE_CMS_BRIDGE.md`

## Fixture Outputs

- `workspace_cms_snapshot.json`
- `workspace_promotion_packet.md`
- `workspace_revision_mismatch_and_authority_rules.md`
- `workspace_management_report.md`
- `workspace_demo_manifest.json`
- `workspace_tool_capability_trace.json`

## Demo Flow

1. Load one bounded Workspace CMS snapshot for a single Drive folder.
2. Classify each content card by lifecycle state.
3. Generate a promotion packet for any card marked
   `ready_for_repo_promotion`.
4. Stop when a card is blocked by revision mismatch, post-start planning drift,
   or missing issue-backed promotion authority.
5. Record live-mode capabilities as gated proposals, not default execution.

## What The Demo Proves

- Workspace content cards can manage document lifecycle without replacing ADL
  issue lifecycle cards.
- Promotion into tracked docs is explicit and issue-backed.
- Revision mismatch is recorded as first-class review state, not hidden behind
  sync language.
- Live Workspace actions remain governed capability proposals, not silent repo
  mutation.

## What The Demo Does Not Prove

- bidirectional sync between Workspace and Git
- canonical repo edits directly from Workspace state
- safe live execution without governed-tool boundaries
- replacement of GitHub issue, PR, or milestone truth with Workspace metadata
