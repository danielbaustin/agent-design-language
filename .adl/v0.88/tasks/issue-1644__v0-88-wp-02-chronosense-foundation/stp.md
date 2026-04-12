---
issue_card_schema: adl.issue.v1
wp: "WP-02"
slug: "v0-88-wp-02-chronosense-foundation"
title: "[v0.88][WP-02] Chronosense foundation"
labels:
  - "track:roadmap"
  - "type:task"
  - "area:runtime"
  - "version:v0.88"
issue_number: 1644
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
  slug: "v0-88-wp-02-chronosense-foundation"
---

## Summary

Establish the conceptual chronosense substrate for `v0.88` as the first bounded temporal runtime band, with clear runtime-facing definitions, acceptance criteria, and at least one proof hook that later temporal work can build on.

## Goal

Define the chronosense foundation as a truthful, inspectable milestone surface that can support temporal schema, continuity, commitments, causality, and retrieval work in later `v0.88` packages.

## Required Outcome

- runtime-facing chronosense definitions are explicit and bounded
- acceptance criteria for the chronosense substrate are concrete and reviewable
- at least one bounded proof hook or fixture path is identified for later execution work
- the issue is scoped as the conceptual/runtime foundation, not the whole temporal milestone

## Deliverables

- tracked chronosense foundation implementation/doc surface required by the repo truth
- any supporting tests or fixtures needed to prove the bounded chronosense hook
- updated milestone-facing references only where needed to keep `v0.88` truthful

## Acceptance Criteria

- chronosense is defined as a bounded substrate rather than vague future aspiration
- the issue names the concrete runtime-facing surfaces it owns
- downstream temporal work can cite this issue as the chronosense foundation without re-deriving scope
- proof expectations are explicit and do not overclaim continuity or full identity completion

## Repo Inputs

- docs/milestones/v0.88/WBS_v0.88.md
- docs/milestones/v0.88/SPRINT_v0.88.md
- docs/milestones/v0.88/DESIGN_v0.88.md
- docs/milestones/v0.88/features/CHRONOSENSE_AND_IDENTITY.md
- docs/milestones/v0.88/features/SUBSTANCE_OF_TIME.md

## Dependencies

- WP-01 canonical planning package complete

## Demo Expectations

- no flagship demo required for this issue
- one bounded proof hook or fixture expectation should be recorded if needed for later validation

## Non-goals

- full continuity and identity semantics
- temporal query/retrieval execution
- commitments/deadlines implementation
- release-tail review or demo integration work

## Issue-Graph Notes

- this is the conceptual/runtime foundation for the temporal band
- later temporal issues should depend on this surface instead of restating chronosense from scratch

## Notes

- keep the language disciplined and runtime-facing
- do not imply future-milestone identity continuity or chronosense completion beyond the bounded `v0.88` scope

## Tooling Notes

- bootstrap to readiness only in this pass; no branch or worktree creation yet

