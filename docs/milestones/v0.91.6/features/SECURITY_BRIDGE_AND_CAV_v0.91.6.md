# Security Bridge And Continuous Adversarial Verification

## Metadata

- Feature Name: Security Bridge And CAV
- Milestone Target: `v0.91.6`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: policy, architecture
- Proof Modes: review, tests, threat-model

## Purpose

Keep security on the activation path by defining first-tranche threat-model,
CAV, malformed-output, provider-trust, prompt-record, and ACIP security work.

## Scope

In scope:

- threat-model refresh;
- Continuous Adversarial Verification route;
- malformed output and prompt-injection handling;
- provider/model trust boundaries;
- public prompt record security;
- ACIP/A2A access and message security.

Out of scope:

- external compliance certification;
- full enterprise security implementation;
- residual `v0.91.7` security closure.

## Required Decisions

- Which security checks block public prompt export?
- Which provider/model failures are security-relevant?
- Which ACIP/A2A messages require access checks or signing?
- Which residual security work blocks `v0.92`?

## Dependencies

- Public prompt records feature doc.
- Provider/model reliability feature doc.
- ACIP/A2A/provider communications feature doc.
- `v0.93` enterprise security planning.

## Validation And Review

- Run focused threat-model review.
- Route CAV checks into deterministic issue proof where possible.
- Record malformed-output and provider-trust test gaps.
- Flag residuals for `v0.91.7` or `v0.93`.

## v0.92 Consumption

`v0.92` may consume only reviewed security boundaries. Security cannot be
silently deferred out of activation.

## Non-Goals

- No broad compliance claims.
- No assumption that public records or provider messages are safe by default.
- No security-by-narrative closure.
