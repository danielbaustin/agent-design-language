# Workspace Promotion Packet Fixture

## Source Workspace Document

- Title: `CodeFriend Review Packet Draft`
- Workspace ref: `gws://drive/docs/doc-review-packet-demo`
- Revision anchor: `workspace-revision-42`
- Owning issue: `#3007`

## Promotion Target

- Target repo path:
  `docs/milestones/v0.91.2/review/codefriend_productization/review_packet_workflow_package.md`
- Promotion posture: issue-backed PR required
- Canonical authority after promotion: Git + PR review + merge history

## Promotion Rationale

This fixture document is eligible for promotion because it is already shaped as
durable milestone review content. The bridge demo proves that ADL can name the
source Workspace doc, preserve a revision anchor, and point at the canonical
repo target without writing that target directly from Workspace state.

## Required Checks Before Promotion

- confirm the Workspace source is still at the recorded revision anchor
- confirm the target repo path is correct and still in scope
- confirm any public milestone-plan edits after milestone start are tied to a
  GitHub issue and PR
- confirm unresolved comments or blockers are recorded before promotion

## Stop Boundary

Stop before editing tracked repository files directly from Workspace state.
Promotion remains a separate issue-scoped Git/PR action.
