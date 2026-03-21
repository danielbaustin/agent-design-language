# Affect-plus-Godel Vertical Slice Demo

This bounded demo shows one concrete causal chain:

- input condition: a failure-driven recovery run starts in `recovery_focus`
- affect change: the adapted rerun settles back to `steady_state`
- downstream change: the top Gödel-adjacent strategy flips from
  `retry_budget_probe` to `maintain_policy_review`
- proof artifact: `godel_affect_vertical_slice.v1.json`

## Run

```bash
bash swarm/tools/demo_affect_godel_vertical_slice.sh
```

## Proof Surface

The demo emits:

- `.adl/runs/v0-3-aee-recovery-initial/learning/affect_state.v1.json`
- `.adl/runs/v0-3-aee-recovery-initial/learning/reasoning_graph.v1.json`
- `.adl/runs/v0-3-aee-recovery-adapted/learning/affect_state.v1.json`
- `.adl/runs/v0-3-aee-recovery-adapted/learning/reasoning_graph.v1.json`
- `.adl/reports/demo-affect-godel-vertical-slice/runs/review-godel-affect-001/godel/godel_hypothesis.v1.json`
- `.adl/reports/demo-affect-godel-vertical-slice/runs/review-godel-affect-001/godel/godel_policy.v1.json`
- `.adl/reports/demo-affect-godel-vertical-slice/runs/review-godel-affect-001/godel/godel_experiment_priority.v1.json`
- `.adl/reports/demo-affect-godel-vertical-slice/runs/review-godel-affect-001/godel/godel_affect_vertical_slice.v1.json`

## What To Look For

In `godel_affect_vertical_slice.v1.json`:

- `input_condition.trigger`
- `affect_transition.initial_affect_mode`
- `affect_transition.adapted_affect_mode`
- `downstream_change.initial_selected_candidate_id`
- `downstream_change.adapted_selected_candidate_id`
- `downstream_change.changed`

The artifact is successful when:

- initial selected candidate is `exp:retry-budget`
- adapted selected candidate is `exp:maintain-policy`
- `downstream_change.changed` is `true`

That is the bounded WP-17 proof that affect changes a Gödel-adjacent downstream ranking surface rather than existing as passive state.
