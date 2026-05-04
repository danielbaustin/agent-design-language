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

Repair the local v0.90.5 records surfaces around `#2683` and `#2699` now that the originally reported missing-bundle state has changed. This issue is bounded to local task-bundle continuity, duplicate-closeout residue cleanup, and rerunning the existing records guards.

## Goal

Normalize only the local issue-record surfaces for `#2683` and `#2699` so the v0.90.5 records inventory is internally consistent and the existing closeout/residue guards stay green.

## Required Outcome

Ship one bounded records-only repair pass that:
- preserves the canonical `#2683` task bundle and aligns its STP/SIP/SOR surfaces with the closed issue and merged PR state
- normalizes the duplicate-closeout residue left in `#2699`'s local SOR
- reruns the closeout-truth and residue guards and records a clean result

## Deliverables

- updated local task-bundle records for `#2683`, `#2699`, and `#2704` only where needed
- no runtime, test, or product-behavior changes
- guard results showing the v0.90.5 local records surfaces are clean after the repair

## Acceptance Criteria

- `#2683` has a present canonical local task bundle with STP/SIP/SOR continuity and no bootstrap-only contradiction against merged PR `#2691`
- `#2699` records its duplicate closeout truthfully with no stale `worktree_only` residue
- `adl/tools/check_milestone_closed_issue_sor_truth.sh --version v0.90.5` passes after the repair
- `adl/tools/check_no_tracked_adl_issue_record_residue.sh` remains clean

## Repo Inputs

- https://github.com/danielbaustin/agent-design-language/issues/2704
- `.adl/v0.90.5/tasks/issue-2683__v0-90-5-daily-coverage-blockers-2026-05-02`
- `.adl/v0.90.5/tasks/issue-2699__v0-90-5-tools-stop-pr-finish-tests-from-leaking-orphaned-post-merge-closeout-watchers`
- `adl/tools/check_milestone_closed_issue_sor_truth.sh`
- `adl/tools/check_no_tracked_adl_issue_record_residue.sh`

## Dependencies

- none recorded yet

## Demo Expectations

- No demo is required. The proof surface is the bounded local records diff plus the existing closeout/residue guard commands.

## Non-goals

- recreating task bundles that already exist locally
- widening into `#2700` implementation work, new coverage authoring, or repo-wide records cleanup
- changing runtime, CI, or milestone semantics

## Issue-Graph Notes

- `#2683` and `#2699` are already closed, so this issue is a local records normalization lane rather than a fresh implementation lane.
- `#2683` is closed with merged PR `#2691`, while `#2699` is a closed duplicate redirected to `#2700`.

## Notes

- Treat the original "missing bundle" report as stale evidence: the local `#2683` bundle now exists, but its continuity/truth still needs cleanup.
- Keep the issue bounded to local `.adl` records surgery and guard reruns.

## Tooling Notes

- Use the conductor-guided workflow and route any card-local cleanup through the matching editor skill.
- Do not widen into `#2700` implementation work or other records-hygiene sweeps beyond the named `#2683`/`#2699` surfaces.
