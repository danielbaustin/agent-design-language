# Security Implementation Readiness

## Metadata

- Feature Name: Security Implementation Readiness
- Milestone Target: `v0.91.7`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: policy, architecture
- Proof Modes: review, tests, threat-model

## Purpose

Account for security/CAV work left after `v0.91.6` and define what blocks
or records an evidence-backed blocker with operator approval before
`v0.92`.

## Scope

In scope:

- threat-model gaps;
- Curiosity and Constructability security implications;
- ACIP/A2A/protobuf security;
- public evidence and profile privacy requirements;
- activation blockers and explicitly approved `v0.93` assignments.

Out of scope:

- full enterprise security implementation;
- external compliance claims;
- broad runtime hardening implementation.

## Required Decisions

- Which requirements block `v0.92`?
- Which requirements are explicitly postponed to `v0.93` enterprise security with evidence and operator approval?
- Which Curiosity/Constructability actions need security gates?
- Which protocol choices require signing, access control, or privacy review?

## Dependencies

- `v0.91.6` security and CAV doc.
- Constructability Gate feature doc.
- ACIP/A2A protobuf implementation doc.

## Validation And Review

- Run focused threat-model review.
- Record requirements as resolved or blocked with evidence and operator approval.
- Prevent silent deferral of activation-path security.

## v0.92 Consumption

`v0.92` may consume only reviewed implementation or blocker status. Security cannot be silently
moved out of activation.

## Non-Goals

- No compliance certification.
- No unreviewed public security claim.
- No closure by narrative.
