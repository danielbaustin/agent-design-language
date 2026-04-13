---
issue_card_schema: adl.issue.v1
wp: "unassigned"
slug: "v0-88-tools-generate-wp-issue-waves-from-canonical-wbs-surfaces"
title: "[v0.88][tools] Generate WP issue waves from canonical WBS surfaces"
labels:
  - "track:roadmap"
  - "type:task"
  - "area:tools"
  - "version:v0.88"
issue_number: 1667
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Pending sprint assignment"
required_outcome_type:
  - "code"
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Mirrored from the authored GitHub issue body during bootstrap/init."
pr_start:
  enabled: false
  slug: "v0-88-tools-generate-wp-issue-waves-from-canonical-wbs-surfaces"
---

## Summary

Create a deterministic WBS-to-issue-wave generator so milestone work packages can be seeded from canonical planning docs without manual re-entry.

## Goal

Generate the main milestone issue wave from the tracked WBS/sprint package so issue creation becomes a reproducible control-plane action instead of a hand-built pass.

## Required Outcome

- the control plane can derive a milestone issue wave from canonical planning inputs
- generated issues preserve WP ordering, titles, dependency notes, and version/label metadata
- the generator stops at readiness/bootstrap rather than silently executing work

## Deliverables

- WBS-to-issue-wave generation surface
- tests covering deterministic generation and metadata parity
- docs describing the generation contract

## Acceptance Criteria

- a canonical milestone package can produce the expected WP issue wave without hand-copying each issue definition
- the output is deterministic for identical planning inputs
- generated issues still stop before branch/worktree creation

## Repo Inputs

- docs/milestones/v0.88/WBS_v0.88.md
- docs/milestones/v0.88/SPRINT_v0.88.md
- .adl/docs/TBD/V0_88_WP_READINESS_QUEUE.md

## Dependencies

- none

## Demo Expectations

- no demo required

## Non-goals

- executing generated issues
- inventing milestone structure outside the tracked package

## Issue-Graph Notes

- child of #1665
- this issue is the direct control-plane follow-on to the manual v0.88 readiness-wave pass

## Notes

- prefer canonical planning inputs over freeform inference

## Tooling Notes

- bootstrap only in this pass; no execution context creation

