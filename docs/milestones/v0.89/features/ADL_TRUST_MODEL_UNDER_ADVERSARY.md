# ADL Trust Model Under Adversary

## Metadata
- Milestone: `v0.89`
- Status: `Planned`
- Source planning doc: `.adl/docs/v0.89planning/ADL_TRUST_MODEL_UNDER_ADVERSARY.md`
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
