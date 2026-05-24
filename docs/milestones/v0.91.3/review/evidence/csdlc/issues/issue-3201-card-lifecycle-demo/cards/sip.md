# ADL Input Card

Task ID: issue-3201
Run ID: issue-3201
Version: v0.91.3
Title: [v0.91.3][WP-03][tools] Card lifecycle integration
Branch: codex/3201-v0-91-3-wp-03-card-lifecycle-integration

Context:
- Issue: https://github.com/danielbaustin/agent-design-language/issues/3201
- PR: https://github.com/danielbaustin/agent-design-language/pull/3236
- Source Issue Prompt: .adl/v0.91.3/bodies/issue-3201-v0-91-3-wp-03-card-lifecycle-integration.md
- Docs: docs/cognitive-sdlc/card-lifecycle.md
- Other: docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/README.md

## Agent Execution Rules

- Do not run `pr start`; the branch and worktree already exist.
- Do not delete or recreate cards.
- Do not switch branches unless explicitly instructed.
- Do not work on `main`.
- Only modify files required for `WP-03`.
- Use repository-relative paths; avoid absolute host paths.
- Write the output record to the paired local task bundle `sor.md` path.
- If repository state is unexpected, stop and ask before attempting repository repair.

Task ID: issue-3201
Run ID: issue-3201
Version: v0.91.3
Title: [v0.91.3][WP-03][tools] Card lifecycle integration
Branch: codex/3201-v0-91-3-wp-03-card-lifecycle-integration
Status: DONE

## Goal

Make the corrected C-SDLC card sequence operational inside the first
Cognitive State Transition slice.

## Required Outcome

lifecycle validator and doctor expectations for the slice.

## Acceptance Criteria

- the work product satisfies the `WP-03` outcome in
  `docs/milestones/v0.91.3/WP_ISSUE_WAVE_v0.91.3.yaml`
- dependencies are respected: `WP-02`
- the implementation stays within the `tools` queue and does not absorb
  unrelated milestone work
- cards remain lifecycle-truthful and use editor skills for card changes
- validation is focused, reproducible, and recorded

## Inputs

- `AGENTS.md`
- `docs/cognitive-sdlc/card-lifecycle.md`
- `docs/tooling/structured-prompt-contracts.md`
- `adl/src/cli/pr_cmd/doctor.rs`
- `adl/src/cli/tooling_cmd/tests/structured_prompt.rs`

## Target Files / Surfaces

- `docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/`
- `docs/milestones/v0.91.3/features/CARD_LIFECYCLE_INTEGRATION.md`
- `docs/milestones/v0.91.3/DEMO_MATRIX_v0.91.3.md`
- `docs/milestones/v0.91.3/FEATURE_PROOF_COVERAGE_v0.91.3.md`

## Validation Plan

- validate the tracked `SIP`, `STP`, `SPP`, `SRP`, and `SOR` bundle directly
- prove the doctor lifecycle classifier accepts the tracked bundle as final
  review/output truth

## Demo / Proof Requirements

- tracked public card bundle in `docs/milestones/v0.91.3/review/evidence/csdlc/issues/`
- focused validator and doctor test proof

## Constraints / Policies

- no hidden scope expansion
- keep `SPP` issue-local
- do not collapse `SRP` and `SOR`

## System Invariants (must remain true)

- `SIP -> STP -> SPP -> SRP -> SOR` remains the canonical issue-local order
- durable C-SDLC card truth must be repo-tracked and public for this proof

## Reviewer Checklist (machine-readable hints)

- confirm the tracked bundle validates without local-only assumptions
- confirm doctor lifecycle expectations match the tracked bundle state

## Non-goals / Out of scope

- default-operation rollout for all ADL issues
- full transition DAG, shard-plan, signed-trace, or ObsMem implementation

## Notes / Risks

- the first proof bundle is intentionally narrow and does not replace local
  issue bundles for active issue execution

## Instructions to the Agent

- keep the work package bounded
- preserve repo-relative paths throughout the tracked bundle
- record real validation and review truth only

## Prompt Spec

```yaml
prompt_schema: adl.v1
actor:
  role: execution_agent
  name: codex
model:
  id: gpt-5-codex
  determinism_mode: stable
inputs:
  sections:
    - goal
    - required_outcome
    - acceptance_criteria
    - inputs
    - target_files_surfaces
    - validation_plan
    - demo_proof_requirements
    - constraints_policies
    - system_invariants
    - reviewer_checklist
    - non_goals_out_of_scope
    - notes_risks
    - instructions_to_agent
outputs:
  output_card: .adl/v0.91.3/tasks/issue-3201__v0-91-3-wp-03-card-lifecycle-integration/sor.md
  summary_style: concise_structured
constraints:
  include_system_invariants: true
  include_reviewer_checklist: true
  disallow_secrets: true
  disallow_absolute_host_paths: true
automation_hints:
  source_issue_prompt_required: true
  target_files_surfaces_recommended: true
  validation_plan_required: true
  required_outcome_type_supported: true
review_surfaces:
  - card_review_checklist.v1
  - card_review_output.v1
  - card_reviewer_gpt.v1.1
```

## Execution

- Agent: codex
- Model: gpt-5
- Provider: openai-codex
- Start Time: 2026-05-21T23:10:00Z
- End Time: 2026-05-21T23:18:00Z
