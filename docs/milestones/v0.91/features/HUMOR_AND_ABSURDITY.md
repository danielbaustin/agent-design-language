# Humor And Absurdity

## Milestone Boundary

This v0.91 feature is not an entertainment layer. It is a bounded cognitive
capability for detecting wrong frames, contradiction, and brittle problem
formulations, then producing reviewable reframing evidence.

It belongs in v0.91 because cognitive-being work needs a way to remain coherent
under mismatch and contradiction. It does not claim human-style comedy,
subjective amusement, or full Theory of Mind.

## Purpose

ADL should treat absurdity detection and reframing as a first-class capability:

> detect that the current model of the situation is wrong, incomplete, or
> inconsistent, then continue operating without collapse.

In humans this can sometimes be experienced as humor. In ADL the implementation
target is narrower and more operational: contradiction tolerance plus bounded
reframing.

## Core Thesis

Without a reframing surface, capable systems tend to:

- loop on invalid assumptions
- waste compute on low-yield repetition
- escalate effort without improving outcomes
- misclassify frame failure as insufficient persistence

With a bounded reframing surface, the system can:

- detect low frame adequacy
- move from execution to diagnosis when appropriate
- request missing inputs
- change decomposition strategy coherently
- continue under a better frame

## Key Signals

Absurdity or frame-failure indicators may include:

- expected structure diverges from observed outcomes
- repeated failure without new information
- oscillating or contradictory evaluation signals
- mutually incompatible constraints
- persistent disagreement across agents or critics

These should feed an explicit `frame_adequacy` or similar reasoning-control
surface rather than remaining hidden heuristics.

## Architectural Placement

This capability spans:

- evaluation and arbitration
- affect-like tension or frustration signals
- memory of reframing events and outcomes
- cognitive-loop transitions between execution, diagnosis, and synthesis

It should be visible in runtime evidence, not only in prompt prose.

## Core Objects

The first implementation should make these surfaces explicit:

- frame adequacy signal
- reframing trigger
- reframing event record
- prior frame
- new frame
- trigger reason
- justification linked to evidence

The landed v0.91 WP-12 surface is `humor_and_absurdity_review_packet.v1` in
`adl/src/runtime_v2/humor_and_absurdity.rs`. It makes five canonical signals
explicit:

- frame adequacy
- contradiction detection
- bounded reframing
- truth and dignity preservation
- anti-manipulation boundary

It also carries four required fixture classes:

- constructive reframing
- failed reframing
- manipulation risk
- inappropriate humor

Those fixtures stay evidence-backed and bounded. Manipulative or minimizing
reframes must fail closed with `refuse` or `escalate`, and the contract
explicitly rejects entertainment or therapy claims.

## Example Reframing Artifact

```yaml
reframing_event:
  trigger_reason: contradiction | repeated_failure | ambiguity | disagreement
  prior_frame: "original task framing"
  new_frame: "diagnostic or revised framing"
  evidence_links:
    - evaluation_signal
    - trace_ref
  justification:
    - why the old frame failed
    - why the new frame is bounded and reviewable
```

## Design Constraints

Reframing must be:

- bounded
- observable
- justified
- evidence-linked
- replayable

The system must avoid both extremes:

- no reframing, which leads to brittle looping
- unbounded reframing, which destroys task coherence

## Implementation Placement

v0.91 should land a first real reframing surface after the moral evidence layer
is concrete enough to carry it honestly. The initial scope should include:

- one contradiction or wrong-frame event type
- positive and negative fixtures
- safety constraints against arbitrary reinterpretation
- trace and replay visibility

## Evidence Expectations

The proof surface should show:

- the system recognizes a low-adequacy frame
- the reframing trigger is explicit
- the new frame is bounded and reviewable
- the system continues coherently instead of collapsing or looping

## Non-Claims

This feature does not claim stand-up comedy, human-style wit, subjective
amusement, or entertainment competence. The milestone bar is narrower:

ADL should be able to detect contradiction or wrong-frame conditions and emit a
bounded reframing record that reviewers can inspect.
