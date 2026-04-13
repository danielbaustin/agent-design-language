# PHI Metrics For ADL

## Purpose

Define a bounded engineering interpretation of `Φ_ADL` for ADL.

This doc does not adopt the metaphysical claims of IIT. It extracts one useful engineering idea:

systems differ in the degree to which their behavior depends on integrated internal structure rather than isolated independent parts.

## Why It Matters

For ADL, increasing integration changes:
- routing requirements
- cost and latency
- adaptation behavior
- policy continuity
- identity persistence

`Φ_ADL` gives the project a structured way to compare:
- simple execution systems
- cognitive systems
- adaptive cognitive systems

## Non-Goals

This doc does not attempt to:
- prove or disprove consciousness claims
- compute formal IIT `Φ`
- endorse panpsychism
- act as a sentience detector

## Core Definition

`Φ_ADL` is the degree to which an ADL system's behavior depends on integrated internal structure rather than on isolated components operating independently.

Informally:
- higher `Φ_ADL` means more of the system's behavior depends on tightly coupled memory, policy, reasoning, instinct, and feedback structure
- lower `Φ_ADL` means the system can be decomposed into relatively independent steps with limited loss of function

## Dimensions

Early `Φ_ADL` should be treated as a structured profile rather than one scalar.

Candidate dimensions:
- structural coupling
- memory coupling
- feedback depth
- policy continuity
- affective / instinctive coupling
- graph irreducibility

## Practical Use in v0.88

In `v0.88`, this feature should help us:
- compare low-integration and high-integration execution paths
- reason about when memory, replay, policy, or instinct materially matter
- give reviewers a bounded language for “how integrated is this runtime path?”

## Relationship to Cost and Routing

`Φ_ADL` does not replace cost, latency, or fast/slow routing.

It complements them:
- cost tells us what execution consumed
- temporal schema tells us when and in what order it happened
- `Φ_ADL` helps explain how much internal integration the path required

This makes cognitive routing partly a coupling-allocation problem, not just a speed/cost problem.

## Relationship to v0.88

This is a bounded metric-family feature for `v0.88`.

The milestone should use it to learn something real about integration and coupling in ADL systems without overclaiming a final formal theory.

## Runtime Surface

`WP-09` owns one bounded engineering review surface:

- `adl::chronosense::PhiIntegrationMetricsContract`
- `adl identity phi --out .adl/state/phi_integration_metrics_v1.json`

The contract must stay reviewer-facing and comparison-oriented:

- explicit dimensions rather than one magic score
- low / medium / high integration profiles
- fixture-backed comparison output that names what changed
- non-goals that explicitly rule out metaphysical or sentience claims

## Required Dimensions

The current bounded `v0.88` dimensions are:

- structural coupling
- memory coupling
- feedback depth
- policy continuity
- instinct coupling
- graph irreducibility
- adaptive depth

These dimensions are not final theory.
They are a reviewer-readable engineering profile for comparing execution paths.

## Comparison Model

`v0.88` should support one bounded comparison across three integration bands:

- low integration path
- medium integration path
- high integration path

The comparison is successful when a reviewer can see:

- which dimensions moved across the three profiles
- which runtime surfaces explain the movement
- why that matters for ADL runtime behavior

## Proof Hook

Current proof hook:

```bash
adl identity phi --out .adl/state/phi_integration_metrics_v1.json
```

Expected proof artifact:

- `.adl/state/phi_integration_metrics_v1.json`

This artifact should remain suitable for later `WP-13` demo-matrix integration.

## Explicit Non-Goals

This surface must not:

- compute formal IIT `Φ`
- make consciousness or sentience claims
- collapse integration into one scalar that replaces cost, time, or routing review
- pull later-band governance or social cognition claims into `v0.88`
