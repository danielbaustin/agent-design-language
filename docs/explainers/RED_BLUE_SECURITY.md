# Red / Blue Adversarial Security

ADL treats adversarial security as a runtime architecture, not as a periodic
external review ritual.

The core idea is simple: serious agent systems should be able to model attacks,
defend against them, replay what happened, and preserve the result as durable
security knowledge.

## The Roles

ADL uses red, blue, and purple roles as reviewable runtime surfaces.

- Red explores bounded offensive hypotheses against declared targets.
- Blue interprets exploit evidence, proposes mitigations, and validates
  defensive outcomes.
- Purple coordinates the loop: prioritization, replay, escalation, regression,
  and durable learning.

These roles are not personality labels. They are accountable responsibilities
with bounded authority and visible artifacts.

## Why This Matters

Agent systems will be probed continuously. Waiting for external attackers to
discover failures is too slow and too risky.

This is the Mythos problem: once frontier vulnerability-finding systems make
discovery cheap and fast, any real weakness should be assumed discoverable by
someone else. The defensive posture has to change from occasional review to
continuous, governed, evidence-producing self-examination.

ADL's answer is bounded self-attack:

```text
surface -> hypothesis -> exploit attempt -> defense -> replay -> learning
```

Every step must preserve target scope, policy posture, evidence, attribution,
and replay status. The system may test itself, but only inside declared
boundaries.

## Relationship To The Runtime

Red/blue security depends on the deterministic runtime and CSM foundation:
attacks, mitigations, and replays must be tied to traces, artifacts, state,
policy decisions, and Freedom Gate boundaries.

It also connects directly to AEE. Self-attack is a bounded adaptive loop:
explore, test, evaluate, mitigate, replay, and retain what was learned.

## Non-Goals

- Red/blue security is not permission to attack arbitrary systems.
- It is not uncontrolled self-harm or automatic mutation.
- It is not a replacement for human security review.
- It is not a claim of external certification.

## Deeper References

- [Red / Blue Agent Architecture](../milestones/v0.89.1/features/RED_BLUE_AGENT_ARCHITECTURE.md)
- [Self-Attacking Systems](../milestones/v0.89.1/features/SELF_ATTACKING_SYSTEMS.md)
- [Adversarial runtime demo matrix](../milestones/v0.89.1/DEMO_MATRIX_v0.89.1.md)
