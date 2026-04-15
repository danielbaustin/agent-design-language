# Vision - v0.89.1

## Metadata
- Milestone: `v0.89.1`
- Version: `v0.89.1`
- Date: `2026-04-14`
- Owner: `Daniel Austin`

## Purpose

Describe the intended shape of `v0.89.1` at the product and architecture level.

## Vision

`v0.89.1` is where ADL becomes explicit about continuous intelligent opposition.

The milestone vision is not merely “better security.” It is:

- runtime architecture that assumes contested operation
- bounded adversarial execution loops
- explicit artifact contracts for replaying and reviewing exploit scenarios
- demo and review surfaces that make red/blue behavior legible instead of theatrical

ADL should be able to say, truthfully and reviewably:

- what adversarial behavior was attempted
- how the system represented that behavior
- what artifacts prove the result
- how a reviewer can replay or inspect the claim

## Core Outcome

By the end of `v0.89.1`, the adversarial/security band should no longer be a loose carry-forward promise. It should be a milestone with:

- explicit runtime model
- explicit execution roles
- explicit exploit/replay artifact family
- explicit proof surfaces

## In-Scope Capabilities

- adversarial runtime framing
- red/blue/purple role architecture
- exploit artifact schema family
- replay manifest and execution runner surfaces
- continuous verification / exploit generation patterns
- self-attacking system patterns
- adversarial demo and security-proof demo planning

## Not The Goal

This milestone is not trying to solve every governance problem, every refusal problem, or every multi-agent constitutional question.

It is trying to make the adversarial/runtime band explicit, bounded, and executable.

## Success Definition

The milestone succeeds when:

- `v0.89` no longer carries an implied adversarial backlog
- reviewers can point to a concrete `v0.89.1` package
- the package is strong enough to seed a truthful implementation issue wave
- the issue wave can be opened quickly from the docs without another design round
