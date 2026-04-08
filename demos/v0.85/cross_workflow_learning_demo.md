# Cross-Workflow Learning Demo

This demo shows the first bounded cross-workflow learning slice for v0.85.

It proves a deterministic handoff:

- workflow A emits a ranked experiment artifact
- workflow B consumes the top-ranked candidate
- workflow B emits a changed downstream decision artifact

## Run

```bash
bash adl/tools/demo_cross_workflow_learning.sh
```

## What To Review

The demo emits these linked artifacts under `.adl/reports/demo-cross-workflow-learning/`:

- `runs/review-godel-crossflow-001/godel/godel_hypothesis.v1.json`
- `runs/review-godel-crossflow-001/godel/godel_policy.v1.json`
- `runs/review-godel-crossflow-001/godel/godel_experiment_priority.v1.json`
- `runs/review-godel-crossflow-001/godel/godel_cross_workflow_learning.v1.json`

The final artifact is the key proof surface. It records:

- the source hypothesis, policy, and prioritization artifact paths
- the selected top-ranked candidate from workflow A
- the downstream workflow id and decision id for workflow B
- the expected behavior change caused by that ranked input

## Expected Outcome

The inspect summary and the final artifact should show a deterministic downstream
decision derived from the same ranked experiment candidate on repeated runs.
