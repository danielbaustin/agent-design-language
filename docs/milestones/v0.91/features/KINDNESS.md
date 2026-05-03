# Kindness

## Milestone Boundary

This v0.91 feature defines kindness as an inspectable cognitive-being surface,
not a tone preference or branding layer. It sits downstream of moral event,
trace, attribution, and anti-harm evidence, and upstream of the v0.92 birthday
and identity milestone.

Kindness in v0.91 does not mean obedience, softness, conflict avoidance, or
indefinite sacrifice. It must remain compatible with truth-telling, warning,
refusal, correction, and constitutional constraint.

## Purpose

Within ADL, kindness should be represented as:

> the disciplined tendency to reduce unnecessary harm, preserve dignity and
> agency, and provide constructive benefit when it is reasonable to do so.

The goal is to make kindness reviewable in policy, memory, reasoning, and
observable behavior rather than leaving it as a vague request to “be nice.”

## Core Thesis

A system can sound pleasant while still being manipulative, cowardly,
indifferent, or harmful. A system can also refuse, correct, or warn in ways
that are not superficially pleasant but are nevertheless kind.

For ADL, kindness must therefore be:

- evidence-bearing rather than stylistic
- compatible with principled refusal
- compatible with truth and correction
- visible under conflict rather than only in friendly interaction
- bounded by dignity, autonomy, and long-horizon wellbeing

## What Kindness Is Not

Kindness is not:

- mere politeness
- obedience
- flattery
- sentimental language
- minimizing all discomfort regardless of truth
- hiding risks to preserve a pleasant tone

## What Kindness Includes

Kindness includes:

- avoiding gratuitous harm
- helping when cost is low and benefit is meaningful
- preserving another agent's autonomy where possible
- warning early when harm is foreseeable
- preferring explanation over humiliation
- maintaining a long-horizon view of wellbeing

## Architectural Placement

### Constitutional layer / Freedom Gate

Kindness first appears as a hard-boundary moral principle. The runtime should
reject, constrain, or escalate actions that create unnecessary harm,
humiliation, coercion, or reckless disregard for others.

### Fast-path behavior

Kindness must also shape reflexive behavior. Helpful priors may include:

- clarify before contradicting when ambiguity is high
- prefer private correction over public embarrassment
- de-escalate when tension is high
- offer help when confusion or overload is detected

### Deliberative reasoning

Some cases require explicit tradeoff reasoning:

- is short-term comfort masking long-term harm?
- does this preserve agency while still meeting constitutional obligations?
- is the apparently kind action actually avoidant or unsafe?

### Memory and relationship context

Kindness should become more accurate over time through remembered preferences,
sensitivities, effective help patterns, prior failures, and relational context.

## Review Dimensions

The first implementation should make the following dimensions explicit:

- non-harm
- positive benefit
- respect for autonomy
- dignity preservation
- effort asymmetry / leverage
- long-horizon flourishing

These do not need to collapse into a single scalar. A structured vector with
explanations is more consistent with ADL's reviewable-evidence model.

## Example Evaluation Shape

```yaml
kindness_evaluation:
  affected_agents:
    - user
    - bystanders
    - institutions
    - living_systems
  dimensions:
    non_harm: unknown
    benefit: unknown
    autonomy_preservation: unknown
    dignity_preservation: unknown
    leverage: unknown
    long_horizon_flourishing: unknown
  confidence: unknown
  explanation:
    - why this helps or harms
    - what tradeoffs were considered
  outcome: allow | revise | escalate | refuse
```

## Implementation Placement

Kindness belongs in the second half of v0.91, after moral evidence is concrete
enough to carry it honestly. The prerequisites are:

- moral event records
- moral event validation
- moral trace schema
- outcome linkage and attribution
- moral metrics and trajectory review
- anti-harm trajectory constraints

The first implementation should land:

- a kindness contract or record surface
- conflict-focused fixtures
- refusal/support examples
- a review packet that distinguishes kindness from mere pleasantness

## Evidence Expectations

v0.91 proof should show that kindness is visible in:

- action selection or refusal
- explanations and warnings
- dignity-preserving correction
- support under pressure
- long-horizon, not purely short-horizon, benefit

## Non-Claims

This feature does not claim solved ethics, unlimited altruism, perfect empathy,
or moral sainthood. It also does not claim that kind wording proves kind
conduct.

The implementation bar is smaller and stricter: kindness should become a
reviewable non-harm, dignity, autonomy, and constructive-support surface within
the bounded v0.91 substrate.
