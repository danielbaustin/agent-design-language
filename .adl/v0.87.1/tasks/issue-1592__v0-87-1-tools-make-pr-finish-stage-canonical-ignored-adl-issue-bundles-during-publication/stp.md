---
issue_card_schema: adl.issue.v1
wp: "unassigned"
slug: "v0-87-1-tools-make-pr-finish-stage-canonical-ignored-adl-issue-bundles-during-publication"
title: "[v0.87.1][tools] Make pr finish stage canonical ignored .adl issue bundles during publication"
labels:
  - "area:tools"
  - "type:bug"
  - "severity:high"
  - "version:v0.87.1"
issue_number: 1592
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
  slug: "v0-87-1-tools-make-pr-finish-stage-canonical-ignored-adl-issue-bundles-during-publication"
---

## Summary

Fix the `pr finish` publication path so required canonical `.adl` issue-bundle artifacts are staged and published even when `.adl/` is gitignored.

## Goal

Make `pr finish` end-to-end truthful and reliable for current-issue publication by ensuring it can stage the canonical issue body plus `stp.md`, `sip.md`, and `sor.md` without requiring manual `git add -f` recovery.

## Required Outcome

- publish the canonical current-issue `.adl` bundle deterministically during `finish`
- keep staging bounded to the required current-issue bundle rather than broadly force-adding ignored files
- add regression coverage for ignored-canonical-bundle publication behavior
- preserve existing validation and non-ignored publication behavior

## Deliverables

- `pr finish` staging/publication fix in the Rust CLI path
- regression tests for canonical ignored bundle publication
- any tightly scoped documentation or comments needed to reflect the behavior

## Acceptance Criteria

- `pr finish` succeeds when the only remaining publishable changes are the canonical ignored `.adl` issue body and task-bundle files
- `pr finish` also succeeds when tracked changes and canonical ignored bundle files are mixed
- the fix force-stages only the required current-issue canonical bundle files, not arbitrary ignored content
- automated regression coverage exists in the `finish` test surface

## Repo Inputs

- `adl/src/cli/pr_cmd.rs`
- `adl/src/cli/pr_cmd_args.rs`
- `adl/src/cli/pr_cmd/lifecycle.rs`
- `adl/src/cli/tests/pr_cmd_inline/finish.rs`
- `.gitignore`

## Dependencies

- existing `pr finish` canonical-output sync behavior should remain the source of truth for what must be published

## Demo Expectations

- no demo required; focused CLI/test validation is sufficient

## Non-goals

- redesigning `.gitignore`
- moving canonical issue records out of `.adl`
- generic support for publishing arbitrary ignored files outside the current issue bundle

## Issue-Graph Notes

- this is the concrete fix for the publication failure reproduced while publishing issue `#1589`
- it should resolve the need for manual `git add -f` of canonical issue-bundle artifacts

## Notes

- this is a publication correctness bug, not a source-prompt or card-editing issue

## Tooling Notes

- use the ADL PR lifecycle
- keep the fix and tests bounded to current-issue canonical bundle publication
