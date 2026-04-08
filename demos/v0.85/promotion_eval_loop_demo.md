# Promotion and Eval Loop Demo

This demo shows the first bounded WP-14 promotion and evaluation loop for v0.85.

It proves a deterministic closure:

- prior Gödel artifacts are consumed
- a structured evaluation report is emitted
- a machine-readable promotion decision is emitted
- the decision is deterministically derived from the evaluation report

## Run

```bash
bash adl/tools/demo_promotion_eval_loop.sh
```

## What To Review

The demo emits these linked artifacts under `.adl/reports/demo-promotion-eval-loop/`:

- `runs/review-godel-promotion-001/godel/godel_hypothesis.v1.json`
- `runs/review-godel-promotion-001/godel/godel_policy.v1.json`
- `runs/review-godel-promotion-001/godel/godel_experiment_priority.v1.json`
- `runs/review-godel-promotion-001/godel/godel_cross_workflow_learning.v1.json`
- `runs/review-godel-promotion-001/godel/godel_eval_report.v1.json`
- `runs/review-godel-promotion-001/godel/godel_promotion_decision.v1.json`

The last two are the key proof surfaces. Together they record:

- the structured evaluation score and rationale
- the upstream artifact chain consumed to produce that score
- the explicit promotion decision (`promote` or `reject`)
- the deterministic mapping from evaluation score to promotion outcome

## Expected Outcome

The inspect summary and the final two artifacts should show a deterministic
evaluation score and a deterministic promotion decision derived from the same
bounded inputs on repeated runs.
