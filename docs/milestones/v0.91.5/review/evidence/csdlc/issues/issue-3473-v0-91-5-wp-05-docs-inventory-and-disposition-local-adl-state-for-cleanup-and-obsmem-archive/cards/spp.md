---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive-execution-plan"
issue: 3473
task_id: "issue-3473"
run_id: "issue-3473"
version: "v0.91.5"
title: "[v0.91.5][WP-05][docs] Inventory and disposition local ADL state for cleanup and ObsMem archive"
branch: "codex/3473-v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive"
generated_at: "2026-06-04T20:59:18Z"
card_status: "ready"
status: "approved"
activation_state: "ready_for_execution"
plan_revision: 1
source_refs:
  - kind: "issue"
    ref: "https://github.com/danielbaustin/agent-design-language/issues/3473"
  - kind: "source_issue_prompt"
    ref: ".adl/v0.91.5/bodies/issue-3473-v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive.md"
  - kind: "stp"
    ref: ".adl/v0.91.5/tasks/issue-3473__v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive/stp.md"
  - kind: "sip"
    ref: ".adl/v0.91.5/tasks/issue-3473__v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive/sip.md"
scope:
  files:
    - ".adl/ local inventory"
    - "docs/planning/TBD_CLEANUP_DISPOSITION_v0.91.2_3150.md"
    - "docs/planning/TBD_PLAN_ALLOCATION_v0.91.2_TO_v0.95.md"
    - "docs/milestones/v0.91.4/C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md"
    - "docs/milestones/v0.91.4/features/ACTIVE_ISSUE_MIGRATION_POLICY.md"
  components:
    - "v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive"
  out_of_scope:
    - "Do not delete `.adl` content in this issue unless explicitly widened after review."
    - "Do not track `.adl/` directly."
    - "Do not ingest records into ObsMem directly."
    - "Do not attempt full historical archaeology of every card or run."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "high"
plan_summary: "Inventory local `.adl` state and publish a tracked, non-destructive disposition matrix for archive, cleanup, promotion, and blocked/sensitive categories."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
  - "The current local `.adl` tree is operator-local evidence, not public canonical truth."
  - "This issue may name future deletion/archive candidates but must not delete them."
proposed_steps:
  - id: "step-1"
    description: "Confirm starting state, including that `#3472` has produced the first exporter PR and that this issue remains non-destructive."
    expected_output: "execution readiness note in SOR"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Run read-only shell inventory over `.adl/` top-level directories and the named high-value/high-risk subtrees."
    expected_output: "repo-relative inventory evidence"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Write the tracked disposition matrix and archive/deletion sequencing plan without moving or deleting local `.adl` content."
    expected_output: "tracked disposition document"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused docs validation, YAML/Markdown hygiene where applicable, and redaction scans over the tracked output."
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
  - "local `.adl` inventory evidence"
  - "v0.91.5 public prompt records planning docs"
  - "tracked cleanup/archive disposition document"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep SRP as review-result truth and SOR as output truth."
risks_and_edge_cases:
  - "Local `.adl` state may contain host paths, private logs, or credentials and must not be copied wholesale."
  - "Inventory commands could accidentally expose absolute paths if recorded raw; tracked output must use repo-relative categories."
  - "Historical cards and review packets may have high provenance value even if they are local-only today."
test_strategy:
  - "Use read-only shell inventory commands only."
  - "Run `git diff --check`."
  - "Run focused redaction scan over the tracked output."
  - "Validate Markdown links if new tracked Markdown links are added."
execution_handoff: "Use this SPP as the design-time plan-of-record, then update it at runtime whenever the actual execution sequence changes."
required_permissions:
  - "workspace-write after execution approval"
stop_conditions:
  - "Stop and re-plan if dependencies are unmet or materially different from this design-time plan."
  - "Stop and update SPP if touched files, proof gates, or validation commands change materially."
  - "Stop and route follow-on work if acceptance requires scope outside this issue."
  - "Stop immediately before any delete, move, archive, or ingestion operation."
alternatives_considered:
  - description: "Rely only on transient chat planning."
    reason_not_chosen: "Chat-only planning is not durable or reviewable enough for this workflow surface."
review_hooks:
  - "Check dependency truth, scope truthfulness, touched-file truthfulness, validation sufficiency, and re-plan triggers."
notes: "SPP editor normalized for non-destructive execution readiness before binding."
---

Canonical Template Source: `docs/templates/prompts/1.0.0/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.91.5][WP-05][docs] Inventory and disposition local ADL state for cleanup and ObsMem archive`.

Inventory local `.adl` state and publish a tracked, non-destructive disposition matrix for archive, cleanup, promotion, and blocked/sensitive categories.

## Codex Plan

1. [pending] Confirm dependencies and starting state from the source issue prompt.
2. [pending] Inspect repo inputs and target surfaces before editing.
3. [pending] Implement the bounded deliverables only.
4. [pending] Run focused validation and proof gates.
5. [pending] Record issue-specific SRP findings and SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.
- The current local `.adl` tree is operator-local evidence, not public canonical truth.
- This issue may name future deletion/archive candidates but must not delete them.

## Proposed Steps

1. Confirm starting state, including that `#3472` has produced the first exporter PR and that this issue remains non-destructive.
2. Run read-only shell inventory over `.adl/` top-level directories and the named high-value/high-risk subtrees.
3. Write the tracked disposition matrix and archive/deletion sequencing plan without moving or deleting local `.adl` content.
4. Run focused docs validation, YAML/Markdown hygiene where applicable, and redaction scans over the tracked output.
5. Record issue-specific review findings in SRP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- local `.adl` inventory evidence
- v0.91.5 public prompt records planning docs
- tracked cleanup/archive disposition document

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep SRP as review-result truth and SOR as output truth.

## Risks And Edge Cases

- Local `.adl` state may contain host paths, private logs, or credentials and must not be copied wholesale.
- Inventory commands could accidentally expose absolute paths if recorded raw; tracked output must use repo-relative categories.
- Historical cards and review packets may have high provenance value even if they are local-only today.

## Test Strategy

- Use read-only shell inventory commands only.
- Run `git diff --check`.
- Run focused redaction scan over the tracked output.
- Validate Markdown links if new tracked Markdown links are added.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then update it at runtime whenever the actual execution sequence changes.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.
- Stop immediately before any delete, move, archive, or ingestion operation.

## Notes

SPP editor normalized for non-destructive execution readiness before binding.
