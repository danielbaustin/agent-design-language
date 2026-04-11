---
issue_card_schema: adl.issue.v1
wp: "unassigned"
slug: "v0-87-1-skills-normalize-repo-code-review-skill-input-schema-and-docs"
title: "[v0.87.1][skills] Normalize repo-code-review skill input schema and docs"
labels:
  - "track:roadmap"
  - "area:tools"
  - "type:task"
  - "version:v0.87.1"
issue_number: 1589
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Pending sprint assignment"
required_outcome_type:
  - "code"
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Mirrored from the authored GitHub issue body during bootstrap/init."
pr_start:
  enabled: false
  slug: "v0-87-1-skills-normalize-repo-code-review-skill-input-schema-and-docs"
---

## Summary

Normalize the `repo-code-review` skill to the same explicit input-schema standard used by the PR-phase and card-editor skills.

## Goal

Make the repo review skill machine-self-describing and automation-safe by adding a canonical input schema surface, wiring it into the manifest, and syncing the guide/test surfaces to match.

## Required Outcome

- add an explicit input schema document for `repo-code-review`
- update the skill manifest to declare the schema id and reference doc
- align the operational guide and any contract tests so the skill is documented and validated like the other normalized skills
- preserve the current review behavior and output contract

## Deliverables

- schema doc under `adl/tools/skills/docs/`
- updated `adl/tools/skills/repo-code-review/adl-skill.yaml`
- any needed guide/test updates for schema parity

## Acceptance Criteria

- `repo-code-review` has a stable named input schema and reference doc in the manifest
- the schema doc explains required fields, optional fields, and invocation shape clearly enough for automation use
- operational docs describe the skill using the new schema surface
- contract or fixture tests cover the new schema linkage
- no review behavior or output contract is weakened

## Repo Inputs

- `adl/tools/skills/repo-code-review/adl-skill.yaml`
- `adl/tools/skills/repo-code-review/SKILL.md`
- `adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`
- neighboring normalized skill manifests and schema docs
- any skill contract tests for repo-local validation

## Dependencies

- existing normalized PR/editor skill schema patterns should be available for reuse

## Demo Expectations

- no runtime demo required; bounded skill-contract validation is sufficient

## Non-goals

- rewriting the repo review output contract
- changing the substantive review methodology beyond schema clarity
- broad skill-system redesign

## Issue-Graph Notes

- this is schema/contract parity work for an existing skill bundle
- prefer the same machine-readable shape already used by `pr-init`, `pr-ready`, `pr-run`, `pr-finish`, `pr-closeout`, and the editor skills

## Notes

- the main gap is input-schema formalization, not output-contract coverage

## Tooling Notes

- use the ADL PR lifecycle
- validate both the manifest linkage and the repo review contract fixtures

