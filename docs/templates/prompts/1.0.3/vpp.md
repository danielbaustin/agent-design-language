---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "<slug>-validation-plan"
issue: <issue>
task_id: "issue-<issue_padded>"
run_id: "issue-<issue_padded>"
version: "<version>"
title: "<title>"
branch: "<branch>"
generated_at: "<timestamp>"
card_status: "<card_status>"
status: "<status>"
initial_pvf_lane: "<initial_pvf_lane>"
planned_pvf_lane: "<planned_pvf_lane>"
lane_registry_path: "<lane_registry_path>"
lane_registry_template_set: "<lane_registry_template_set>"
validation_runtime_class: "<validation_runtime_class>"
validation_resource_profile: "<validation_resource_profile>"
expected_proof_cost: "<expected_proof_cost>"
planned_validation_seconds: "<planned_validation_seconds>"
planned_validation_tokens: "<planned_validation_tokens>"
issue_goal_ref: "<issue_goal_ref>"
sprint_goal_ref: "<sprint_goal_ref>"
goal_metrics_rollup_ref: "<goal_metrics_rollup_ref>"
source_refs:
  - kind: "issue"
    ref: "<issue_url>"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
  - kind: "spp"
    ref: "<spp_card>"
selected_lanes:
  - "<selected_lanes_inline>"
parallel_groups:
  - "<parallel_groups_inline>"
validation_commands:
  - "<validation_commands_inline>"
failure_policy: "<failure_policy>"
notes: "<notes_risks_inline>"
---

Canonical Template Source: `docs/templates/prompts/1.0.3/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

<plan_summary>

## Lane Registry Inputs

- Registry path: `<lane_registry_path>`
- Registry template set: `<lane_registry_template_set>`
- Initial PVF lane from issue creation: `<initial_pvf_lane>`
- Planned PVF lane for execution: `<planned_pvf_lane>`

## Selected Validation Lanes

- <selected_lanes_inline>

## Parallelization Plan

- Parallel groups: <parallel_groups_inline>
- Validation runtime class: `<validation_runtime_class>`
- Validation resource profile: `<validation_resource_profile>`

## Goal Accounting Hooks

- Issue goal ref: `<issue_goal_ref>`
- Sprint goal ref: `<sprint_goal_ref>`
- Goal metrics rollup ref: `<goal_metrics_rollup_ref>`

## Proof Cost / Runtime Expectations

- Expected proof cost: `<expected_proof_cost>`
- Planned validation seconds: `<planned_validation_seconds>`
- Planned validation tokens: `<planned_validation_tokens>`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- <validation_commands_inline>

## Failure Semantics

- <failure_policy>

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

<notes_risks_inline>
