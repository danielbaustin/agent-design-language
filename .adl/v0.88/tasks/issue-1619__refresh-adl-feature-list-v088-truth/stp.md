---
issue_card_schema: adl.issue.v1
wp: "unassigned"
slug: "refresh-adl-feature-list-v088-truth"
title: "[v0.88][docs] Update ADL feature list for reconciled milestone truth"
labels:
  - "track:roadmap"
  - "area:docs"
  - "type:docs"
  - "version:v0.88"
issue_number: 1619
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Pending sprint assignment"
required_outcome_type:
  - "docs"
repo_inputs:
  - "docs/planning/ADL_FEATURE_LIST.md"
  - "docs/milestones/v0.88/README.md"
  - "docs/milestones/v0.88/FEATURE_DOCS_v0.88.md"
  - "docs/milestones/v0.88/WBS_v0.88.md"
  - "docs/milestones/v0.88/DESIGN_v0.88.md"
canonical_files:
  - "docs/planning/ADL_FEATURE_LIST.md"
demo_required: false
demo_names: []
issue_graph_notes:
  - "The root feature list was edited directly on main during the v0.88 planning reconciliation and must be re-landed through the normal issue/worktree/PR flow."
  - "This issue should align the feature list with the tracked v0.88 canonical docs without widening milestone scope."
pr_start:
  enabled: false
  slug: "refresh-adl-feature-list-v088-truth"
---

# [v0.88][docs] Update ADL feature list for reconciled milestone truth

## Summary

Refresh `docs/planning/ADL_FEATURE_LIST.md` so its current-status framing and `v0.88` description match the reconciled tracked milestone package. This is a narrow docs fix to re-land useful content through the proper issue/worktree/PR lifecycle after it was accidentally edited directly on `main`.

## Goal

Make the root feature list truthful about the current milestone story:
- `v0.87` remains the most recently completed milestone
- `v0.87.1` is the active release-tail completion band
- `v0.88` is the next major planned milestone
- the `v0.88` feature band now centers on chronosense, execution posture/cost reviewability, PHI metrics, instinct, bounded agency, and the bounded `Paper Sonata` flagship demo

## Required Outcome

- `docs/planning/ADL_FEATURE_LIST.md` is updated to reflect the reconciled `v0.88` package
- the wording aligns with the tracked `v0.88` canonical docs rather than the older persistence/aptitude framing
- the change lands through the normal issue/worktree/PR flow instead of remaining as `main` drift

## Deliverables

- updated `docs/planning/ADL_FEATURE_LIST.md`
- issue/worktree/PR lifecycle record showing the docs fix landed through the tracked flow

## Acceptance Criteria

- the feature-list current-status section reflects `v0.87`, `v0.87.1`, and `v0.88` truthfully
- the feature matrix row for `v0.88` matches the reconciled milestone package
- the `v0.88` milestone description no longer centers aptitude as a core `v0.88` commitment
- the feature-list wording stays aligned with tracked `v0.88` canonical docs and does not widen roadmap scope
- the main checkout is left clean after the fix is re-landed properly

## Repo Inputs

- https://github.com/danielbaustin/agent-design-language/issues/1619
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/milestones/v0.88/README.md`
- `docs/milestones/v0.88/FEATURE_DOCS_v0.88.md`
- `docs/milestones/v0.88/WBS_v0.88.md`
- `docs/milestones/v0.88/DESIGN_v0.88.md`

## Dependencies

- none recorded yet

## Demo Expectations

- No standalone demo is required. The proof surface is the docs diff staying tightly aligned with the tracked `v0.88` milestone package.

## Non-goals

- changing the `v0.88` milestone package itself
- broad roadmap reshaping beyond the feature-list wording
- pulling aptitude back into `v0.88` after it was intentionally moved later

## Issue-Graph Notes

- This issue exists because the useful docs change first appeared as accidental `main` drift and needs to be re-landed properly.
- Keep this fix narrow and truthful; it supports the reconciled `v0.88` package but does not replace milestone planning or execution issues.

## Notes

- The tracked `v0.88` package is already reconciled in the milestone docs and PR history; this issue only fixes the root feature inventory to match.

## Tooling Notes

- Follow the current lifecycle: doctor -> `pr run` -> docs edit in the bound worktree -> `pr finish`.
- Do not leave the feature-list update as untracked `main` drift.
