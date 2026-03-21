# Affect Engine Demo

This bounded demo shows the first working affect-engine slice for v0.85.

It proves:
- a concrete affect-state model is emitted as a structured artifact
- failure evidence deterministically updates that affect state
- the affect state influences the downstream AEE recovery recommendation
- the bounded recovery path changes observable behavior on the next run

## One-command demo

From repository root:

```bash
swarm/tools/demo_affect_engine.sh
```

## Primary proof artifacts

- `.adl/runs/v0-3-aee-recovery-initial/learning/affect_state.v1.json`
- `.adl/runs/v0-3-aee-recovery-initial/learning/aee_decision.json`
- `.adl/runs/v0-3-aee-recovery-adapted/learning/affect_state.v1.json`
- `.adl/runs/v0-3-aee-recovery-adapted/learning/aee_decision.json`

## What to inspect

- The initial affect artifact should show a recovery-focused affect mode.
- The initial AEE decision should reference that affect state and recommend a raised retry budget.
- The adapted run should succeed under the bounded overlay aligned with that affect-guided recommendation.
