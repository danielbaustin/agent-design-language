---
issue_card_schema: adl.issue.v1
wp: "unassigned"
queue: "wp"
slug: "v0-90-5-records-repair-missing-duplicate-task-bundle-records"
title: "[v0.90.5][records] Repair missing/duplicate task-bundle records: #2683 and #2699 duplicate residue"
labels:
  - "track:roadmap"
  - "area:records"
  - "type:task"
  - "version:v0.90.5"
issue_number: 2704
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
  - "Bootstrap-generated from GitHub issue metadata because no canonical local issue prompt existed yet."
pr_start:
  enabled: false
  slug: "v0-90-5-records-repair-missing-duplicate-task-bundle-records"
---

# [v0.90.5][records] Repair missing/duplicate task-bundle records: #2683 and #2699 duplicate residue

## Summary

Repair the local v0.90.5 records surfaces for `#2683` and `#2699` so the named task bundles and duplicate-closeout records match current repo and GitHub truth.

## Goal

Normalize only the local issue-record surfaces involved in the `#2683`/`#2699` report and rerun the existing records guards.

## Required Outcome

This issue must ship a bounded records-surgery pass that restores `#2683` STP/SIP/SOR continuity, removes stale duplicate-closeout residue from `#2699`, and records clean guard results without changing runtime behavior.

## Deliverables

- updated local task-bundle records for `#2683`, `#2699`, and this issue where needed
- no code, test, or milestone-behavior changes
- clean closeout-truth and residue guard results

## Acceptance Criteria

- `#2683` retains a present canonical local bundle with internally consistent STP/SIP/SOR truth and merged PR linkage
- `#2699` records duplicate closeout truthfully with no stale `worktree_only` language
- `adl/tools/check_milestone_closed_issue_sor_truth.sh --version v0.90.5` passes
- `adl/tools/check_no_tracked_adl_issue_record_residue.sh` passes
- the issue remains bounded to local records surfaces only

## Repo Inputs

- https://github.com/danielbaustin/agent-design-language/issues/2704
- `.adl/v0.90.5/tasks/issue-2683__v0-90-5-daily-coverage-blockers-2026-05-02`
- `.adl/v0.90.5/tasks/issue-2699__v0-90-5-tools-stop-pr-finish-tests-from-leaking-orphaned-post-merge-closeout-watchers`
- `adl/tools/check_milestone_closed_issue_sor_truth.sh`
- `adl/tools/check_no_tracked_adl_issue_record_residue.sh`

## Dependencies

- none recorded yet

## Demo Expectations

- No demo is required. The proof surface is the bounded local records diff plus the existing records guard commands.

## Non-goals

- recreating task bundles that already exist locally
- widening into `#2700` implementation work, runtime fixes, or repo-wide records cleanup
- changing runtime, CI, or milestone semantics

## Issue-Graph Notes

- `#2683` is already closed with merged PR `#2691`; this issue repairs continuity/truth in the local records only.
- `#2699` is already closed as a duplicate of `#2700`; this issue should not reopen or re-execute that tooling fix.

## Notes

- Treat the original missing-bundle report as stale evidence and repair only the remaining records drift.

## Tooling Notes

- Use the conductor-guided workflow.
- Route STP/SIP/SOR cleanup through the matching editor skill rather than ad hoc card edits.
