# Gödel Scientific Loop Integration (v0.8)

## Purpose

This document aligns v0.8 Gödel surfaces into one deterministic, bounded architecture view.

It consolidates outputs from:

- #609 ExperimentRecord
- #610 Canonical Evidence View
- #611 Mutation format
- #612 EvaluationPlan
- #613 workflow template
- #614 ObsMem indexing surfaces
- #615 deterministic demo flow

## Canonical Loop Order

The canonical stage order is:

1. failure
2. hypothesis
3. mutation
4. experiment
5. evaluation
6. record
7. indexing

This order is the shared terminology baseline for v0.8 docs.

## Stage-to-Artifact Map

| Stage | Canonical artifact/surface | Source |
|---|---|---|
| failure | Canonical failure signal/evidence refs | `CANONICAL_EVIDENCE_VIEW_V1.md` (#610) |
| hypothesis | Hypothesis statement tied to failure/evidence | `GODEL_SCIENTIFIC_METHOD.md` + demo flow (#615) |
| mutation | Bounded mutation descriptor | `MUTATION_FORMAT_V1.md` (#611), `adl-spec/schemas/v0.8/mutation.v1.json` |
| experiment | Structured workflow template | `GODEL_EXPERIMENT_WORKFLOW_TEMPLATE_V1.md` + `adl-spec/examples/v0.8/godel_experiment_workflow.template.v1.json` (#613) |
| evaluation | Deterministic checks and verdict criteria | `EVALUATION_PLAN_V1.md` + `adl-spec/schemas/v0.8/evaluation_plan.v1.json` (#612) |
| record | Full experiment outcome record | `EXPERIMENT_RECORD_V1.md` + `adl-spec/schemas/v0.8/experiment_record.v1.schema.json` (#609) |
| indexing | Queryable run/experiment index surfaces | #614 indexing surfaces (`run_summary`, `experiment_index_entry`) |

## Integration Contract

- Each stage consumes explicit artifacts from earlier stages.
- No hidden state may be required to understand stage decisions.
- Cross-stage references use stable IDs and repo-relative artifact references.
- Evaluation and record stages must preserve deterministic evidence linkage.

## Determinism and Replay

- Stage ordering is fixed.
- Comparisons are based on canonical evidence views, not raw volatile logs.
- Artifacts remain replay-compatible and audit-friendly.
- v0.8 docs do not assume autonomous runtime mutation execution.

## v0.8 Scope Boundary

Included in v0.8 docs/design:

- deterministic schema/spec surfaces
- workflow-template contracts
- bounded demo narrative of failure -> hypothesis -> experiment

Not claimed as implemented in this docs pass:

- autonomous Gödel agent runtime
- automatic policy learning loop
- unconstrained self-modifying execution
