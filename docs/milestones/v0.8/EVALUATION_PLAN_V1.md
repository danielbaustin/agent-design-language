# EvaluationPlan v1

## Purpose
`EvaluationPlan v1` is the canonical deterministic evaluation contract for v0.8 Godel experiments.

It defines:
- what evidence is evaluated,
- what metrics are computed,
- what decision rules are applied,
- how outcomes are derived and represented,
- and how experiment expansion is bounded.

This is a schema/specification artifact only. Runtime evaluation execution is out of scope for #612.

## Canonical Artifact Identity
- `schema_name`: `evaluation_plan`
- `schema_version`: `1`

Canonical machine-readable artifacts:
- `adl-spec/schemas/v0.8/evaluation_plan.v1.json`
- `adl-spec/examples/v0.8/evaluation_plan.v1.example.json`

## Determinism Contract
For identical normalized inputs, an EvaluationPlan must produce deterministic evaluation behavior in:
- metric evaluation order,
- rule evaluation order,
- tie-break behavior,
- outcome classification.

Required ordering semantics:
- `evidence_inputs`: sorted by `evidence_id` (lexicographic).
- `metrics`: sorted by `metric_id` (lexicographic).
- `decision_rules`: sorted by `precedence` ascending, tie-break by `rule_id` lexicographic.
- `outcome_model.decision_order`: fixed explicit order.

No hidden state or undeclared side effects are permitted.

## Security / Privacy Contract
Evaluation plans must not contain:
- secrets,
- tokens,
- raw prompts,
- tool arguments,
- absolute host paths.

Evidence references must be schema-linked and repository-safe.

## Required Top-Level Fields
- `schema_name`
- `schema_version`
- `plan_id`
- `experiment_id`
- `baseline_run_id`
- `candidate_run_id`
- `mutation_ref`
- `evidence_inputs`
- `metrics`
- `decision_rules`
- `outcome_model`

## Optional Top-Level Fields
- `experiment_policy`
- `notes`
- `metadata`

## Evidence Inputs
`evidence_inputs[]` defines evidence dependencies for deterministic evaluation.

Each entry includes:
- `evidence_id`
- `source_artifact`
- `evidence_view_ref` (must reference `canonical_evidence_view` v1)
- optional `selectors[]` (normalized, deterministic list)

## Metrics Model
`metrics[]` defines deterministic metric computations.

Each metric includes:
- `metric_id`
- `metric_type`
- `direction`
- `aggregation`
- `weight`
- optional `threshold`

`metric_type` is bounded to known deterministic metric families in v1.

## Decision Rules
`decision_rules[]` defines deterministic rule evaluation.

Each rule includes:
- `rule_id`
- `kind`
- `precedence`
- optional `metric_id`
- `operator`
- optional `target_value`
- optional `on_fail`

Rules are declarative and bounded.
No ad hoc procedural scripts are canonical in v1.

## Outcome Model
`outcome_model` defines final classification behavior.

Required fields:
- `decision_order` (bounded enum sequence)
- `default_decision`
- `tie_break` (explicit deterministic strategy)

Allowed decision values:
- `adopt`
- `reject`
- `requires_human_review`

## Optional Experiment Policy (bounded expansion control)
`experiment_policy` is optional but recommended.

It bounds hypothesis/experiment growth with declarative limits:
- `max_hypotheses_per_failure`
- `max_parallel_experiments`
- `max_experiments_per_hypothesis`
- optional `admission_thresholds`:
  - `min_evidence_count`
  - `min_confidence_score`
  - `require_policy_approval`

This block prevents uncontrolled hypothesis/experiment expansion and preserves disciplined scientific-loop behavior.

## Relationship to Adjacent Artifacts

### ExperimentRecord (#609)
Evaluation outputs are recorded via `ExperimentRecord` decision and outcome surfaces.
`EvaluationPlan` specifies how those outcomes are computed; `ExperimentRecord` stores what was decided.

### Canonical Evidence View (#610)
`evidence_inputs[].evidence_view_ref` requires `canonical_evidence_view` v1 linkage, ensuring deterministic evidence normalization for evaluation.

### Mutation format (#611)
`mutation_ref` links the evaluated candidate mutation (`schema_name = mutation`, `schema_version = 1`).
`EvaluationPlan` evaluates mutation outcomes; it does not encode mutation execution semantics.

## Non-goals
- Runtime evaluation execution engine.
- Autonomous open-ended experiment orchestration.
- Procedural scripting language for rule execution.
- Expansion beyond bounded declarative evaluation policy in v1.
