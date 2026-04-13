# Instinct Model

## Purpose

Define instinct in ADL as a bounded, persistent background pressure that shapes prioritization, routing, and follow-through without replacing goals, affect, policy, or review.

This doc is the conceptual feature surface for instinct in `v0.88`.

## Core Claim

Instinct is not the same as:
- goals
- affect
- prompt wording
- unconstrained autonomy

Instinct is the structured answer to:
- what does this system tend to care about under uncertainty?
- what persistent pressures remain active across tasks?
- what bounded directional bias should exist before full deliberation?

## Why It Matters

Without an instinct surface, the rest of the system stays under-motivated:
- contradictions do not create principled pressure
- anomalies do not necessarily trigger follow-up
- unfinished work does not create structured completion pressure
- all motivation remains externalized to task prompts

Instinct provides a bounded engineering mechanism for:
- persistence
- anomaly sensitivity
- coherence pressure
- completion pressure

## Instinct vs Goals vs Affect

Goals are:
- explicit
- bounded
- task-specific

Affect is:
- dynamic evaluation of current state
- tension, confidence, urgency, contradiction, or curiosity signal

Instinct is:
- persistent across tasks
- background directional pressure
- a low-latency influence on selection and routing

Useful shorthand:
- instinct = what the system leans toward
- affect = how current state feels relative to that pressure
- goals = what work is currently being attempted

## Candidate Core Instincts

Initial bounded instinct set:
- `integrity`
- `curiosity`
- `coherence`
- `completion`

These should remain:
- small
- interpretable
- auditable
- subordinate to policy and safety constraints

## Runtime Implications

Instinct should be able to influence:
- prioritization among candidate actions
- routing between fast / slow or shallow / deeper paths
- anomaly follow-up behavior
- persistence on started work

Instinct must not:
- bypass policy
- introduce hidden non-determinism
- become an excuse for vague or theatrical agent behavior

## Reviewability

Instinct is only useful if it is visible.

The system should make it possible to inspect:
- which instinct settings were declared
- where they influenced selection or routing
- whether higher-level constraints overrode them

## Non-Goals

This doc does not attempt to:
- deliver a full psychology model
- define long-term identity through instinct alone
- replace later governance, freedom, or social coordination systems

## Relationship to v0.88

In `v0.88`, instinct is a bounded substrate feature.

The milestone should prove that instinct can be:
- declared explicitly
- applied in a narrow runtime path
- reviewed through trace and artifacts

## Runtime Surface

`WP-10` owns one bounded substrate contract:

- `adl::chronosense::InstinctModelContract`
- `adl identity instinct --out .adl/state/instinct_model_v1.json`

This substrate is intentionally small and explicit so `WP-11` can wire it into runtime behavior without redefining what instinct means.

## Initial Instinct Set

The initial `v0.88` instinct set is:

- `integrity`
- `curiosity`
- `coherence`
- `completion`

Each instinct must remain:

- interpretable
- policy-subordinate
- stable enough for later runtime attachment
- visible to reviewers as a declared surface rather than hidden prompt flavor

## Representation Contract

The bounded instinct representation must support:

- `instinct_id`
- `default_strength`
- `meaning`

Optional but allowed:

- enablement state
- allowed influence surfaces
- explicit higher-order constraints

## Semantics Boundaries

The contract must preserve these distinctions:

- instinct is persistent directional pressure
- goals are explicit task-local aims
- affect is dynamic state evaluation

Instinct may shape prioritization or routing later, but this issue does not yet claim:

- runtime arbitration proof
- instinct-driven route changes
- bounded agency demos

Those belong to `WP-11`.

## Proof Hook

Current proof hook:

```bash
adl identity instinct --out .adl/state/instinct_model_v1.json
```

Expected proof artifact:

- `.adl/state/instinct_model_v1.json`

## Explicit Non-Goals

This surface must not become:

- a full psychology model
- implicit autonomy theater
- a substitute for policy or safety
- later-band identity, governance, or social-cognition work
