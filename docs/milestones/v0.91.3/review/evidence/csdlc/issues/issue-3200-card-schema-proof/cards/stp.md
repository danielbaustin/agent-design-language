---
issue_card_schema: adl.issue.v1
wp: "WP-02"
queue: "docs_tools"
slug: "v0-91-3-wp-02-cognitive-transition-schema"
title: "[v0.91.3][WP-02][docs/tools] Cognitive Transition schema"
labels:
  - "track:roadmap"
  - "type:task"
  - "area:tools"
  - "version:v0.91.3"
issue_number: 3200
status: "draft"
action: "edit"
depends_on:
  - "WP-01"
milestone_sprint: "Sprint 1 / Transition Substrate"
required_outcome_type:
  - "code"
repo_inputs:
  - "AGENTS.md"
  - "docs/milestones/v0.91.3/WBS_v0.91.3.md"
  - "docs/milestones/v0.91.3/SPRINT_v0.91.3.md"
  - "docs/milestones/v0.91.3/WP_EXECUTION_READINESS_v0.91.3.md"
  - "docs/milestones/v0.91.3/WP_ISSUE_WAVE_v0.91.3.yaml"
  - "docs/milestones/v0.91.3/features/COGNITIVE_TRANSITION_MANIFEST.md"
  - "docs/cognitive-sdlc/"
canonical_files:
  - "docs/milestones/v0.91.3/features/COGNITIVE_TRANSITION_MANIFEST.md"
  - "docs/milestones/v0.91.3/WP_ISSUE_WAVE_v0.91.3.yaml"
demo_required: false
demo_names: []
issue_graph_notes:
  - "Mirrored from the authored GitHub issue body during bootstrap/init."
pr_start:
  enabled: false
  slug: "v0-91-3-wp-02-cognitive-transition-schema"
---

## Summary

Execute `WP-02` for `v0.91.3`: Cognitive Transition schema. This issue does the bounded WP-02 work after its dependencies are satisfied.

## Goal

Produce useful, reviewable work for the first Cognitive SDLC implementation milestone while preserving the `SIP -> STP -> SPP -> SRP -> SOR` lifecycle and the repository-local workflow discipline from `AGENTS.md`.

## Required Outcome

manifest schema, actor-role seed, states, fixtures, and validation plan.

## Deliverables

- bounded `WP-02` work product for `Cognitive Transition schema`
- updated source, docs, fixtures, demos, or review records required by the work package
- truthful `SIP`, `STP`, `SPP`, `SRP`, and `SOR` cards
- focused validation evidence appropriate to the touched surface
- pre-PR review results recorded in `SRP`
- closeout-ready `SOR` outcome truth after merge or intentional closure

## Acceptance Criteria

- the work product satisfies the `WP-02` outcome in `docs/milestones/v0.91.3/WP_ISSUE_WAVE_v0.91.3.yaml`
- dependencies are respected: `WP-01`
- the implementation stays within the `docs/tools` queue and does not absorb unrelated milestone work
- cards remain lifecycle-truthful and use editor skills for card changes
- validation is focused, reproducible, and recorded
- no broad runtime test cycle is run unless the touched surface requires it
- review findings are fixed or explicitly routed before PR publication

## Repo Inputs

- `AGENTS.md`
- `docs/milestones/v0.91.3/README.md`
- `docs/milestones/v0.91.3/WBS_v0.91.3.md`
- `docs/milestones/v0.91.3/SPRINT_v0.91.3.md`
- `docs/milestones/v0.91.3/WP_EXECUTION_READINESS_v0.91.3.md`
- `docs/milestones/v0.91.3/WP_ISSUE_WAVE_v0.91.3.yaml`
- `docs/milestones/v0.91.3/FEATURE_PROOF_COVERAGE_v0.91.3.md`
- `docs/milestones/v0.91.3/QUALITY_GATE_v0.91.3.md`
- `docs/milestones/v0.91.3/features/`
- `docs/cognitive-sdlc/`

## Dependencies

- `WP-01`

## Demo Expectations

- Follow `docs/milestones/v0.91.3/DEMO_MATRIX_v0.91.3.md`; if this WP has no demo lane, record the no-demo rationale in `SOR`.

## Non-goals

- do not widen beyond `WP-02`
- do not bypass `workflow-conductor`
- do not edit cards without editor skills
- do not work on `main`
- do not claim full C-SDLC adoption before v0.91.4
- do not depend on GWS or any external collaboration workspace as required infrastructure

## Issue-Graph Notes

- Milestone: `v0.91.3`
- Work package: `WP-02`
- Queue: `docs_tools`
- Dependency expression from issue wave: `WP-01`
- Created by `WP-01` / `#3199`.

## Notes

- Every WP in `v0.91.3` must produce real, useful work; docs-only is acceptable only when the WP itself is a docs/control-plane work package.
- Keep proof claims evidence-bound and repo-relative.

## Tooling Notes

- Use `workflow-conductor` for lifecycle routing.
- Use `adl/tools/pr.sh run <issue> --version v0.91.3` to bind execution.
- Use the matching editor skill for card updates.
- Use focused validation for the touched surface.
- Use `pr finish` for publication and `pr closeout` after merge or closure.
