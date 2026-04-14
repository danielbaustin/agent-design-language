# Security And Threat Modeling

## Metadata
- Milestone: `v0.89`
- Status: `Planned`
- Source planning input: local `v0.89` planning corpus
- Planned WP: `WP-09`

## Purpose

Turn ADL security from intuition into an explicit trust-boundary and threat-model package.

## Scope

`v0.89` should define:
- active trust boundaries
- attacker and abuse models
- canonical threat classes
- required mitigations
- reviewer and validation expectations

## Main Runtime Commitments

- security claims become tied to actual runtime surfaces
- trust boundaries are described in a way that can drive implementation and demos
- the main band stays focused on governance and threat-model clarity, while heavier adversarial runtime proof work moves to `v0.89.1`

## Non-Goals

- the full adversarial runtime implementation
- exploit generation and replay package completion

## Dependencies

- Action Mediation Layer
- Decision surfaces
- trace, provider, memory, and review surfaces

## Exit Criteria

- the milestone package can answer what can go wrong, where, and how ADL intends to contain it
- later security proof work has an explicit parent threat model rather than drifting on intuition
