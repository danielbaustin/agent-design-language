---
issue_card_schema: adl.issue.v1
wp: "WP-03"
queue: "tools"
slug: "v0-91-3-wp-03-card-lifecycle-integration"
title: "[v0.91.3][WP-03][tools] Card lifecycle integration"
labels:
  - "track:roadmap"
  - "type:task"
  - "area:tools"
  - "version:v0.91.3"
issue_number: 3201
status: "draft"
action: "edit"
depends_on:
  - "WP-02"
milestone_sprint: "Sprint 1 / Transition Substrate"
required_outcome_type:
  - "code"
repo_inputs:
  - "AGENTS.md"
  - "docs/cognitive-sdlc/card-lifecycle.md"
  - "docs/tooling/structured-prompt-contracts.md"
  - "adl/src/cli/pr_cmd/doctor.rs"
  - "adl/src/cli/tooling_cmd/tests/structured_prompt.rs"
canonical_files:
  - "docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/sip.md"
  - "docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/stp.md"
  - "docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/spp.md"
  - "docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/srp.md"
  - "docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/sor.md"
demo_required: false
demo_names: []
issue_graph_notes:
  - "Tracked public bundle for the first WP-03 card-lifecycle proof."
pr_start:
  enabled: false
  slug: "v0-91-3-wp-03-card-lifecycle-integration"
---

## Summary

Execute `WP-03` for `v0.91.3`: Card lifecycle integration.

## Goal

Produce useful, reviewable work for the first Cognitive SDLC implementation
milestone while preserving the `SIP -> STP -> SPP -> SRP -> SOR` lifecycle.

## Required Outcome

lifecycle validator and doctor expectations for the slice.

## Deliverables

- tracked public card bundle for the first C-SDLC issue-local proof
- focused validator and doctor lifecycle tests
- truthful `SIP`, `STP`, `SPP`, `SRP`, and `SOR` cards

## Acceptance Criteria

- the work product satisfies the `WP-03` outcome in
  `docs/milestones/v0.91.3/WP_ISSUE_WAVE_v0.91.3.yaml`
- new durable card records for the proof live under
  `docs/milestones/v0.91.3/review/evidence/csdlc/issues/`
- validators accept the tracked bundle directly
- doctor lifecycle expectations classify the tracked bundle correctly

## Repo Inputs

- `AGENTS.md`
- `docs/cognitive-sdlc/card-lifecycle.md`
- `docs/tooling/structured-prompt-contracts.md`
- `docs/milestones/v0.91.3/features/CARD_LIFECYCLE_INTEGRATION.md`
- `adl/src/cli/pr_cmd/doctor.rs`
- `adl/src/cli/tooling_cmd/tests/structured_prompt.rs`

## Dependencies

- `WP-02`

## Demo Expectations

- Follow `docs/milestones/v0.91.3/DEMO_MATRIX_v0.91.3.md`; this proof is
  validator/doctor driven rather than a standalone runtime demo.

## Non-goals

- do not widen beyond `WP-03`
- do not turn the tracked bundle into a default replacement for local active
  issue bundles

## Issue-Graph Notes

- Milestone: `v0.91.3`
- Work package: `WP-03`
- Queue: `tools`

## Notes

- Keep proof claims evidence-bound and repo-relative.

## Tooling Notes

- Use `workflow-conductor` for lifecycle routing.
- Use focused validation for the touched surface.
