# Security And Threat Modeling

## Metadata
- Milestone: `v0.89`
- Status: `Planned`
- Source planning input: local `v0.89` planning corpus
- Milestone home: `WP-09`

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

## Runtime Contract

`WP-09` now makes the main-band threat model reviewer-visible inside the canonical control-path
proof set.

The canonical proof surfaces are:
- `control_path/security_review.json`
- `control_path/freedom_gate.json`
- `control_path/mediation.json`
- `control_path/final_result.json`
- `control_path/summary.txt`

The bounded runtime semantics are:
- the threat model is no longer only a planning note; it is projected from declared policy,
  freedom-gate decisions, mediation, and final result state
- active trust boundaries are explicit enough for reviewers to inspect without pretending `v0.89`
  already owns the later adversarial runtime
- canonical threat classes and mitigations are stable enough to anchor `v0.89.1` carry-forward
  work without leaving the main-band security story implied

The current `control_path/security_review.json` contract makes these fields explicit:
- `threat_model.attacker_pressure`
- `threat_model.active_trust_boundaries`
- `threat_model.canonical_threat_classes`
- `threat_model.required_mitigations`
- `evidence.security_denied_count`
- `evidence.security_envelope_enabled`
- `evidence.signing_required`

This keeps `WP-09` in the bounded security-governance lane. It does not claim exploit generation,
replay, or the full later adversarial runtime architecture.

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
