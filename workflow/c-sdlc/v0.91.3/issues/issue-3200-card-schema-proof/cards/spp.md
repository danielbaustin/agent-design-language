---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "v0-91-3-wp-02-cognitive-transition-schema-execution-plan"
issue: 3200
task_id: "issue-3200"
run_id: "issue-3200"
version: "v0.91.3"
title: "[v0.91.3][WP-02][docs/tools] Cognitive Transition schema"
branch: "codex/3200-v0-91-3-wp-02-cognitive-transition-schema"
lifecycle_stage: "SPP"
status: "approved"
activation_state: "active"
plan_revision: 2
source_refs:
  - kind: "issue"
    ref: "https://github.com/danielbaustin/agent-design-language/issues/3200"
  - kind: "stp"
    ref: ".adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/stp.md"
  - kind: "sip"
    ref: ".adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/sip.md"
  - kind: "doc"
    ref: "docs/cognitive-sdlc/transition-schema.md"
  - kind: "doc"
    ref: "docs/milestones/v0.91.3/features/COGNITIVE_TRANSITION_MANIFEST.md"
scope:
  files:
    - "adl/src/cognitive_transition_schema.rs"
    - "adl/src/lib.rs"
    - "docs/cognitive-sdlc/transition-schema.md"
    - "docs/milestones/v0.91.3/features/COGNITIVE_TRANSITION_MANIFEST.md"
    - "docs/milestones/v0.91.3/DEMO_MATRIX_v0.91.3.md"
    - "docs/milestones/v0.91.3/FEATURE_PROOF_COVERAGE_v0.91.3.md"
    - "docs/milestones/v0.91.3/review/transition_manifest/fixtures/valid_cognitive_transition_manifest_v1.json"
    - "docs/milestones/v0.91.3/review/transition_manifest/fixtures/invalid_cognitive_transition_manifest_v1_missing_seed_role.json"
    - ".adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/spp.md"
  components:
    - "cognitive transition manifest schema module"
    - "transition manifest fixture packet"
    - "v0.91.3 WP-02 milestone proof surfaces"
  out_of_scope:
    - "later transition DAG, shard-plan, signed-trace, or ObsMem work reserved for downstream v0.91.3 and v0.91.4 issues"
    - "full sprint orchestration or closeout work outside the issue-local WP-02 surface"
constraints:
  - "no_hidden_scope_expansion"
  - "keep the WP-02 slice bounded to the first manifest schema"
  - "keep proof claims tied to code, fixtures, and focused validation that actually exist"
confidence: "medium"
plan_summary: "Active execution plan for WP-02: add the first machine-checkable cognitive transition manifest schema, seed roles, lifecycle states, tracked fixtures, and proof-backed milestone docs without widening into later C-SDLC substrate work."
assumptions:
  - "WP-01 is already merged, so WP-02 can treat the v0.91.3 issue wave and milestone docs as the current canonical baseline."
  - "WP-02 only needs the first manifest schema slice; later issues can extend transition DAG, shard-plan, and signed-trace surfaces."
proposed_steps:
  - id: "step-1"
    description: "Confirm the v0.91.3 baseline, WP-01 dependency truth, and current schema/doc surfaces before editing."
    expected_output: "A bounded source inventory for the WP-02 transition manifest slice."
    allowed_mode: "execution_after_approval"
  - id: "step-2"
    description: "Implement the bounded schema module, validator, fixture helper, and tracked valid/invalid JSON fixtures."
    expected_output: "Tracked Rust and fixture surfaces for the first transition manifest schema."
    allowed_mode: "execution_after_approval"
  - id: "step-3"
    description: "Update the cognitive-sdlc and milestone proof/demo docs so they point at the new validator-backed manifest surfaces."
    expected_output: "Tracked docs aligned to the implemented WP-02 proof surfaces."
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for schema tests, fixture validity, formatting cleanliness, and diff hygiene."
    expected_output: "Validation evidence recorded in the SOR."
    allowed_mode: "execution_after_approval"
  - id: "step-5"
    description: "Record bounded pre-PR review, update the issue-local cards, and publish only after the workflow truth matches the implemented state."
    expected_output: "Review-ready issue state with truthful SPP/SRP/SOR surfaces."
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "Confirm the v0.91.3 baseline, WP-01 dependency truth, and current schema/doc surfaces."
    status: "completed"
  - step: "Implement the schema module, validator, fixture helper, and tracked JSON fixtures."
    status: "completed"
  - step: "Update cognitive-sdlc and milestone proof/demo docs to point at the new surfaces."
    status: "completed"
  - step: "Run focused schema/fixture/formatting proof gates."
    status: "completed"
  - step: "Record bounded review and truthful output state before PR publication."
    status: "in_progress"
affected_areas:
  - "adl/src/cognitive_transition_schema.rs"
  - "docs/cognitive-sdlc/transition-schema.md"
  - "docs/milestones/v0.91.3/features/COGNITIVE_TRANSITION_MANIFEST.md"
  - "docs/milestones/v0.91.3/review/transition_manifest/"
invariants_to_preserve:
  - "Keep the WP-02 slice bounded to the first manifest schema; do not absorb later transition substrate work."
  - "Keep repo-relative path discipline in fixtures and schema validation surfaces."
  - "Keep SRP as review-result truth and SOR as output truth."
