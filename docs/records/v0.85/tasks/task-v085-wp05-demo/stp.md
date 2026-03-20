---
artifact_type: "Structured Task Prompt"
title: "[v0.85][WP-05] First authoring/editor surfaces"
task_id: "task-v085-wp05-demo"
issue_number: "870"
status: "draft"
action: "edit"
milestone_sprint: "Sprint 2"
---

# [v0.85][WP-05] First authoring/editor surfaces

## Summary

Ship the first real editor surface for tracked task-bundle artifacts.

## Goal

Provide a bounded human-usable editor for Structured Task Prompts and Structured Implementation Prompts.

## Required Outcome

- real editor code exists in the repo
- public task-bundle destinations are visible during editing
- a bounded demo/proof surface exists

## Acceptance Criteria

- a user can edit required structured fields without writing raw markdown from scratch
- the rendered artifact preview updates live
- the public task-bundle destination is visible while editing
- the linked workspace keeps STP, SIP, and SOR together as one task bundle

## Repo Inputs

- `docs/tooling/editor/index.html`
- `docs/tooling/structured-prompt-contracts.md`
- `docs/records/v0.85/tasks/`

## Dependencies

- `WP-04`

## Demo Expectations

- required demo: `editor-workflow-demo`

## Non-goals

- full long-term productization

## Notes

- this is the first tracked public-record example for the editor slice
- the demo now serves as the canonical proof surface for the bounded editor loop
