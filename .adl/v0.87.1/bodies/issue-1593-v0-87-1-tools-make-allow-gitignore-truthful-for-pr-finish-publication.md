---
issue_card_schema: adl.issue.v1
wp: "unassigned"
slug: "v0-87-1-tools-make-allow-gitignore-truthful-for-pr-finish-publication"
title: "[v0.87.1][tools] Make --allow-gitignore truthful for pr finish publication"
labels:
  - "area:tools"
  - "type:bug"
  - "severity:medium"
  - "version:v0.87.1"
issue_number: 1593
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
  slug: "v0-87-1-tools-make-allow-gitignore-truthful-for-pr-finish-publication"
---

## Summary

Fix the `pr finish --allow-gitignore` contract so the flag’s behavior matches what operators reasonably infer from its name.

## Goal

Ensure `--allow-gitignore` either truthfully enables the bounded ignored-path publication behavior required by `finish` or is narrowed/renamed so it no longer implies staging support that does not exist.

## Required Outcome

- define the intended semantics of `--allow-gitignore` for `pr finish`
- align staging/publication behavior with that contract, or narrow the flag so it cannot mislead operators
- add regression coverage for the resolved behavior
- keep the semantics bounded to `finish` publication, not arbitrary ignored-file handling

## Deliverables

- `pr finish` flag-behavior fix or contract tightening
- updated tests covering the resolved semantics
- any needed help text or documentation adjustments

## Acceptance Criteria

- `--allow-gitignore` behavior matches its operator-facing meaning for `pr finish`
- `finish` no longer implies ignored required outputs will be staged when they will not be
- automated tests cover the resolved flag contract
- the resulting behavior is deterministic and bounded to the current issue/publication path

## Repo Inputs

- `adl/src/cli/pr_cmd.rs`
- `adl/src/cli/pr_cmd_args.rs`
- `adl/src/cli/tests/pr_cmd_inline/finish.rs`
- `adl/tools/pr.sh`
- any relevant finish/help documentation

## Dependencies

- issue-level publication behavior for canonical ignored bundle files should already be fixed or be fixed in parallel

## Demo Expectations

- no demo required; focused CLI/test validation is sufficient

## Non-goals

- redesigning global gitignore policy
- adding generic ignored-file publication support outside `finish`
- broad CLI flag cleanup unrelated to this contract

## Issue-Graph Notes

- this is closely related to the canonical ignored-bundle publication defect but focuses specifically on the operator-facing flag contract
- it should land as a bounded follow-on or companion fix, not a general CLI redesign

## Notes

- today the flag suppresses the `.gitignore` diff guard but does not change ignored-path staging behavior

## Tooling Notes

- use the ADL PR lifecycle
- cover both the happy path and the misleading current behavior in tests
