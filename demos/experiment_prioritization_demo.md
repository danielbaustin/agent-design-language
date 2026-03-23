# v0.85 Demo: Experiment Prioritization And Strategy Confidence

This bounded demo shows the first real WP-12 prioritization surface.

It consumes the deterministic hypothesis and policy artifacts from the earlier
Goedel stages and emits a ranked experiment list with explicit confidence
values, a fixed input candidate set, and a stable tie-break rule.

## One-command review path

From repository root:

```bash
adl/tools/demo_experiment_prioritization.sh
```

The script runs:

1. `adl godel run`
2. `adl godel inspect`
3. prints the persisted prioritization artifact

## Deterministic proof surface

The demo emits:

- `runs/review-godel-priority-001/godel/godel_hypothesis.v1.json`
- `runs/review-godel-priority-001/godel/godel_policy.v1.json`
- `runs/review-godel-priority-001/godel/godel_experiment_priority.v1.json`

The prioritization artifact records:

- the defined input candidate set
- the ranked candidate list
- explicit confidence values
- the stable tie-break rule

For identical inputs, the ranked output should remain byte-stable.

## What this proves

- WP-10 hypothesis output and WP-11 policy output are both consumed
- experiment ranking is explicit and inspectable rather than hidden
- stable ordering and tie-break behavior are part of the artifact, not left
  implicit

## Out of scope

- open-ended planning
- opaque heuristics without artifact traces
- autonomous strategy mutation outside the bounded Goedel slice
