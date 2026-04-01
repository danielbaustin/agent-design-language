---
issue_card_schema: adl.issue.v1
wp: "unassigned"
slug: "v0-85-docs-capture-road-to-v0-95-roadmap-restructure-edits-from-main"
title: "[v0.85][docs] Capture ROAD_TO_v0.95 roadmap restructure edits from main"
labels:
  - "track:roadmap"
  - "type:task"
  - "area:docs"
  - "version:v0.85"
issue_number: 1245
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Pending sprint assignment"
required_outcome_type:
  - "docs"
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Bootstrap-generated from GitHub issue metadata because no canonical local issue prompt existed yet."
pr_start:
  enabled: true
  slug: "v0-85-docs-capture-road-to-v0-95-roadmap-restructure-edits-from-main"
---

---
issue_card_schema: adl.issue.v1
wp: "DOCS"
slug: "v0-85-docs-capture-road-to-v0-95-roadmap-restructure-edits-from-main"
title: "[v0.85][docs] Capture ROAD_TO_v0.95 roadmap restructure edits from main"
labels:
  - "track:roadmap"
  - "type:task"
  - "area:docs"
  - "version:v0.85"
status: "active"
action: "edit"
milestone_sprint: "Planning follow-up"
required_outcome_type:
  - "docs"
repo_inputs:
  - ".adl/docs/roadmaps/ROAD_TO_v0.95.md"
canonical_files:
  - ".adl/docs/roadmaps/ROAD_TO_v0.95.md"
demo_required: false
demo_names: []
issue_graph_notes:
  - "Tracks a user-authored roadmap restructuring diff that was left loose on main."
pr_start:
  enabled: true
  slug: "v0-85-docs-capture-road-to-v0-95-roadmap-restructure-edits-from-main"
---

# [v0.85][docs] Capture ROAD_TO_v0.95 roadmap restructure edits from main

## Summary

Move the loose roadmap restructuring edits from the primary checkout into a tracked issue/worktree so the planning change is reviewable and `main` remains clean while roadmap editing continues. Also relocate `ROAD_TO_v0.95.md` out of `v0.85planning` into a cross-milestone roadmap location without losing any of the current edits.

## Goal

Capture the current `ROAD_TO_v0.95.md` changes on their own issue branch, preserve the in-progress content exactly, and move the document to a semantically correct non-`v0.85planning` home.

## Required Outcome

This issue is docs-only.

## Deliverables

- a tracked issue for the loose roadmap edit
- a started worktree carrying the current `ROAD_TO_v0.95.md` diff
- a clean primary checkout after the diff is moved
- a follow-through relocation plan and execution path for moving `ROAD_TO_v0.95.md` out of `.adl/docs/v0.85planning/`

## Acceptance Criteria

- the exact current diff from `ROAD_TO_v0.95.md` is preserved in the issue worktree and moved to `.adl/docs/roadmaps/ROAD_TO_v0.95.md`
- the primary checkout no longer has that loose roadmap modification
- the issue explicitly tracks relocation of the roadmap to a cross-milestone docs location, preserving all current edits
- no unrelated files are changed

## Repo Inputs

- `.adl/docs/roadmaps/ROAD_TO_v0.95.md`

## Dependencies

- none

## Demo Expectations

- no demo required

## Non-goals

- revising the roadmap content beyond preserving the existing edit and tracking the path relocation work
- editing unrelated docs or roadmap files

## Issue-Graph Notes

- This issue exists only to preserve and track the currently loose roadmap edit.
- The roadmap is misfiled under `v0.85planning`; this issue now also tracks relocating it to a cross-milestone roadmap location.

## Notes

- Keep the edit content exactly as-is while moving it off `main`.
- Preserve the current roadmap edits first; do not lose them while fixing the document location.

## Tooling Notes

- Use the normal `pr create -> pr init -> pr start` lifecycle.
