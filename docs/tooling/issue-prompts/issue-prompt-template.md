---
issue_prompt_schema: adl.issue.v1
wp: "WP-XX"
slug: "replace-me"
title: "[v0.85][WP-XX] Replace Me"
labels:
  - "track:roadmap"
  - "version:v0.85"
  - "type:task"
  - "area:runtime"
issue_number:
status: "draft"
action: "create"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "Sprint X"
required_outcome_type:
  - "code"
  - "docs"
  - "tests"
  - "demo"
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes: []
pr_start:
  enabled: true
  slug: "replace-me"
---

# Replace Me

## Summary

One short paragraph describing the work package, why it matters in the milestone, and what visible capability or proof surface it is expected to produce.

## Goal

State the concrete outcome this issue is meant to produce.

## Required Outcome

State what must be real by the end of this issue.

- If code is required, say so explicitly.
- If a demo is required, say so explicitly.
- If docs-only completion is allowed, say so explicitly.
- Prefer "must result in" language over vague improvement language.

## Deliverables

- Name the real outputs expected from the issue.
- Prefer concrete artifacts, code paths, tests, demos, or docs.

## Acceptance Criteria

- Make the criteria testable and bounded.
- Prefer "what must exist" over "what should feel improved".
- If demos are required, name them.
- If code is required, say so explicitly.

## Repo Inputs

- List the exact milestone docs, source files, schemas, prompts, or modules that define the starting point for this issue.
- Prefer concrete repo paths over generic references.
- Include the canonical issue(s) or issue prompt(s) that this work absorbs, depends on, or supersedes.

## Inputs

- Canonical milestone docs
- Existing issues that this work absorbs or supersedes
- Code/docs/schemas already in the repo that shape the work

## Dependencies

- List upstream WPs or issues

## Demo Expectations

- State the required demo or proof surface if this issue must produce one.
- If no demo is required, say why.
- If the demo belongs to a later WP, name that dependency explicitly.

## Non-goals

- Say what this issue is not meant to do

## Issue-Graph Notes

- Record duplicate, supersede, split, merge, or renumbering expectations here.
- Keep tracker cleanup intent explicit if this issue is part of a milestone reorganization.

## Notes

- Add migration, design, scope, or sequencing notes here if needed.

## Tooling Notes

- `title`, `labels`, `milestone_sprint`, `required_outcome_type`, `repo_inputs`, `canonical_files`, and demo metadata are machine-readable in the front matter.
- The H1 should identify the specific task, usually by reusing the human-readable `title` value or a cleaner version of it.
- Do not use a generic heading like "Issue Prompt" or "Structured Task Prompt" as the permanent visible title.
- `pr_start.slug` is the value tooling should pass to:
  - `swarm/tools/pr.sh start <issue> --slug <slug>`
- `repo_inputs` should stay lightweight coordination metadata; the long-form sections below remain the canonical issue content.
