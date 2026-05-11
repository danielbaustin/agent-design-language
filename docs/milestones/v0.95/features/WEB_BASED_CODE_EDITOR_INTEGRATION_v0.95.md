# v0.95 Feature: Web-Based Code Editor Integration

## Status

Forward-planning feature contract for `v0.95`.

## Purpose

Define the required editor/operator integration baseline for the MVP using
HTML-based in-repo editor surfaces, independently of whether Zed is ultimately
shipped.

## Source Inputs

- `docs/milestones/v0.85/HTA_EDITOR_PLANNING.md`
- `docs/milestones/v0.85/ideas/V095_MVP_BOUNDARY.md`
- `docs/planning/ADL_FEATURE_LIST.md`

## Scope

This feature should establish:

- a credible HTML-based editor capability for authoring, execution, and review
- explicit coupling to the validated control-plane lifecycle rather than a
  parallel state model
- the minimum required editor surface even if Zed remains optional

## Non-goals

- silent substitution of host-editor integration for the required web/editor
  baseline
- bypassing control-plane validation or workflow truth
- claiming polished host-specific integration as the only acceptable MVP path

## Completion Target

`v0.95`
