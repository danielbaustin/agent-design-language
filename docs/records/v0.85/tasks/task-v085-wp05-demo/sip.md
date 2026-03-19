---
artifact_type: "Structured Implementation Prompt"
title: "[v0.85][WP-05] First authoring/editor surfaces"
task_id: "task-v085-wp05-demo"
run_id: "task-v085-wp05-demo"
version: "v0.85"
branch: "codex/870-v085-wp05-first-editor-surfaces"
required_outcome_type: "code"
demo_required: "true"
---

# Structured Implementation Prompt

## Summary

Implement the first bounded editor surface and prove that it supports tracked public task records.

## Goal

Land a static tracked editor that helps a human author STPs and SIPs with less structural fragility.

## Required Outcome

- code for the editor surface
- docs describing the first-slice workflow
- a tracked example task bundle

## Acceptance Criteria

- the editor supports STP and SIP authoring
- required sections are checked in-browser
- the rendered markdown artifact is previewed live

## Inputs

- source STP for `#870`
- tracked public record layout

## Target Files / Surfaces

- `docs/tooling/editor/index.html`
- `docs/tooling/editor/task_bundle_editor.js`
- `docs/records/v0.85/tasks/task-v085-wp05-demo/`

## Validation Plan

- open the editor HTML locally
- confirm validation reacts to missing required fields
- confirm the preview shows the full rendered artifact

## Demo / Proof Requirements

- required proof surface: tracked example STP/SIP pair and editor walkthrough

## Constraints / Policies

- preserve tracked public-record orientation
- avoid absolute host paths

## Non-goals / Out of Scope

- direct file writes from the browser

## Notes / Risks

- SOR editing is intentionally deferred to later work
