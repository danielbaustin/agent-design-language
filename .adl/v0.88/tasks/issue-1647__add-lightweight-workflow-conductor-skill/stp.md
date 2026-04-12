---
issue_card_schema: adl.issue.v1
wp: "unassigned"
slug: "add-lightweight-workflow-conductor-skill"
title: "[v0.88][tools] Add lightweight workflow conductor skill"
labels:
  - "track:roadmap"
  - "type:task"
  - "area:tools"
  - "version:v0.88"
issue_number: 1647
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
  slug: "add-lightweight-workflow-conductor-skill"
---

# [v0.88][tools] Add lightweight workflow conductor skill

## Summary

Add a thin workflow conductor skill that routes into the existing ADL lifecycle and editor skills instead of replacing them. The conductor should reduce operator memory burden, enforce skill/subagent policy, and detect where a partially prepared issue should resume in the process.

## Goal

Make skill orchestration itself a first-class ADL workflow surface so operators can invoke one bounded conductor that chooses the right next skill and picks up from the correct lifecycle point.

## Required Outcome

The repository has a lightweight `workflow-conductor` skill that can inspect issue/workflow state, determine the correct next phase, route into the matching lifecycle or card-editor skill, and record workflow-compliance facts without duplicating the underlying skill logic.

## Deliverables

- a new `workflow-conductor` skill bundle under `adl/tools/skills/`
- a structured input schema for conductor invocation
- operator-facing docs in the operational skills guide
- tests covering routing, editor-skill selection, subagent policy handling, and resume-from-partial-state behavior
- bounded workflow-compliance output/recording guidance

## Acceptance Criteria

- The conductor can detect whether bootstrap, readiness, execution, finish, janitor, or closeout steps have already been completed and select the correct next skill.
- The conductor routes STP/SIP/SOR work to the matching editor skill rather than silently editing cards ad hoc.
- The conductor supports explicit skill/subagent execution policy and records workflow-compliance outcomes.
- The conductor remains thin: it orchestrates existing skills rather than duplicating their implementation logic.
- The conductor can resume from partially completed initial steps instead of assuming every issue always starts from bootstrap.
- The skill bundle includes schema/docs/tests comparable to the other operational skills.

## Repo Inputs

- `adl/tools/skills/`
- `adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`
- `.adl/docs/TBD/ADL_EXECUTION_POLICY_FOR_SKILLS_AND_SUBAGENTS.md`
- `.adl/docs/TBD/LIGHTWEIGHT_WORKFLOW_CONDUCTOR_SKILL.md`
- existing `pr-*` and card-editor skills

## Dependencies

- Existing `pr-init`, `pr-ready`, `pr-run`, `pr-finish`, `pr-janitor`, `pr-closeout`, `stp-editor`, `sip-editor`, and `sor-editor` skills remain the authoritative implementations the conductor must call.

## Demo Expectations

No standalone public demo is required. Proof is the conductor skill contract, routing tests, and operator-facing usage examples.

## Non-goals

- replacing the existing lifecycle or editor skills
- inventing a second hidden workflow engine
- silently widening issue scope or bypassing editor skills
- forcing every phase to use a subagent when policy does not require it

## Issue-Graph Notes

- This work should stay aligned with the planned v0.88 issue-preparation wave.
- The conductor should make it easier to resume workflow from the right point rather than assuming every issue always starts from bootstrap.

## Notes

Use the normal ADL PR lifecycle for this issue. The implementation should prefer thin orchestration, explicit policy, and auditable routing over large new control-plane complexity.

## Tooling Notes

The conductor should be able to detect partially completed initial steps and choose the next appropriate lifecycle or editor skill rather than restarting the issue from the beginning.
