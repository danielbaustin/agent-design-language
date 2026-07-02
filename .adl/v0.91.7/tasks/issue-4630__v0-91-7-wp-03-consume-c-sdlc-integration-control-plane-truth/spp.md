---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth-execution-plan"
issue: 4630
task_id: "issue-4630"
run_id: "issue-4630"
version: "v0.91.7"
title: "[v0.91.7][WP-03] Consume C-SDLC integration control-plane truth"
branch: "codex/4630-v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth"
generated_at: "2026-07-02T17:35:41Z"
card_status: "ready"
status: "ready"
activation_state: "ready"
plan_revision: 2
initial_pvf_lane: "prompt_template"
planned_pvf_lane: "tooling"
planned_pvf_lane_source: "planning_corrected_for_lifecycle_shepherd_implementation_slice"
estimate_elapsed_seconds: "5400"
estimate_total_tokens: "1000000"
estimate_validation_seconds: "600"
issue_goal_token_budget: "1000000"
variance_threshold_percent: "10"
estimate_confidence: "medium"
estimate_data_source: "manual_entry"
estimate_source_ref: "AGENTS.md"
issue_goal_ref: "issue-4630"
sprint_goal_ref: "unknown"
goal_metrics_rollup_ref: "unknown"
source_refs:
  - kind: "issue"
    ref: "https://github.com/danielbaustin/agent-design-language/issues/4630"
  - kind: "source_issue_prompt"
    ref: ".adl/v0.91.7/bodies/issue-4630-v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth.md"
  - kind: "stp"
    ref: ".adl/v0.91.7/tasks/issue-4630__v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth/stp.md"
  - kind: "sip"
    ref: ".adl/v0.91.7/tasks/issue-4630__v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth/sip.md"
scope:
  files:
    - "adl/src/cli/pr_cmd.rs"
    - "adl/src/cli/pr_cmd_args.rs"
    - "adl/src/cli/pr_cmd/github.rs"
    - "adl/tools/run_v0913_proof_validation_lane.sh"
    - "adl/tools/pr.sh"
    - "adl/tools/pr_delegate.sh"
    - "adl/tools/pr_usage.sh"
    - "docs/milestones/v0.91.3/review/card_lifecycle_integration/CARD_LIFECYCLE_PROOF_PACKET_v0.91.3.md"
    - "docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/spp.md"
    - "docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/sor.md"
    - "docs/tooling/ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md"
  components:
    - "v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth"
  out_of_scope:
    - "Do not implement unrelated WPs here.; Do not create hundreds of child issues.; Do not start v0.92 implementation.; Do not use raw gh."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Issue-local execution plan for [v0.91.7][WP-03] Consume C-SDLC integration control-plane truth."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm that the closed v0.91.6 shepherd line (#4235, #4436, #4443) did not produce a first-class full-lifecycle command surface."
    expected_output: ".adl/v0.91.7/tasks/issue-4630__v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo-native watcher, doctor, validation, finish, janitor, closeout, and session-ledger surfaces before editing."
    expected_output: ".adl/v0.91.7/tasks/issue-4630__v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement the smallest first-class `adl-pr-shepherd` owner binary that composes existing doctor/watch/validation routing and emits deterministic JSON state without gaining merge or close authority."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused tooling proof for shepherd routing and JSON output, then repair any blocker-driven PR-fast proof-lane overbreadth with exact retained-proof contract tests and matching replay-surface updates."
    expected_output: "validation evidence recorded in VPP/SOR"
    allowed_mode: "execution_after_approval"
  - id: "step-5"
    description: "Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges."
    expected_output: "reviewed SRP and truthful VPP/SOR"
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "Confirm prior shepherd/watcher work and current command surfaces."
    status: "completed"
  - step: "Inspect doctor/watch/validation/session-ledger routing before editing."
    status: "completed"
  - step: "Implement the bounded `adl-pr-shepherd` owner-binary surface only."
    status: "completed"
  - step: "Run focused validation and proof gates."
    status: "completed"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "completed"
affected_areas:
  - "v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Generated card may need editor tightening if the source issue prompt is underspecified."
test_strategy:
  - "Focused `adl-pr-shepherd` command tests plus git diff --check."
  - "No broad nextest lane unless the implementation touches shared command parsing beyond shepherd routing."
execution_handoff: "Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges."
required_permissions:
  - "workspace-write after execution approval"
stop_conditions:
  - "Stop and re-plan if dependencies are unmet or materially different from this design-time plan."
  - "Stop and update SPP if touched files, proof gates, or validation commands change materially."
  - "Stop and route follow-on work if acceptance requires scope outside this issue."
alternatives_considered:
  - description: "Rely only on transient chat planning."
    reason_not_chosen: "Chat-only planning is not durable or reviewable enough for this workflow surface."
review_hooks:
  - "Check dependency truth, scope truthfulness, touched-file truthfulness, validation sufficiency, and re-plan triggers."
notes: "Execution narrowed this broad WP consumption issue to the first-class `adl-pr-shepherd` owner binary above existing doctor/watch/janitor/closeout surfaces. That implementation slice is now in place, then a blocker-driven janitor repair narrowed the retained v0.91.3 proof-validation lane to exact `--bin adl` card-lifecycle tests after CI disk exhaustion proved the older broad commands over-compiled the tree. The plan now tracks both the implementation files and the ancillary retained proof surfaces that had to move with that repair."
---

Canonical Template Source: `docs/templates/prompts/1.0.3/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.91.7][WP-03] Consume C-SDLC integration control-plane truth`.

Issue-local execution plan for [v0.91.7][WP-03] Consume C-SDLC integration control-plane truth.

## PVF Lane Plan

- Initial PVF lane from issue creation: `prompt_template`
- Planned PVF lane for execution: `tooling`
- Planning lane source: `planning_confirmed_from_configured_policy_from_title_labels_and_body_inference`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `5400`
- Estimated total tokens: `1000000`
- Estimated validation seconds: `600`
- Issue goal token budget: `1000000`
- Variance threshold percent: `10`
- Estimate confidence: `medium`
- Estimate data source: `manual_entry`
- Estimate source ref: `AGENTS.md`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [completed] Confirm prior shepherd/watcher work and current command surfaces.
2. [completed] Inspect doctor/watch/validation/session-ledger routing before editing.
3. [completed] Implement the bounded shepherd command surface only.
4. [completed] Run focused validation and proof gates, including blocker-driven retained-proof lane repair when CI proved the original contract commands over-broad.
5. [completed] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm that the closed v0.91.6 shepherd line (#4235, #4436, #4443) did not produce a first-class full-lifecycle command surface.
2. Review repo-native watcher, doctor, validation, finish, janitor, closeout, and session-ledger surfaces before editing.
3. Implement the smallest first-class `adl-pr-shepherd` owner binary that composes existing doctor/watch/validation routing and emits deterministic JSON state without gaining merge or close authority.
4. Run focused tooling proof for shepherd routing and JSON output; keep broad Rust validation deferred unless touched code requires it.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Generated card may need editor tightening if the source issue prompt is underspecified.

## Test Strategy

- Focused `adl-pr-shepherd` command tests plus git diff --check.
- No broad nextest lane unless the implementation touches shared command parsing beyond shepherd routing.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Generated from 1.0.3 template; implementation and branch-refresh reconciliation are complete, and the plan now accounts for the blocker-driven retained-proof lane repair that was required after PR publication. The remaining execution path is truthful PR-tail shepherding until checks settle and closeout can continue.
