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

`WP-11` owns one bounded, reviewable runtime hook:
- a shared instinct-sensitive agency-selection rule
- deterministic candidate shifts inside the already-selected fast / slow path
- a proof surface that exposes when instinct changed the selected candidate and when policy held it in place

Owned runtime surfaces:
- `adl::execute::select_instinct_runtime_candidate`
- `adl::execute::AgencySelectionState`
- `adl::execute::RuntimeControlState`
- `adl identity instinct-runtime --out .adl/state/instinct_runtime_surface_v1.json`

At runtime:
- instinct is read after arbitration and fast / slow path selection
- instinct may change the selected bounded candidate within that path
- instinct remains subordinate to risk and policy constraints

## Observability Requirements

Instinct influence must be visible in trace or derived artifacts.

At minimum, a reviewer should be able to see:
- which instinct setting was dominant
- which path had already been selected
- which candidate was chosen
- why that candidate was chosen
- whether risk / policy overrode instinct pressure

If instinct influence is not inspectable, the feature is too implicit to trust.

## Integration Points

Primary integrations:
- arbitration
- fast / slow routing surfaces
- agency candidate selection
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

## Bounded Decision Rule

The shared rule is intentionally small:
- on `fast_path`, `curiosity` or `integrity` upgrades direct execution to one bounded verification pass
- on `slow_path`, `curiosity` may choose bounded deferral rather than immediate execution
- on `slow_path`, `high` risk or `integrity` / `coherence` keeps review selected
- on `slow_path`, `completion` still stays bounded inside review-first slow-path semantics

This is enough to make instinct operational without turning it into hidden initiative.

## Proof Surface

`v0.88` now includes a bounded proof hook:

```text
adl identity instinct-runtime --out .adl/state/instinct_runtime_surface_v1.json
```

That proof surface includes review cases where:
- `curiosity` changes a `fast_path` candidate from direct execution to bounded verification
- `curiosity` changes a `slow_path` candidate from review to bounded deferral
- `high` risk keeps the `slow_path` on review even when curiosity would otherwise defer

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
- how instinct changes bounded candidate selection at runtime
