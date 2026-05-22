---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "v0-91-3-wp-03-card-lifecycle-demo-plan"
issue: 3201
task_id: "issue-3201"
run_id: "issue-3201"
version: "v0.91.3"
title: "[v0.91.3][WP-03][tools] Card lifecycle integration"
branch: "codex/3201-v0-91-3-wp-03-card-lifecycle-integration"
lifecycle_stage: "SPP"
status: "approved"
activation_state: "active"
plan_revision: 2
source_refs:
  - kind: "issue"
    ref: "https://github.com/danielbaustin/agent-design-language/issues/3201"
  - kind: "stp"
    ref: "workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/stp.md"
  - kind: "sip"
    ref: "workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/sip.md"
scope:
  files:
    - "workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/"
    - "adl/src/cli/tooling_cmd/tests/structured_prompt.rs"
    - "adl/src/cli/pr_cmd/doctor.rs"
  components:
    - "tracked public card bundle"
    - "card validator proof"
    - "doctor lifecycle proof"
  out_of_scope:
    - "full v0.91.4 lifecycle rollout"
constraints:
  - "no_hidden_scope_expansion"
  - "keep SPP issue-local and public for this tracked proof"
confidence: "medium"
plan_summary: "Active WP-03 plan: prove the canonical card lifecycle with one tracked public issue bundle plus focused validator and doctor expectations."
assumptions:
  - "WP-02 already landed the first transition manifest slice."
  - "The public proof bundle complements rather than replaces local issue bundles."
proposed_steps:
  - id: "step-1"
    description: "Create the tracked public card bundle under workflow/c-sdlc/v0.91.3/issues/."
    expected_output: "Tracked SIP/STP/SPP/SRP/SOR proof cards."
    allowed_mode: "execution_after_approval"
  - id: "step-2"
    description: "Back the bundle with focused structured-prompt validator tests."
    expected_output: "Validator-backed proof for all five tracked cards."
    allowed_mode: "execution_after_approval"
  - id: "step-3"
    description: "Back the bundle with doctor lifecycle expectations for final review/output truth."
    expected_output: "Doctor-backed proof for active-stage and readiness classification."
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "Create the tracked public card bundle."
    status: "completed"
  - step: "Back the bundle with structured-prompt validator tests."
    status: "completed"
  - step: "Back the bundle with doctor lifecycle expectations."
    status: "completed"
  - step: "Run focused validation and record truthful output state."
    status: "completed"
affected_areas:
  - "workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/"
  - "adl/src/cli/tooling_cmd/tests/structured_prompt.rs"
  - "adl/src/cli/pr_cmd/doctor.rs"
invariants_to_preserve:
  - "Keep the canonical issue-local order SIP -> STP -> SPP -> SRP -> SOR."
  - "Do not collapse SRP review truth into SOR outcome truth."
  - "Keep the tracked proof bundle repo-relative and public."
risks_and_edge_cases:
  - "The tracked bundle could accidentally depend on local-only .adl assumptions."
  - "Doctor expectations could pass on temp fixtures while drifting from the public bundle."
test_strategy:
  - "cargo test --manifest-path adl/Cargo.toml tracked_csdlc_card_bundle -- --nocapture"
  - "cargo test --manifest-path adl/Cargo.toml card_lifecycle_accepts_tracked_csdlc_bundle -- --nocapture"
  - "bash adl/tools/validate_structured_prompt.sh --type sip --phase bootstrap --input workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/sip.md"
  - "bash adl/tools/validate_structured_prompt.sh --type stp --input workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/stp.md"
  - "bash adl/tools/validate_structured_prompt.sh --type spp --phase final --input workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/spp.md"
  - "bash adl/tools/validate_structured_prompt.sh --type srp --phase final --input workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/srp.md"
  - "bash adl/tools/validate_structured_prompt.sh --type sor --phase final --input workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/sor.md"
execution_handoff: "Use this artifact as the public issue-local plan proof for WP-03."
required_permissions:
  - "workspace-write"
stop_conditions:
  - "Stop and re-plan if WP-03 needs default-operation lifecycle migration outside the tracked proof bundle."
alternatives_considered:
  - description: "Keep the lifecycle proof local-only under .adl bundles."
    reason_not_chosen: "WP-03 explicitly requires a tracked, public, auditable proof surface."
review_hooks:
  - "Check that the tracked bundle validates directly and that doctor expectations match it."
notes: "Tracked public SPP proof surface for the first C-SDLC card lifecycle slice."
---

# Structured Plan Prompt

## Plan Summary

Active WP-03 plan: prove the canonical card lifecycle with one tracked public
issue bundle plus focused validator and doctor expectations.

## Codex Plan

1. [completed] Create the tracked public card bundle.
2. [completed] Back the bundle with structured-prompt validator tests.
3. [completed] Back the bundle with doctor lifecycle expectations.
4. [completed] Run focused validation and record truthful output state.

## Assumptions

- WP-02 already landed the first transition manifest slice.
- The public proof bundle complements rather than replaces local issue bundles.

## Proposed Steps

1. Create the tracked public card bundle under `workflow/c-sdlc/v0.91.3/issues/`.
2. Back the bundle with focused structured-prompt validator tests.
3. Back the bundle with doctor lifecycle expectations for final review/output truth.

## Affected Areas

- `workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/`
- `adl/src/cli/tooling_cmd/tests/structured_prompt.rs`
- `adl/src/cli/pr_cmd/doctor.rs`

## Invariants To Preserve

- Keep the canonical issue-local order `SIP -> STP -> SPP -> SRP -> SOR`.
- Do not collapse `SRP` review truth into `SOR` outcome truth.
- Keep the tracked proof bundle repo-relative and public.

## Risks And Edge Cases

- The tracked bundle could accidentally depend on local-only `.adl` assumptions.
- Doctor expectations could pass on temp fixtures while drifting from the public bundle.

## Test Strategy

- `cargo test --manifest-path adl/Cargo.toml tracked_csdlc_card_bundle -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml card_lifecycle_accepts_tracked_csdlc_bundle -- --nocapture`
- `bash adl/tools/validate_structured_prompt.sh --type sip --phase bootstrap --input workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/sip.md`
- `bash adl/tools/validate_structured_prompt.sh --type stp --input workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/stp.md`
- `bash adl/tools/validate_structured_prompt.sh --type spp --phase final --input workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/spp.md`
- `bash adl/tools/validate_structured_prompt.sh --type srp --phase final --input workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/srp.md`
- `bash adl/tools/validate_structured_prompt.sh --type sor --phase final --input workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/sor.md`

## Execution Handoff

Use this artifact as the public issue-local plan proof for WP-03.

## Stop Conditions

- Stop and re-plan if WP-03 needs default-operation lifecycle migration outside the tracked proof bundle.

## Notes

Tracked public SPP proof surface for the first C-SDLC card lifecycle slice.
