---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0-91-3-wp-03-card-lifecycle-demo-validation-plan"
issue: 3201
task_id: "issue-3201"
run_id: "issue-3201"
version: "v0.91.3"
title: "[v0.91.3][wp-03][cards] Card lifecycle demo"
branch: "main"
generated_at: "2026-06-22T00:00:00Z"
card_status: "ready"
status: "reviewed"
initial_pvf_lane: "docs"
planned_pvf_lane: "docs"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "vpp.lane.v1"
validation_runtime_class: "tiny"
validation_resource_profile: "local"
expected_proof_cost: "small"
planned_validation_seconds: "unknown"
planned_validation_tokens: "unknown"
issue_goal_ref: "issue-3201"
sprint_goal_ref: "unknown"
goal_metrics_rollup_ref: "unknown"
source_refs:
  - kind: "stp"
    ref: "docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/stp.md"
  - kind: "sip"
    ref: "docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/sip.md"
  - kind: "spp"
    ref: "docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/spp.md"
selected_lanes:
  - "docs"
parallel_groups:
  - "local"
validation_commands:
  - "bash adl/tools/validate_structured_prompt.sh --type sip --phase bootstrap --input docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/sip.md"
  - "bash adl/tools/validate_structured_prompt.sh --type stp --input docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/stp.md"
  - "bash adl/tools/validate_structured_prompt.sh --type spp --phase final --input docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/spp.md"
  - "bash adl/tools/validate_structured_prompt.sh --type srp --phase final --input docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/srp.md"
  - "bash adl/tools/validate_structured_prompt.sh --type sor --phase final --input docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/sor.md"
failure_policy: "fail_closed"
notes: "Retained VPP added when VPP became a first-class lifecycle surface so the historical card-lifecycle demo remains structurally complete."
---

Canonical Template Source: `docs/templates/prompts/1.0.3/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Historical retained VPP for the `v0.91.3` card-lifecycle demo packet.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `vpp.lane.v1`
- Initial PVF lane from issue creation: `docs`
- Planned PVF lane for execution: `docs`

## Selected Validation Lanes

- docs

## Parallelization Plan

- Parallel groups: local
- Validation runtime class: `tiny`
- Validation resource profile: `local`

## Goal Accounting Hooks

- Issue goal ref: `issue-3201`
- Sprint goal ref: `unknown`
- Goal metrics rollup ref: `unknown`

## Proof Cost / Runtime Expectations

- Expected proof cost: `small`
- Planned validation seconds: `unknown`
- Planned validation tokens: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- bash adl/tools/validate_structured_prompt.sh --type sip --phase bootstrap --input docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/sip.md
- bash adl/tools/validate_structured_prompt.sh --type stp --input docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/stp.md
- bash adl/tools/validate_structured_prompt.sh --type spp --phase final --input docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/spp.md
- bash adl/tools/validate_structured_prompt.sh --type srp --phase final --input docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/srp.md
- bash adl/tools/validate_structured_prompt.sh --type sor --phase final --input docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/sor.md

## Failure Semantics

- fail_closed

## Handoff

This retained VPP records the validation-planning truth needed for the historical lifecycle demo after VPP became a required stage.

## Notes

Retained VPP added when VPP became a first-class lifecycle surface so the historical card-lifecycle demo remains structurally complete.
