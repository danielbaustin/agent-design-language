# ADL Trust Model Under Adversary

## Metadata
- Milestone: `v0.89`
- Status: `Planned`
- Source planning input: local `v0.89` planning corpus
- Planned WP: `WP-09`

## Purpose

Define how trust assumptions change when the system is operating under adversarial pressure.

## Scope

`v0.89` should make explicit:
- what surfaces remain trustworthy
- where trust must be reduced or re-earned
- how provider, transport, memory, and reviewer surfaces are treated under contest
- how posture and threat-model claims influence trust boundaries

## Main Runtime Commitments

- trust under adversary is explicit rather than implied by normal-path assumptions
- the system can distinguish ordinary runtime confidence from contested confidence
- later `v0.89.1` adversarial work inherits a coherent trust story

## Runtime Contract

`WP-09` now makes contested trust legible through one bounded reviewer-facing artifact rather than
scattering it across planning notes.

The canonical proof surfaces are:
- `control_path/security_review.json`
- `control_path/action_proposals.json`
- `control_path/memory.json`
- `control_path/freedom_gate.json`
- `control_path/summary.txt`

The bounded runtime semantics are:
- trusted surfaces are separated from reduced-trust surfaces under contest
- non-authoritative proposals remain reduced-trust until runtime mediation authorizes them
- revalidation requirements and escalation paths are explicit enough to drive later adversarial
  proof work without widening `v0.89` into that later band

The current `control_path/security_review.json` contract makes these fields explicit:
- `trust_under_adversary.trust_state`
- `trust_under_adversary.trusted_surfaces`
- `trust_under_adversary.reduced_trust_surfaces`
- `trust_under_adversary.revalidation_requirements`
- `trust_under_adversary.escalation_path`
- `evidence.trace_visibility_expectation`

This gives `v0.89` a stable trust language for contested operation while keeping the full red/blue
runtime architecture out of scope.

## Non-Goals

- the full red/blue runtime architecture
- final exploit artifact and replay protocol details

## Dependencies

- Security and Threat Modeling
- ADL Security Posture Model
- provider / memory / trace substrate

## Exit Criteria

- the milestone package has a stable trust language for contested operation
- review and planning docs stop treating adversarial trust as generic caution
