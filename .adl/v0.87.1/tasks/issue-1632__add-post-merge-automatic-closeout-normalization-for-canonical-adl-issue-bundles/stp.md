---
issue_card_schema: adl.issue.v1
wp: "unassigned"
slug: "add-post-merge-automatic-closeout-normalization-for-canonical-adl-issue-bundles"
title: "[v0.87.1][tools] Add post-merge automatic closeout normalization for canonical .adl issue bundles"
labels:
  - "track:roadmap"
  - "type:task"
  - "area:tools"
  - "version:v0.87.1"
issue_number: 1632
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
  - "Automates canonical closeout normalization after merge so cleanup becomes default behavior."
pr_start:
  enabled: false
  slug: "add-post-merge-automatic-closeout-normalization-for-canonical-adl-issue-bundles"
---

## Summary

Make canonical `.adl` issue-bundle closeout happen automatically after merge instead of relying on later manual cleanup.

## Goal

Turn post-merge closeout normalization into the default behavior so merged issues stop leaving stale root `.adl` bundle state behind.

## Required Outcome

- automatically normalize the root canonical `.adl` issue bundle to merged/closed truth after merge
- perform safe post-merge residue cleanup without hiding provenance
- make the automation idempotent and reviewable

## Deliverables

- a post-merge normalization path for canonical `.adl` issue bundles
- bounded residue cleanup that preserves provenance when needed
- regression coverage for success and guarded failure paths

## Acceptance Criteria

- after PR merge closes an issue, tooling automatically normalizes the root canonical `.adl` issue bundle to merged/closed truth
- canonical closeout fields in `sor.md` are updated without requiring a separate manual hygiene issue
- safe post-merge residue cleanup is performed for the merged issue path while preserving provenance where duplicate or superseded bundles exist
- the automation reports what it normalized so the result is reviewable rather than opaque
- when normalization cannot be completed safely, the tool fails loudly with a bounded actionable error
- rerunning the automation after successful normalization does not create churn
- regression coverage includes a successful post-merge normalization path and a guarded failure path

## Repo Inputs

- `https://github.com/danielbaustin/agent-design-language/issues/1632`
- `.adl/docs/TBD/MILESTONE_COMPRESSION_PLAN.md`
- `.adl/v0.87.1/tasks/issue-1555__v0-87-1-records-normalize-remaining-closed-issue-task-bundles-to-truthful-merged-closeout-state/`
- `adl/tools/skills/pr-closeout/SKILL.md`

## Dependencies

- `#1555` as the motivating cleanup wave
- the compressed issue-record model and existing closeout tooling

## Demo Expectations

- No standalone demo required. Proof is deterministic post-merge normalization behavior plus regression coverage.

## Non-goals

- manual one-off cleanup as the primary closeout path
- unrelated branch/worktree lifecycle changes
- silent mutation with no operator-visible normalization report

## Issue-Graph Notes

- This issue is the automation counterpart to the SOR-truth guard and the legacy-residue migration.

## Notes

- The desired result is that future merged issues self-normalize instead of spawning cleanup issues.

## Tooling Notes

- Keep GitHub issue metadata, local source prompt, and task cards aligned.
