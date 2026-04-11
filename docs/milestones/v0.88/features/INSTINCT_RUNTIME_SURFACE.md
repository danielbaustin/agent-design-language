# Instinct Runtime Surface

## Purpose

Define the concrete runtime surface for instinct in ADL.

This doc turns instinct from a conceptual planning theme into a bounded runtime contract.

## Scope

This feature owns:
- instinct declaration and representation
- bounded influence on routing or prioritization
- trace and artifact visibility for instinct-driven behavior

It does not own:
- the full instinct philosophy
- general identity semantics
- later governance layers

## Core Runtime Contract

Instinct should exist as an explicit structured surface attached to an agent or execution context.

Each instinct entry should make it possible to represent:
- an identifier
- a bounded weight or strength
- optional constraints or enablement state

At runtime:
- instincts are read during arbitration or planning
- they influence prioritization or routing in a bounded way
- they remain subordinate to higher-level policy and safety constraints

## Observability Requirements

Instinct influence must be visible in trace or derived artifacts.

At minimum, a reviewer should be able to see:
- which instinct settings were present
- where they influenced a decision
- which candidate or route was selected
- whether policy or governance overrode the instinct pressure

If instinct influence is not inspectable, the feature is too implicit to trust.

## Integration Points

Primary integrations:
- arbitration
- fast / slow or equivalent routing surfaces
- trace emission
- proof artifacts

Secondary integrations:
- memory or context retrieval where relevant
- demo / review surfaces

## Determinism and Bounds

The runtime surface must remain:
- deterministic for the same inputs
- bounded in influence
- policy-constrained
- replayable or at least replay-explainable

Instinct must not become a hidden source of non-deterministic drift.

## Planned Proof Surface

`v0.88` should include at least one bounded proof path where:
- two or more candidate actions or routes exist
- instinct settings differ materially
- the selected result changes in a reviewable way
- the result remains policy-constrained and deterministic

Good proof examples:
- completion-biased choice between finishing current work and exploring novelty
- curiosity / coherence bias toward anomaly follow-up
- integrity bias toward a slower, more constrained route

## Acceptance Shape

This feature is substantively real only if:
- instinct is represented explicitly
- instinct changes at least one decision surface
- the effect is visible in trace or artifacts
- the behavior remains deterministic and bounded

## Relationship to v0.88

This is the runtime companion to `INSTINCT_MODEL.md`.

Together, the two docs define:
- what instinct means
- how instinct shows up in actual ADL runtime behavior
