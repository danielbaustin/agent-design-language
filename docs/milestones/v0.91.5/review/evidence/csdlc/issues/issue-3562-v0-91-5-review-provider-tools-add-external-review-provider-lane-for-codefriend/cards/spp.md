---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend-execution-plan"
issue: 3562
task_id: "issue-3562"
run_id: "issue-3562"
version: "v0.91.5"
title: "[v0.91.5][review-provider][tools] Add external review provider lane for CodeFriend"
branch: "codex/3562-v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend"
generated_at: "2026-06-01T03:18:59Z"
card_status: "ready"
status: "reviewed"
activation_state: "ready_for_execution_binding"
plan_revision: 2
source_refs:
  - kind: "issue"
    ref: "https://github.com/danielbaustin/agent-design-language/issues/3562"
  - kind: "source_issue_prompt"
    ref: ".adl/v0.91.5/bodies/issue-3562-v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend.md"
  - kind: "stp"
    ref: ".adl/v0.91.5/tasks/issue-3562__v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend/stp.md"
  - kind: "sip"
    ref: ".adl/v0.91.5/tasks/issue-3562__v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend/sip.md"
scope:
  files:
    - "`#3549` native DeepSeek API provider issue; Existing provider communication substrate; Existing podcast/multi-agent provider setup; Existing CodeFriend review packet expectations; Existing ADL review skills and internal-review artifact shapes"
  components:
    - "v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend"
  out_of_scope:
    - "Do not replace native Codex subagents.; Do not make external provider output authoritative without synthesis/review.; Do not require all providers to be implemented in the first slice.; Do not expose credentials or raw provider logs in tracked artifacts."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Issue-local execution plan for [v0.91.5][review-provider][tools] Add external review provider lane for CodeFriend."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Depends on `#3549` before implementation work that requires DeepSeek provider coverage.; Can start with design before `#3549` closes, but execution should not claim full provider coverage until DeepSeek is available."
    expected_output: ".adl/v0.91.5/tasks/issue-3562__v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: `#3549` native DeepSeek API provider issue; Existing provider communication substrate; Existing podcast/multi-agent provider setup; Existing CodeFriend review packet expectations; Existing ADL review skills and internal-review artifact shapes"
    expected_output: ".adl/v0.91.5/tasks/issue-3562__v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: `ReviewProviderV1` design and contract covering request, provider route, result, run record, logs, and provenance.; A CLI or tool entrypoint proposal for running one bounded review lane from an issue, diff, file set, and rubric.; Provider sequencing plan that depends on `#3549` for native DeepSeek support and reuses existing hosted/local provider paths where possible.; CodeFriend integration notes describing how review-provider outputs become findings packets rather than authority claims.; Follow-on implementation slices if the issue proves too large for one PR."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: The issue is explicitly sequenced after `#3549`.; The contract distinguishes native Codex subagents from external provider-backed reviewer agents.; The design supports Claude, Gemini, OpenAI, Ollama, and DeepSeek without hard-coding one provider as special.; Outputs include model identity, timestamps, elapsed time, request identity, log location, redaction status, and review-result status.; The design includes fail-closed behavior for provider errors, empty outputs, timeout, auth failure, and malformed findings.; CodeFriend can consume the result as a review artifact with findings-first synthesis."
    expected_output: "validation evidence recorded in SOR"
    allowed_mode: "execution_after_approval"
  - id: "step-5"
    description: "Record issue-specific review findings in SRP, issue outcome truth in SOR, and refresh this SPP if execution diverges."
    expected_output: "reviewed SRP and truthful SOR"
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "Confirm dependencies and starting state from the source issue prompt."
    status: "completed"
  - step: "Inspect repo inputs and target surfaces before editing."
    status: "completed"
  - step: "Implement the bounded deliverables only."
    status: "completed"
  - step: "Run focused validation and proof gates."
    status: "completed"
  - step: "Record issue-specific SRP findings and SOR outcome truth."
    status: "completed"
affected_areas:
  - "v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep SRP as review-result truth and SOR as output truth."
risks_and_edge_cases:
  - "Generated card may need editor tightening if the source issue prompt is underspecified."
test_strategy:
  - "Use the normal ADL issue workflow. Keep credentials external, redact logs by default, and capture model identity using the emerging model identity contract."
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

Design-time operative plan for `[v0.91.5][review-provider][tools] Add external review provider lane for CodeFriend`.

Issue-local execution plan for [v0.91.5][review-provider][tools] Add external review provider lane for CodeFriend.

## Codex Plan

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [completed] Inspect repo inputs and target surfaces before editing.
3. [completed] Implement the bounded deliverables only.
4. [completed] Run focused validation and proof gates.
5. [completed] Record issue-specific SRP findings and SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: Depends on `#3549` before implementation work that requires DeepSeek provider coverage.; Can start with design before `#3549` closes, but execution should not claim full provider coverage until DeepSeek is available.
2. Review repo inputs and scoped surfaces before editing: `#3549` native DeepSeek API provider issue; Existing provider communication substrate; Existing podcast/multi-agent provider setup; Existing CodeFriend review packet expectations; Existing ADL review skills and internal-review artifact shapes
3. Implement only the bounded deliverables: `ReviewProviderV1` design and contract covering request, provider route, result, run record, logs, and provenance.; A CLI or tool entrypoint proposal for running one bounded review lane from an issue, diff, file set, and rubric.; Provider sequencing plan that depends on `#3549` for native DeepSeek support and reuses existing hosted/local provider paths where possible.; CodeFriend integration notes describing how review-provider outputs become findings packets rather than authority claims.; Follow-on implementation slices if the issue proves too large for one PR.
4. Run focused proof gates for acceptance: The issue is explicitly sequenced after `#3549`.; The contract distinguishes native Codex subagents from external provider-backed reviewer agents.; The design supports Claude, Gemini, OpenAI, Ollama, and DeepSeek without hard-coding one provider as special.; Outputs include model identity, timestamps, elapsed time, request identity, log location, redaction status, and review-result status.; The design includes fail-closed behavior for provider errors, empty outputs, timeout, auth failure, and malformed findings.; CodeFriend can consume the result as a review artifact with findings-first synthesis.
5. Record issue-specific review findings in SRP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep SRP as review-result truth and SOR as output truth.

## Risks And Edge Cases

- Generated card may need editor tightening if the source issue prompt is underspecified.

## Test Strategy

- Use the normal ADL issue workflow. Keep credentials external, redact logs by default, and capture model identity using the emerging model identity contract.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then update it at runtime whenever the actual execution sequence changes.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Generated from 1.0.0 template; update before continuing if execution diverges.
