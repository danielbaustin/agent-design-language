---
issue_card_schema: adl.issue.v1
wp: "unassigned"
slug: "v0-88-tools-make-issue-creation-fail-unless-doctor-reaches-ready-state"
title: "[v0.88][tools] Make issue creation fail unless doctor reaches ready state"
labels:
  - "track:roadmap"
  - "type:task"
  - "area:tools"
  - "version:v0.88"
issue_number: 1668
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
  slug: "v0-88-tools-make-issue-creation-fail-unless-doctor-reaches-ready-state"
---

## Summary

Make issue creation automatically run doctor/readiness validation and fail if the newly created issue does not reach immediate ready state.

## Goal

Collapse issue bootstrap and readiness validation into one truthful control-plane step so bad issue surfaces are rejected at creation time instead of being discovered later.

## Required Outcome

- `pr create` runs the canonical doctor/readiness surface automatically
- create fails when the new issue is not immediately `ready`
- callers receive actionable failure output describing what blocked readiness

## Deliverables

- control-plane integration between create and doctor
- tests for pass/fail create outcomes based on readiness state
- any docs needed to describe the stronger create contract

## Acceptance Criteria

- new tracked issues do not end in a silent `created but not ready` state
- readiness failure leaves enough evidence to repair the issue deterministically
- authored-body happy path remains fast and clean

## Repo Inputs

- adl/tools/pr.sh
- adl/src/cli/pr_cmd/doctor.rs
- adl/src/cli/pr_cmd/

## Dependencies

- none

## Demo Expectations

- no demo required

## Non-goals

- executing the issue after creation
- replacing `pr doctor` as the canonical diagnostic surface

## Issue-Graph Notes

- child of #1665
- complements the authored-body guard by enforcing readiness immediately after bootstrap

## Notes

- the desired outcome is `create -> ready`, not `create -> later surprise repair`

## Tooling Notes

- bootstrap only in this pass; no execution context creation

