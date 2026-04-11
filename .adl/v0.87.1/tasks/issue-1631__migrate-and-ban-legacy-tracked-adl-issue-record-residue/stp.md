---
issue_card_schema: adl.issue.v1
wp: "unassigned"
slug: "migrate-and-ban-legacy-tracked-adl-issue-record-residue"
title: "[v0.87.1][tools] Migrate and ban legacy tracked .adl issue-record residue"
labels:
  - "track:roadmap"
  - "type:task"
  - "area:tools"
  - "area:docs"
  - "version:v0.87.1"
issue_number: 1631
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
  - "Completes the compressed-model migration by removing legacy tracked record residue."
pr_start:
  enabled: false
  slug: "migrate-and-ban-legacy-tracked-adl-issue-record-residue"
---

## Summary

Finish the compressed-model migration by eliminating legacy tracked `.adl` issue-record residue and preventing it from coming back.

## Goal

Remove the class of legacy tracked `.adl` issue-record drift that forced the `#1555` cleanup wave.

## Required Outcome

- identify and normalize the remaining legacy tracked `.adl` issue-record residue
- preserve truthful provenance where history must be retained
- add a guard that rejects reintroduction of banned tracked residue patterns

## Deliverables

- normalized or migrated legacy tracked record surfaces
- a tooling guard that rejects newly introduced legacy tracked residue
- documentation of the canonical tracked-vs-local boundary

## Acceptance Criteria

- all remaining legacy tracked `.adl` issue-record residue covered by this issue is identified and normalized
- migration or removal preserves truthful provenance instead of silently discarding history
- a guard fails when new tracked legacy `.adl` issue-record residue is introduced
- the guard clearly distinguishes allowed canonical current-state issue bundles from banned tracked residue patterns
- the failure path surfaces a bounded remediation message
- regression coverage proves the guard rejects legacy tracked residue and allows canonical current-state bundles
- the canonical tracked-vs-local boundary is documented clearly enough to prevent future ambiguity

## Repo Inputs

- `https://github.com/danielbaustin/agent-design-language/issues/1631`
- `.adl/docs/TBD/MILESTONE_COMPRESSION_PLAN.md`
- `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`
- `.adl/v0.87.1/tasks/issue-1555__v0-87-1-records-normalize-remaining-closed-issue-task-bundles-to-truthful-merged-closeout-state/`

## Dependencies

- `#1555` as the motivating residue-cleanup issue
- the compressed local-record model adopted in the current tooling flow

## Demo Expectations

- No standalone demo required. Proof is migration truth plus guard behavior and regression coverage.

## Non-goals

- creating another parallel record model
- silent deletion of historical residue without provenance
- broad unrelated docs cleanup outside the tracked-vs-local boundary problem

## Issue-Graph Notes

- This issue complements the SOR-truth gate by removing the legacy residue that still violates the compressed model.

## Notes

- Treat this as the migration-and-ban step, not just a one-time cleanup note.

## Tooling Notes

- Keep GitHub issue metadata, local source prompt, and task cards aligned.
