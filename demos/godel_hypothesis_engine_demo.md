# Gödel Hypothesis Engine Demo

This bounded demo is the canonical reviewer-facing proof surface for the first
real Gödel hypothesis engine slice in `v0.85`.

It demonstrates:

1. deterministic bounded Gödel runtime execution
2. structured persisted hypothesis generation
3. inspectable downstream runtime artifacts that prove the hypothesis surface
   is reusable rather than free-form narrative

## One-command demo

From repository root:

```bash
swarm/tools/demo_godel_hypothesis_engine.sh
```

## Primary proof artifacts

- `.adl/reports/demo-godel-hypothesis-engine/runs/review-godel-cli-001/godel/godel_hypothesis.v1.json`
- `.adl/reports/demo-godel-hypothesis-engine/runs/review-godel-cli-001/godel/canonical_evidence_view.v1.json`
- `.adl/reports/demo-godel-hypothesis-engine/runs/review-godel-cli-001/godel/mutation.v1.json`
- `.adl/reports/demo-godel-hypothesis-engine/runs/review-godel-cli-001/godel/evaluation_plan.v1.json`

## What to inspect

- `godel_hypothesis.v1.json`
  - should be structured, deterministic, and named
- `canonical_evidence_view.v1.json`
  - should tie the hypothesis back to the bounded failure inputs
- `mutation.v1.json`
  - should show the bounded proposed change surface
- `evaluation_plan.v1.json`
  - should prove downstream usability of the emitted hypothesis

## Why this matters

This is the cleanest first reviewer entry point into the bounded Gödel loop.
It proves the milestone now emits a real structured learning artifact rather
than descriptive learning language.
