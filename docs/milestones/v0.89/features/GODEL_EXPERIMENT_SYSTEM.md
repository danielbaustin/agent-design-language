# Godel Experiment System

## Metadata
- Milestone: `v0.89`
- Status: `Planned`
- Source planning doc: `.adl/docs/v0.89planning/GODEL_EXPERIMENT_SYSTEM.md`
- Planned WP: `WP-07`

## Purpose

Deepen the bounded scientific loop into a real experiment system with explicit evidence and governed adopt/reject behavior.

## Scope

`v0.89` should define:
- experiment records
- baseline / variant pairing
- bounded mutations
- evaluation plans
- adoption and rejection decisions
- durable experiment evidence

## Main Runtime Commitments

- system improvement claims become explicit experiment artifacts
- adoption decisions are governed acts, not hidden preferences
- experiment evidence can later support ObsMem and review surfaces

## Runtime Contract

`WP-07` now exposes the bounded Godel experiment package through the public CLI instead of
leaving it implicit inside stage-loop persistence.

The bounded proof entrypoints are:

```bash
cargo run --manifest-path adl/Cargo.toml -- godel run --run-id run-745-a --workflow-id wf-godel-loop --failure-code tool_failure --failure-summary "step failed with deterministic parse error" --evidence-ref runs/run-745-a/run_status.json --evidence-ref runs/run-745-a/logs/activation_log.json --runs-dir /tmp/adl-godel-demo
cargo run --manifest-path adl/Cargo.toml -- godel inspect --run-id run-745-a --runs-dir /tmp/adl-godel-demo
```

The reviewer-facing proof surfaces now include both the runtime loop artifacts and the canonical
experiment package:
- runtime stage-loop artifacts:
  - `runs/<run-id>/godel/godel_hypothesis.v1.json`
  - `runs/<run-id>/godel/godel_policy.v1.json`
  - `runs/<run-id>/godel/godel_policy_comparison.v1.json`
  - `runs/<run-id>/godel/godel_experiment_priority.v1.json`
  - `runs/<run-id>/godel/godel_cross_workflow_learning.v1.json`
  - `runs/<run-id>/godel/godel_eval_report.v1.json`
  - `runs/<run-id>/godel/godel_promotion_decision.v1.json`
  - `runs/<run-id>/godel/experiment_record.runtime.v1.json`
  - `runs/<run-id>/godel/obsmem_index_entry.runtime.v1.json`
- canonical experiment artifacts:
  - `runs/<run-id>/godel/evaluation_plan.v1.json`
  - `runs/<run-id>/godel/mutation.v1.json`
  - `runs/<run-id>/godel/canonical_evidence_view.v1.json`
  - `runs/<run-id>/godel/experiment_record.v1.json`

`godel inspect` now surfaces the canonical package identifiers and decision semantics directly:
- `canonical_evaluation_plan_id`
- `canonical_mutation_id`
- `canonical_evidence_view_id`
- `canonical_experiment_id`
- `canonical_decision_result`
- `baseline_run_id`
- `variant_run_id`

This keeps `WP-07` bounded to governed experiment packaging and reviewability without pretending
that later multi-run optimization, broader experiment scheduling, or adversarial self-mutation are
already in scope.

## Non-Goals

- unconstrained self-modification
- open-ended recursive optimization
- later full reasoning-graph architecture

## Dependencies

- AEE Convergence Model
- Decision surfaces and schema
- ObsMem evidence and ranking

## Exit Criteria

- the milestone package can describe bounded improvement as an explicit evidence-bearing subsystem
- later demos can show adopt / reject behavior without narrative hand-waving
- the public `godel run` / `godel inspect` summaries expose both the runtime loop artifacts and
  the canonical experiment package for review
