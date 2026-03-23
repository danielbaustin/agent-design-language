---
issue_card_schema: adl.issue.v1
wp: "WP-20"
slug: "redistribute-post-v085-planning-feature-docs"
title: "[v0.85][docs] Redistribute post-v0.85 planning feature docs into milestone planning directories"
labels:
  - "track:roadmap"
  - "version:v0.85"
  - "type:docs"
  - "area:planning"
  - "area:roadmap"
issue_number: 997
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "Sprint 4"
required_outcome_type:
  - "docs"
repo_inputs:
  - ".adl/docs/v0.85planning/FEATURE_FILES_DISTRIBUTION.md"
  - ".adl/docs/v0.86planning/"
  - ".adl/docs/v0.87planning/"
  - ".adl/docs/v0.88planning/"
  - ".adl/docs/v0.89planning/"
  - ".adl/docs/v0.90planning/"
  - ".adl/docs/v0.91planning/"
  - ".adl/docs/v0.92planning/"
  - ".adl/docs/v0.93planning/"
  - ".adl/docs/v0.94planning/"
  - ".adl/docs/v0.95planning/"
canonical_files:
  - ".adl/docs/v0.85planning/FEATURE_FILES_DISTRIBUTION.md"
demo_required: false
demo_names: []
issue_graph_notes:
  - "This issue only redistributes existing post-v0.85 feature/planning docs across `.adl/docs/` milestone planning directories."
  - "It does not promote docs into `docs/milestones/`."
  - "The distribution table in `FEATURE_FILES_DISTRIBUTION.md` is the source of truth for destination placement."
pr_start:
  enabled: true
  slug: "redistribute-post-v085-planning-feature-docs"
---

# Redistribute post-v0.85 planning feature docs

## Summary

Redistribute existing post-v0.85 planning and feature docs into their appropriate
future `.adl/docs/v0.xxplanning/` directories, using
`.adl/docs/v0.85planning/FEATURE_FILES_DISTRIBUTION.md` as the move map.

## Goal

Make the future planning tree structurally coherent so later milestone promotion
work starts from the correct `.adl/docs/` directories instead of from a mixed or
misbucketed planning set.

## Required Outcome

This issue is docs-only:

- move planning/feature docs only within `.adl/docs/`
- assign docs to the correct future milestone planning directories
- do not promote docs into `docs/milestones/` in this pass
- preserve roadmap, reconciliation, and admin/control docs unless the
  distribution table explicitly reassigns them

## Deliverables

- future planning docs redistributed into the milestone-planning directories
  specified by `FEATURE_FILES_DISTRIBUTION.md`
- no unintended doc rewrites beyond path/location maintenance needed for the move
- no milestone-canon promotions

## Acceptance Criteria

- every in-scope post-v0.85 planning/feature doc named in
  `FEATURE_FILES_DISTRIBUTION.md` is moved to its assigned `.adl/docs/v0.xxplanning/`
  destination
- docs that are meant to remain in place are left in place
- no redistributed document remains duplicated across planning milestone
  directories; each document must exist in exactly one `.adl/docs/v0.xxplanning/`
  location unless duplication is explicitly intended and justified
- roadmap/reconciliation/admin docs are not moved unless explicitly listed in
  the distribution table
- no docs are promoted into `docs/milestones/`
- path references updated only where required by the move itself

## Out Of Scope

- promoting future planning docs into milestone canon
- rewriting the content of planning docs beyond what is necessary for relocation
- changing roadmap intent or milestone scope
- broad cleanup outside the distribution-table instructions

## Risk / Review Checkpoints

- `.adl/docs/v0.85planning/FEATURE_FILES_DISTRIBUTION.md`
- any file moved across milestone planning directories
- any file intentionally left in place despite appearing in the distribution
  table

## Validation

- drive validation from `.adl/docs/v0.85planning/FEATURE_FILES_DISTRIBUTION.md`
- verify every file listed in the distribution table is either:
  - present at its assigned destination, or
  - intentionally left in place with justification
- verify each moved file exists only at the intended destination under `.adl/docs/`
- verify no duplicate copies remain after redistribution unless explicitly
  intended and justified
- verify no target file was overwritten unexpectedly
- verify the distribution table and resulting tree agree
- run targeted path checks (`git status`, `find`, `ls`) rather than code/test gates

## Dependencies

- none required for correctness; `#880` is related context only

## Demo Expectations

- no demo required; proof surface is the final directory layout under `.adl/docs/`

## Source

- `.adl/docs/v0.85planning/FEATURE_FILES_DISTRIBUTION.md`
