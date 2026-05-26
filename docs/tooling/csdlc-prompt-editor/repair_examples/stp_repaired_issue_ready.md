---
issue_card_schema: adl.issue.v1
wp: "WP-EX"
slug: "example-stp-repair"
title: "[example][STP] Ready task repair"
labels:
  - "track:roadmap"
issue_number: 4002
generated_at: "2026-05-26T12:00:00Z"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on:
  - "#4001"
milestone_sprint: "v0.91.4"
required_outcome_type:
  - "code"
repo_inputs:
  - ".adl/v0.91.4/bodies/example-stp-repair.md"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Example STP showing issue-specific deliverables and non-goals after bounded repair."
pr_start:
  enabled: true
  slug: "example-stp-repair"
---

Canonical Template Source: `docs/templates/prompts/1.0.0/stp.md`
Generated: 2026-05-26T12:00:00Z

# Structured Task Prompt

## Summary

Repair a generic STP into a bounded, issue-specific task card.

## Goal

Make the task executable without changing the source issue intent.

## Required Outcome

Deliver an issue-specific `STP` with concrete deliverables, scoped repo inputs,
clear acceptance criteria, and explicit non-goals.

## Deliverables

Validator-clean task wording; explicit repo inputs; clear acceptance criteria;
and an issue-local validation plan.

## Acceptance Criteria

The repaired `STP` is specific enough for design-time readiness and does not
silently widen scope.

## Repo Inputs

`docs/templates/prompts/1.0.0/`; `docs/tooling/csdlc-prompt-editor/`

## Dependencies

`#4001`

## Target Files / Surfaces

`docs/tooling/csdlc-prompt-editor/repair_examples/`

## Validation Plan

Run the matching structured prompt validator for `stp`.

## Demo Expectations

None. Proof is the repaired card plus validator pass.

## Non-goals

Do not invent implementation results or rewrite unrelated cards.

## Issue-Graph Notes

This example models the shape expected after bounded `stp-editor` repair.

## Notes

Keep the card specific enough for execution planning, but do not turn it into a
full execution log.

## Tooling Notes

Use `workflow-conductor`, then route only the STP surface through `stp-editor`.
