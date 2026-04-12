---
issue_card_schema: adl.issue.v1
wp: WP-03
slug: v0-88-wp-03-temporal-schema
title: '[v0.88][WP-03] Temporal schema'
labels:
- track:roadmap
- type:task
- area:runtime
- version:v0.88
status: draft
action: edit
depends_on:
- WP-01
milestone_sprint: Sprint 1
required_outcome_type:
- code
- docs
- tests
repo_inputs:
- docs/milestones/v0.88/WBS_v0.88.md
- docs/milestones/v0.88/SPRINT_v0.88.md
- docs/milestones/v0.88/DESIGN_v0.88.md
- docs/milestones/v0.88/FEATURE_DOCS_v0.88.md
- docs/milestones/v0.88/features/SUBSTANCE_OF_TIME.md
- docs/milestones/v0.88/features/TEMPORAL_SCHEMA_V01.md
canonical_files:
- docs/milestones/v0.88/features/TEMPORAL_SCHEMA_V01.md
demo_required: false
demo_names: []
issue_graph_notes:
- WP-03 establishes the canonical temporal schema for the v0.88 temporal band.
- The issue should stay focused on anchors, clocks, execution-policy trace hooks, and serialization surface truth.
pr_start:
  enabled: false
  slug: v0-88-wp-03-temporal-schema
issue_number: 1646
---

# [v0.88][WP-03] Temporal schema

## Summary

Define the canonical temporal schema for the v0.88 chronosense band, including anchors, clocks, execution-policy trace hooks, and the fields needed to support temporal review and replay.

## Goal

Make the temporal record shape concrete enough that later continuity, retrieval, commitment, causality, and cost work can build on one authoritative schema.

## Required Outcome

- define the canonical temporal fields for agent events and memories
- make execution-policy and realized-cost anchoring explicit enough for reviewability
- keep the schema focused on structural time, not broad identity theory

## Deliverables

- temporal schema surface and supporting docs
- runtime serialization or validation surface for the schema
- targeted tests covering the schema contract

## Acceptance Criteria

- the schema captures the minimum temporal anchors required by the `v0.88` temporal package
- objective time, subjective time, and execution-policy/cost anchors are represented truthfully
- later docs can rely on this issue without inventing duplicate temporal fields
- the issue stays bounded to schema definition and associated tests/docs

## Repo Inputs

- `docs/milestones/v0.88/WBS_v0.88.md`
- `docs/milestones/v0.88/SPRINT_v0.88.md`
- `docs/milestones/v0.88/DESIGN_v0.88.md`
- `docs/milestones/v0.88/FEATURE_DOCS_v0.88.md`
- `docs/milestones/v0.88/features/SUBSTANCE_OF_TIME.md`
- `docs/milestones/v0.88/features/TEMPORAL_SCHEMA_V01.md`

## Dependencies

- `WP-01`

## Demo Expectations

- no standalone demo required
- proof is the schema contract plus validation/tests

## Non-goals

- broad chronosense philosophy rewrite
- retrieval semantics
- commitment lifecycle semantics
- later identity or governance scope

## Issue-Graph Notes

- This is the schema-contract issue for the v0.88 temporal band.
- Keep it distinct from the continuity, retrieval, commitment, causality, and cost issues that depend on it.

## Notes

- Prefer one authoritative schema over scattered temporal fields.

## Tooling Notes

- Keep the GitHub issue body and local source prompt aligned.
