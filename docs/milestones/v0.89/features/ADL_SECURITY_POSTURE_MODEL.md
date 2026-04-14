# ADL Security Posture Model

## Metadata
- Milestone: `v0.89`
- Status: `Planned`
- Source planning doc: `.adl/docs/v0.89planning/ADL_SECURITY_POSTURE_MODEL.md`
- Planned WP: `WP-09`

## Purpose

Define declared security posture as a first-class execution contract in ADL.

## Scope

`v0.89` should specify:
- posture dimensions
- posture-linked runtime consequences
- reviewer-visible posture evidence
- relationship between posture, accepted risk, mitigation authority, and proof obligations

## Main Runtime Commitments

- security posture is declared, not inferred after the fact
- posture materially affects what contested execution is allowed to do
- posture is visible in trace and artifacts

## Non-Goals

- full adversarial runtime implementation
- final exploit replay package details

## Dependencies

- Security and Threat Modeling
- Trust model under adversary
- later `v0.89.1` adversarial execution work

## Exit Criteria

- the milestone package has a crisp definition of posture as both policy surface and execution surface
- later adversarial work can cite stable posture classes instead of vague mode names
