---
issue_card_schema: adl.issue.v1
wp: "unassigned"
slug: "hide-compromised-issues-1609-1626-and-preserve-only-approved-survivors"
title: "[v0.87.1][security] Hide compromised issues 1609-1626 and preserve only approved survivors"
labels:
  - "track:roadmap"
  - "type:task"
  - "area:security"
  - "area:docs"
  - "version:v0.87.1"
issue_number: 1634
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
  - "Security/process issue to hide compromised backlog/planning issues while preserving approved survivors and durable planning truth."
pr_start:
  enabled: false
  slug: "hide-compromised-issues-1609-1626-and-preserve-only-approved-survivors"
---

## Summary

Hide compromised issues `#1609` through `#1626`, preserve only the approved survivor issues `#1614` and `#1618`, and record the disposition truthfully so project planning remains auditable.

## Goal

Contain the compromised issue surface without losing the parts of planning that still matter, and do it through one explicit tracked process issue rather than ad hoc deletions.

## Required Outcome

- define the exact hide/delete scope for `#1609` through `#1626`
- preserve `#1614` and `#1618` as the only surviving issues from that range
- record where any still-needed planning truth lives after the hide/delete action
- scrub tracked/public-facing references to deleted issue numbers where those references would continue to advertise the removed planning surface
- leave a truthful audit trail for why the issues were hidden and what was intentionally preserved

## Deliverables

- a complete disposition list for issues `#1609` through `#1626`
- explicit confirmation that only `#1614` and `#1618` remain open in that range
- updated local planning/process notes pointing to preserved docs or successor issues where needed
- updated tracked/public docs where deleted issue-number references would otherwise remain visible
- a truthful closeout note describing the security-driven removal decision

## Acceptance Criteria

- every issue from `#1609` through `#1626` has an explicit disposition recorded in this issue
- `#1614` and `#1618` are explicitly marked as the only approved survivors in that range
- the process does not silently drop still-needed planning truth; preserved content is mapped to docs or successor issues where needed
- the hide/delete action is described truthfully as security-driven containment
- tracked/public-facing references to deleted issue numbers are removed or replaced where practical
- the resulting issue surface is simpler and no longer exposes the compromised range beyond the approved survivors

## Repo Inputs

- `https://github.com/danielbaustin/agent-design-language/issues/1609`
- `https://github.com/danielbaustin/agent-design-language/issues/1614`
- `https://github.com/danielbaustin/agent-design-language/issues/1618`
- `https://github.com/danielbaustin/agent-design-language/issues/1626`
- `.adl/docs/TBD/MEDIUM_ARTICLE_SERIES_PLAN.md`
- `.adl/docs/TBD/REVIEW_KNOWLEDGE_SURFACE_PLAN.md`
- `.adl/docs/v0.88planning/PAPER_SONATA_IMPLEMENTATION_PLAN.md`
- `docs/milestones/v0.88/README.md`
- `docs/milestones/v0.88/FEATURE_DOCS_v0.88.md`
- `docs/milestones/v0.88/WBS_v0.88.md`
- `docs/milestones/v0.88/DECISIONS_v0.88.md`
- `docs/milestones/v0.88/RELEASE_NOTES_v0.88.md`

## Dependencies

- preserve survivor truth for `#1614` and `#1618`
- preserve any non-issue planning truth that must remain after the compromised issue range is hidden

## Demo Expectations

- No standalone demo required. Proof is the recorded disposition set and the resulting visible issue surface.

## Non-goals

- broad milestone replanning beyond the compromised range
- deleting planning docs that are still needed as durable local truth
- treating this as a product-feature issue instead of a security/process containment issue

## Issue-Graph Notes

- This is the authoritative process issue for hiding compromised issues `#1609` through `#1626`.
- Only `#1614` and `#1618` should remain from that range after this issue is completed.
- Public milestone docs should not continue advertising deleted issue numbers after the hide/delete action completes.

## Notes

- Keep the process truthful and minimal. The value here is containment plus preserved planning truth, not more workflow ceremony.

## Tooling Notes

- Keep GitHub issue metadata, local source prompt, and task cards aligned.
