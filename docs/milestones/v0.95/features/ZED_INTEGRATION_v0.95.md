# v0.95 Feature: Zed Integration

## Status

Forward-planning feature contract for `v0.95`.

## Purpose

Define the decision boundary for Zed as an ADL editor/operator host surface:
either ship a bounded credible integration or explicitly keep it out of the
must-have MVP set.

## Source Inputs

- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/milestones/v0.85/ideas/V095_MVP_BOUNDARY.md`
- `docs/milestones/v0.85/HTA_EDITOR_PLANNING.md`
- `docs/milestones/v0.85/features/ROAD_TO_v0.95.md`

## Scope

This feature should establish:

- the bounded Zed integration decision and success criteria
- compatibility with the validated control-plane lifecycle rather than a
  parallel workflow
- explicit relationship to the required HTML/editor-capability baseline
- a truth-preserving rule for shipping, deferring, or dropping Zed from MVP

## Non-goals

- silently promoting Zed into the must-have set
- bypassing control-plane validation
- treating host-editor preference as architectural proof

## Completion Target

`v0.95`
