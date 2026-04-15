# ADL Security Posture Model

## Metadata
- Milestone: `v0.89`
- Status: `Planned`
- Source planning input: local `v0.89` planning corpus
- Milestone home: `WP-09`

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

## Runtime Contract

`WP-09` now projects declared posture into a reviewer-facing control-path artifact instead of
leaving it as planning rhetoric.

The canonical proof surfaces are:
- `control_path/security_review.json`
- `control_path/freedom_gate.json`
- `control_path/mediation.json`
- `control_path/summary.txt`

The bounded runtime semantics are:
- posture is tied to actual commitment outcomes such as allow, defer, refuse, or escalate
- mitigation authority and runtime consequence are explicit enough for reviewers to see what
  posture actually does
- accepted risk level is recorded from the bounded run rather than invented after review

The current `control_path/security_review.json` contract makes these fields explicit:
- `posture.declared_posture`
- `posture.accepted_risk_level`
- `posture.commitment_policy`
- `posture.mitigation_authority`
- `posture.runtime_consequence`
- `posture.posture_rationale`

This keeps posture visible as an execution contract while leaving heavier later-band security modes
to `v0.89.1`.

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
