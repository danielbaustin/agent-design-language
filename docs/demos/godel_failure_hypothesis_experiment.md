# v0.8 Demo: Failure -> Hypothesis -> Experiment

## Purpose

This demo illustrates the deterministic Gödel scientific loop as documentation artifacts:

1. observed failure
2. hypothesis from evidence
3. bounded experiment proposal

This is a docs/demo surface only. It does not execute runtime mutation.

## Integration Surfaces

The demo aligns with:

- `docs/milestones/v0.8/EXPERIMENT_RECORD_V1.md` (#609)
- `docs/milestones/v0.8/CANONICAL_EVIDENCE_VIEW_V1.md` (#610)
- `docs/milestones/v0.8/MUTATION_FORMAT_V1.md` (#611)
- `docs/milestones/v0.8/EVALUATION_PLAN_V1.md` (#612)
- `docs/milestones/v0.8/GODEL_EXPERIMENT_WORKFLOW_TEMPLATE_V1.md` (#613)
- `docs/milestones/v0.8/OBSMEM_INDEXING_SURFACES_V1.md` (#614)

## Deterministic Demo Artifacts

- Failure signal:
  - `adl-spec/examples/v0.8/demos/godel_failure_signal.v1.example.json`
- Hypothesis:
  - `adl-spec/examples/v0.8/demos/godel_hypothesis.v1.example.json`
- Experiment proposal:
  - `adl-spec/examples/v0.8/demos/godel_experiment_proposal.v1.example.json`

## Loop Walkthrough

### Stage 1: Failure

The failure artifact records a bounded, deterministic failure summary:

- stable identifiers (`run_id`, `workflow_id`, `failure_id`)
- structured failure class
- canonical evidence references

### Stage 2: Hypothesis

The hypothesis artifact explains the failure using evidence IDs and deterministic claims:

- one failure class target
- one explicit causal claim
- bounded confidence value
- deterministic links to evidence and prior run references

### Stage 3: Experiment

The experiment proposal artifact defines a bounded test:

- one mutation descriptor (no unconstrained patch language)
- one evaluation plan reference
- expected success criteria and stop criteria
- deterministic ordering fields and stable IDs

## Replay and Audit Notes

- All artifact references are repo-relative.
- No raw logs, prompts, tool arguments, or secrets are stored.
- Artifact shape is deterministic for identical input conditions.
- Runtime execution is intentionally out of scope for this demo.

## Non-goals

- Autonomous hypothesis generation
- Runtime mutation execution
- Policy-learning or scheduler changes
