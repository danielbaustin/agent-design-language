# Reasoning Graph And Affect Demo

This bounded demo shows the first affect-linked reasoning graph slice for v0.85.

It proves:
- a structured reasoning graph artifact is emitted for the bounded AEE recovery flow
- the graph includes explicit affect-linked structure
- affect changes the ranked action path in the graph
- the graph-derived output matches the downstream bounded AEE decision

## One-command demo

From repository root:

```bash
swarm/tools/demo_reasoning_graph_affect.sh
```

## Primary proof artifacts

- `.adl/runs/v0-3-aee-recovery-initial/learning/reasoning_graph.v1.json`
- `.adl/runs/v0-3-aee-recovery-adapted/learning/reasoning_graph.v1.json`
- `.adl/runs/v0-3-aee-recovery-initial/learning/affect_state.v1.json`
- `.adl/runs/v0-3-aee-recovery-adapted/learning/affect_state.v1.json`

## What to inspect

- The initial reasoning graph should show `dominant_affect_mode: "recovery_focus"`.
- The initial graph should select `action.retry_budget` as the ranked path.
- The adapted reasoning graph should show `dominant_affect_mode: "steady_state"`.
- The adapted graph should select `action.maintain_policy` as the ranked path.
- The selected path should match the downstream bounded AEE decision recorded in `aee_decision.json`.
