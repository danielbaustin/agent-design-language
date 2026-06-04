---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter-execution-plan"
issue: 3472
task_id: "issue-3472"
run_id: "issue-3472"
version: "v0.91.5"
title: "[v0.91.5][WP-04][tools] Add public C-SDLC prompt packet exporter"
branch: "codex/3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter"
generated_at: "2026-06-04T19:39:09Z"
card_status: "ready"
status: "reviewed"
activation_state: "ready"
plan_revision: 1
source_refs:
  - kind: "issue"
    ref: "https://github.com/danielbaustin/agent-design-language/issues/3472"
  - kind: "source_issue_prompt"
    ref: ".adl/v0.91.5/bodies/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter.md"
  - kind: "stp"
    ref: ".adl/v0.91.5/tasks/issue-3472__v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/stp.md"
  - kind: "sip"
    ref: ".adl/v0.91.5/tasks/issue-3472__v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/sip.md"
scope:
  files:
    - "`#3471`; `docs/milestones/v0.91.4/C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md`; `docs/planning/C_SDLC_PROMPT_TEMPLATE_EDITOR_TRANSITION_PLAN.md`; `docs/templates/prompts/current.json`; `adl/tools/pr.sh`; `adl/src/cli/pr_cmd*`; local `.adl/v0.91.4/tasks/` card bundles as source inputs only"
  components:
    - "v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter"
  out_of_scope:
    - "Do not bulk-export all historical cards.; Do not ingest into ObsMem directly.; Do not support every external tracker adapter in this issue.; Do not rewrite card content beyond safe redaction/sanitization required for public records."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Ready issue-local execution plan for [v0.91.5][WP-04][tools] Add public C-SDLC prompt packet exporter."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
  - "The original issue body references v0.91.4 because it predates the v0.91.5 bridge split; implement the exporter as version-aware and use v0.91.5 for this milestone's immediate public-card wave."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Depends on `#3471` for final packet contract, but can proceed with the draft namespace and manifest shape if explicitly recorded."
    expected_output: ".adl/v0.91.5/tasks/issue-3472__v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: `#3471`; `docs/milestones/v0.91.4/C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md`; `docs/planning/C_SDLC_PROMPT_TEMPLATE_EDITOR_TRANSITION_PLAN.md`; `docs/templates/prompts/current.json`; `adl/tools/pr.sh`; `adl/src/cli/pr_cmd*`; local `.adl/v0.91.4/tasks/` card bundles as source inputs only"
    expected_output: ".adl/v0.91.5/tasks/issue-3472__v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Exporter command or helper design for public prompt packets.; Initial implementation if small enough; otherwise an exact implementation follow-on with command shape and contract tests.; Public packet manifest fields for issue number, slug, template set, source refs, lifecycle state, validation state, redaction status, and tracker URL.; Rules for copying/sanitizing local cards into tracked packet records without treating `.adl` as canonical public truth.; Closeout integration notes for `pr finish` / `pr closeout`."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Export output uses repo-relative paths only.; Export does not add `.adl/` to Git.; Export refuses obvious secret markers, absolute host paths, private key filenames, and local scratch paths.; Export preserves template version and card lifecycle status.; Export distinguishes GitHub tracker identity from tracker-agnostic work-item identity so Jira or other adapters remain possible.; Focused tests or documented test plan prove the exporter contract."
    expected_output: "validation evidence recorded in SOR"
    allowed_mode: "execution_after_approval"
  - id: "step-5"
    description: "Record issue-specific review findings in SRP, issue outcome truth in SOR, and refresh this SPP if execution diverges."
    expected_output: "reviewed SRP and truthful SOR"
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "Confirm dependencies and starting state from the source issue prompt."
    status: "pending"
  - step: "Inspect repo inputs and target surfaces before editing."
    status: "pending"
  - step: "Implement the bounded deliverables only."
    status: "pending"
  - step: "Run focused validation and proof gates."
    status: "pending"
  - step: "Record issue-specific SRP findings and SOR outcome truth."
    status: "pending"
affected_areas:
  - "v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep SRP as review-result truth and SOR as output truth."
risks_and_edge_cases:
  - "Generated card may need editor tightening if the source issue prompt is underspecified."
test_strategy:
  - "Use `workflow-conductor` and the normal issue lifecycle.; Use focused tooling tests only unless Rust source changes require broader proof."
execution_handoff: "Use this SPP as the design-time plan-of-record, then update it at runtime whenever the actual execution sequence changes."
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
notes: "Generated from 1.0.0 template; update before continuing if execution diverges."
---

Canonical Template Source: `docs/templates/prompts/1.0.0/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.91.5][WP-04][tools] Add public C-SDLC prompt packet exporter`.

Issue-local execution plan for [v0.91.5][WP-04][tools] Add public C-SDLC prompt packet exporter.

## Codex Plan

1. [pending] Confirm dependencies and starting state from the source issue prompt.
2. [pending] Inspect repo inputs and target surfaces before editing.
3. [pending] Implement the bounded deliverables only.
4. [pending] Run focused validation and proof gates.
5. [pending] Record issue-specific SRP findings and SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: Depends on `#3471` for final packet contract, but can proceed with the draft namespace and manifest shape if explicitly recorded.
2. Review repo inputs and scoped surfaces before editing: `#3471`; `docs/milestones/v0.91.4/C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md`; `docs/planning/C_SDLC_PROMPT_TEMPLATE_EDITOR_TRANSITION_PLAN.md`; `docs/templates/prompts/current.json`; `adl/tools/pr.sh`; `adl/src/cli/pr_cmd*`; local `.adl/v0.91.4/tasks/` card bundles as source inputs only
3. Implement only the bounded deliverables: Exporter command or helper design for public prompt packets.; Initial implementation if small enough; otherwise an exact implementation follow-on with command shape and contract tests.; Public packet manifest fields for issue number, slug, template set, source refs, lifecycle state, validation state, redaction status, and tracker URL.; Rules for copying/sanitizing local cards into tracked packet records without treating `.adl` as canonical public truth.; Closeout integration notes for `pr finish` / `pr closeout`.
4. Run focused proof gates for acceptance: Export output uses repo-relative paths only.; Export does not add `.adl/` to Git.; Export refuses obvious secret markers, absolute host paths, private key filenames, and local scratch paths.; Export preserves template version and card lifecycle status.; Export distinguishes GitHub tracker identity from tracker-agnostic work-item identity so Jira or other adapters remain possible.; Focused tests or documented test plan prove the exporter contract.
5. Record issue-specific review findings in SRP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep SRP as review-result truth and SOR as output truth.

## Risks And Edge Cases

- Generated card may need editor tightening if the source issue prompt is underspecified.

## Test Strategy

- Use `workflow-conductor` and the normal issue lifecycle.; Use focused tooling tests only unless Rust source changes require broader proof.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then update it at runtime whenever the actual execution sequence changes.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Generated from 1.0.0 template; update before continuing if execution diverges.
