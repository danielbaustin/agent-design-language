---
issue_card_schema: adl.issue.v1
wp: "WP-25"
slug: "remove-duplicate-and-misplaced-planning-docs-after-roadmap-reorganization"
title: "[v0.85][docs] Remove duplicate and misplaced planning docs after roadmap reorganization"
labels:
  - "track:roadmap"
  - "version:v0.85"
  - "type:task"
  - "area:docs"
issue_number: 1079
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on:
  - "#1060"
milestone_sprint: "Sprint 4"
required_outcome_type:
  - "docs"
repo_inputs:
  - ".adl/docs/v0.86planning/"
  - ".adl/docs/v0.88planning/"
  - ".adl/docs/v0.89planning/"
  - ".adl/docs/v0.90planning/"
  - ".adl/docs/v0.91planning/"
  - ".adl/docs/v0.92planning/"
  - ".adl/docs/v0.93planning/"
  - ".adl/docs/v0.95planning/"
  - ".adl/docs/v0.85planning/ROAD_TO_v0.95.md"
canonical_files:
  - ".adl/docs/v0.88planning/INSTINCT_MODEL.md"
  - ".adl/docs/v0.88planning/PHI_METRICS_FOR_ADL.md"
  - ".adl/docs/v0.89planning/SECURITY_AND_THREAT_MODELING.md"
  - ".adl/docs/v0.90planning/TRACE_QUERY_LANGUAGE.md"
  - ".adl/docs/v0.91planning/AFFECT_MODEL_v0.90.md"
demo_required: false
demo_names: []
issue_graph_notes:
  - "This is a bounded post-#1060 cleanup issue."
  - "Remove duplicate planning docs but do not delete any unique-content doc."
  - "Keep exactly one roadmap-home copy for each affected planning document."
pr_start:
  enabled: true
  slug: "remove-duplicate-and-misplaced-planning-docs-after-roadmap-reorganization"
---

# Remove duplicate and misplaced planning docs after roadmap reorganization

## Summary

Clean up duplicate and misplaced planning docs under `.adl/docs/v0.86planning`
and above so the roadmap bands have one truthful copy of each active feature doc.

## Goal

Finish the roadmap reorganization cleanup by removing only confirmed duplicate
planning docs and keeping the surviving copy in the milestone band defined by
the current roadmap.

## Required Outcome

This issue is docs-only. It must:

- remove duplicate copies of active planning docs that still remain after `#1060`
- keep the roadmap-home copy for each affected document
- preserve any doc with unique content
- leave the active `v0.86+` planning bands without duplicate copies of the same
  active feature doc

## Deliverables

- duplicate planning docs removed from the wrong bands
- surviving planning docs kept in the roadmap-home bands
- any necessary local planning references updated if they still point at removed
  duplicate paths

## Acceptance Criteria

- `INSTINCT_MODEL.md` exists only in `v0.88planning`
- `PHI_METRICS_FOR_ADL.md` exists only in `v0.88planning`
- `SECURITY_AND_THREAT_MODELING.md` exists only in `v0.89planning`
- `TRACE_QUERY_LANGUAGE.md` exists only in `v0.90planning`
- `AFFECT_MODEL_v0.90.md` remains the kept affect-model file in `v0.91planning`
- no unique-content planning doc is deleted
- no demo is required

## Repo Inputs

- `.adl/docs/v0.86planning/INSTINCT_MODEL.md`
- `.adl/docs/v0.88planning/INSTINCT_MODEL.md`
- `.adl/docs/v0.86planning/PHI_METRICS_FOR_ADL.md`
- `.adl/docs/v0.88planning/PHI_METRICS_FOR_ADL.md`
- `.adl/docs/v0.89planning/SECURITY_AND_THREAT_MODELING.md`
- `.adl/docs/v0.95planning/SECURITY_AND_THREAT_MODELING.md`
- `.adl/docs/v0.90planning/TRACE_QUERY_LANGUAGE.md`
- `.adl/docs/v0.95planning/TRACE_QUERY_LANGUAGE.md`
- `.adl/docs/v0.91planning/AFFECT_MODEL_v0.9.md`
- `.adl/docs/v0.91planning/AFFECT_MODEL_v0.90.md`
- `.adl/docs/v0.85planning/ROAD_TO_v0.95.md`

## Dependencies

- `#1060`

## Demo Expectations

- No demo is required because this is a bounded planning-tree cleanup.

## Non-goals

- changing roadmap content beyond path-truth cleanup
- editing support/planning docs that are intentionally out of the milestone list
- rewriting substantive feature-doc content

## Issue-Graph Notes

- Remove only confirmed duplicate or misplaced copies.
- When two files differ only by formatting, keep the roadmap-home copy.
- If any unexpected unique content is discovered, stop and preserve it.

## Notes

- The active roadmap-home bands were already agreed during the `#1060`
  milestone reorganization work.
- This cleanup should be limited to duplicate removal and path-truth updates
  needed to keep the planning tree coherent.

## Tooling Notes

- The active `.adl/docs/...` planning tree is local and ignored, so the
  publishable tracked artifact for this issue is the issue body/PR record while
  the planning-tree cleanup itself is verified in the issue worktree.
