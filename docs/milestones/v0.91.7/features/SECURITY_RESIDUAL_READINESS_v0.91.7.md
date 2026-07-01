# Security Residual Readiness

## Metadata

- Feature Name: Security Residual Readiness
- Milestone Target: `v0.91.7`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: policy, architecture
- Proof Modes: review, tests, threat-model

## Purpose

Account for security/CAV residuals left after `v0.91.6` and define what blocks
or records an evidence-backed blocker/non-claim with operator approval before
`v0.92`.

## Scope

In scope:

- residual threat-model gaps;
- Curiosity and Constructability security implications;
- ACIP/A2A/protobuf residual security;
- public evidence and profile privacy residuals;
- activation blockers and routes to `v0.93`.

Out of scope:

- full enterprise security implementation;
- external compliance claims;
- broad runtime hardening implementation.

## Required Decisions

- Which residuals block `v0.92`?
- Which residuals route to `v0.93` enterprise security?
- Which Curiosity/Constructability actions need security gates?
- Which protocol choices require signing, access control, or privacy review?

## Dependencies

- `v0.91.6` security bridge and CAV doc.
- Constructability Gate feature doc.
- ACIP/A2A protobuf residual doc.

## Validation And Review

- Run focused threat-model review.
- Record residuals as resolved, explicitly non-claimed with operator approval,
  or blocked with evidence and operator approval.
- Prevent silent deferral of activation-path security.

## v0.92 Consumption

`v0.92` may consume only reviewed residual status. Security cannot be silently
deferred out of activation.

## Non-Goals

- No compliance certification.
- No unreviewed public security claim.
- No closure by narrative.
