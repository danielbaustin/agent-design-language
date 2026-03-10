# Gödel Scientific Loop Diagram (v0.8)

This document provides the canonical visual summary of the v0.8 Gödel scientific loop.

The loop is documentation-only at this stage. It describes the bounded, artifact-driven scientific process established across issues #609–#616 and does **not** imply a fully autonomous Gödel agent.

## Canonical Architecture Diagram

```text
┌─────────┐
│ Failure │
└────┬────┘
     │
     ▼
┌────────────┐
│ Hypothesis │
└────┬───────┘
     │
     ▼
┌──────────┐
│ Mutation │
└────┬─────┘
     │
     ▼
┌────────────┐
│ Experiment │
└────┬───────┘
     │
     ▼
┌────────────┐
│ Evaluation │
└────┬───────┘
     │
     ▼
┌────────┐
│ Record │
└────┬───┘
     │
     ▼
┌───────────────────┐
│ Indexing / ObsMem │
└─────────┬─────────┘
          │
          ▼
Future hypothesis search / experiment selection
```

## Stage-to-artifact map

- **Failure** → run summary, failure signal, or prior execution evidence
- **Hypothesis** → bounded hypothesis statement grounded in evidence
- **Mutation** → `Mutation v1` (#611)
- **Experiment** → bounded experiment proposal or workflow step from `Godel Experiment Workflow Template v1` (#613)
- **Evaluation** → `EvaluationPlan v1` (#612) applied to `Canonical Evidence View v1` (#610)
- **Record** → `ExperimentRecord v1` (#609)
- **Indexing / ObsMem** → `run_summary` and `experiment_index_entry` surfaces (#614)

## Canonical notes

- Canonical stage order for v0.8 is:
  `failure -> hypothesis -> mutation -> experiment -> evaluation -> record -> indexing`
- The loop is bounded and artifact-driven.
- The loop does not rely on hidden state outside declared artifacts.
- The indexing step enables later retrieval of similar failures, prior hypotheses, and experiment outcomes.
- The diagram should be read together with:
  - `docs/milestones/v0.8/GODEL_LOOP_INTEGRATION_V0.8.md`
  - `docs/milestones/v0.8/GODEL_AGENT_NOTES.md`
  - `docs/milestones/v0.8/GODEL_SCIENTIFIC_METHOD.md`

## Conceptual framing

This loop is the visual expression of the Gödel–Hadamard–Bayes cognition model emerging in ADL:

- **Gödel** → self-referential system improvement through explicit mutation and experiment records
- **Hadamard** → hypothesis generation and conceptual search
- **Bayes** → evidence-based evaluation of experiment outcomes

In ADL v0.8, these ideas are represented through deterministic artifacts and bounded workflows rather than unconstrained autonomous self-modification.