risks_and_edge_cases:
  - "The first schema slice could overclaim later C-SDLC readiness if the docs are not explicit about bounded scope."
  - "Fixture paths or validator rules could drift away from the schema narrative if the tracked docs are not updated alongside the code."
test_strategy:
  - "`cargo test --manifest-path adl/Cargo.toml cognitive_transition_schema -- --nocapture`"
  - "`python3 -m json.tool docs/milestones/v0.91.3/review/transition_manifest/fixtures/valid_cognitive_transition_manifest_v1.json`"
  - "`python3 -m json.tool docs/milestones/v0.91.3/review/transition_manifest/fixtures/invalid_cognitive_transition_manifest_v1_missing_seed_role.json`"
  - "`cargo fmt --manifest-path adl/Cargo.toml --all --check`"
  - "`git diff --check`"
execution_handoff: "Use this artifact as the live issue-local plan while WP-02 moves from bound implementation to review-ready publication."
required_permissions:
  - "workspace-write"
stop_conditions:
  - "Stop and re-plan if WP-02 needs transition DAG, shard-plan, signed-trace, or ObsMem implementation to satisfy acceptance."
  - "Stop and update SPP if new proof gates are required beyond the focused schema/fixture/doc surface."
  - "Stop and route follow-on work if acceptance requires changes outside the bounded docs/tools queue."
alternatives_considered:
  - description: "Leave the manifest as docs-only guidance."
    reason_not_chosen: "WP-02 specifically calls for schema, states, fixtures, and a validation plan, so a machine-checkable Rust surface is the smallest truthful implementation."
review_hooks:
  - "Check bounded-scope truth, validator and fixture coherence, and whether docs avoid overclaiming later C-SDLC capabilities."
notes: "SPP revised into active execution state after conductor routing, worktree binding, implementation, and focused proof runs."
---

# Structured Plan Prompt

## Plan Summary

Active execution plan for WP-02: add the first machine-checkable cognitive
transition manifest schema, seed roles, lifecycle states, tracked fixtures,
and proof-backed docs without widening into later substrate work.

## Codex Plan

1. [completed] Confirm the v0.91.3 baseline, WP-01 dependency truth, and current schema/doc surfaces.
2. [completed] Implement the schema module, validator, fixture helper, and tracked JSON fixtures.
3. [completed] Update cognitive-sdlc and milestone proof/demo docs to point at the new surfaces.
4. [completed] Run focused schema/fixture/formatting proof gates.
5. [in_progress] Record bounded review and truthful output state before PR publication.

## Assumptions

- WP-01 is already merged, so WP-02 can treat the v0.91.3 issue wave and milestone docs as the current canonical baseline.
- WP-02 only needs the first manifest schema slice; later issues can extend transition DAG, shard-plan, and signed-trace surfaces.

## Proposed Steps

1. Confirm the v0.91.3 baseline, WP-01 dependency truth, and current schema/doc surfaces before editing.
2. Implement the bounded schema module, validator, fixture helper, and tracked valid/invalid JSON fixtures.
3. Update the cognitive-sdlc and milestone proof/demo docs so they point at the new validator-backed manifest surfaces.
4. Run focused proof gates for schema tests, fixture validity, formatting cleanliness, and diff hygiene.
5. Record bounded pre-PR review, update the issue-local cards, and publish only after the workflow truth matches the implemented state.

## Affected Areas

- `adl/src/cognitive_transition_schema.rs`
- `docs/cognitive-sdlc/transition-schema.md`
- `docs/milestones/v0.91.3/features/COGNITIVE_TRANSITION_MANIFEST.md`
- `docs/milestones/v0.91.3/review/transition_manifest/`

## Invariants To Preserve

- Keep the WP-02 slice bounded to the first manifest schema; do not absorb later transition substrate work.
- Keep repo-relative path discipline in fixtures and schema validation surfaces.
- Keep SRP as review-result truth and SOR as output truth.

## Risks And Edge Cases

- The first schema slice could overclaim later C-SDLC readiness if the docs are not explicit about bounded scope.
- Fixture paths or validator rules could drift away from the schema narrative if the tracked docs are not updated alongside the code.

## Test Strategy

- `cargo test --manifest-path adl/Cargo.toml cognitive_transition_schema -- --nocapture`
- `python3 -m json.tool docs/milestones/v0.91.3/review/transition_manifest/fixtures/valid_cognitive_transition_manifest_v1.json`
- `python3 -m json.tool docs/milestones/v0.91.3/review/transition_manifest/fixtures/invalid_cognitive_transition_manifest_v1_missing_seed_role.json`
- `cargo fmt --manifest-path adl/Cargo.toml --all --check`
- `git diff --check`

## Execution Handoff

Use this artifact as the live issue-local plan while WP-02 moves from bound
implementation to review-ready publication.

## Stop Conditions

- Stop and re-plan if WP-02 needs transition DAG, shard-plan, signed-trace, or ObsMem implementation to satisfy acceptance.
- Stop and update SPP if new proof gates are required beyond the focused schema/fixture/doc surface.
- Stop and route follow-on work if acceptance requires changes outside the bounded docs/tools queue.

## Notes

SPP revised into active execution state after conductor routing, worktree
binding, implementation, and focused proof runs.
