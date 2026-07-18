# Security Implementation Readiness

## Metadata

- Feature Name: Security Implementation Readiness
- Milestone Target: `v0.91.7`
- Status: WP-12 children #4656-#4660 and review remediation #5404/#5406 closed; activation remains proof-bound
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

## WP-12 Gate Record

Issue `#4656` records the controlling WP-12 security and CAV gate in:

- `docs/milestones/v0.91.7/review/security/WP12_SECURITY_CAV_PRE_V092_REQUIREMENTS_4656.md`
- `docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json`

The gate keeps security/CAV readiness fail-closed while child issues remain
open:

- `#4657`: SSM and local polis operations readiness, proven by
  `docs/milestones/v0.91.7/review/security/WP12_SSM_READINESS_4657.md`.
- `#4658`: ACIP/A2A schema and protobuf projection, proven by
  `docs/milestones/v0.91.7/review/security/WP12_ACIP_SCHEMA_PROTOBUF_PROJECTION_4658.md`.
- `#4659`: ACIP WebSocket transport path.
- `#4660`: external-agent access rules.
- `#4914`: CAV runtime red-blue proof.
- `#4917`: tamper-evident evidence custody.
- `#4920`: key rotation and break-glass policy.

Rows without integrated proof or explicit operator-scoped-out approval remain
blockers for `v0.92` activation claims.

## v0.92 Consumption

`v0.92` may consume only reviewed implementation or blocker status. Security cannot be silently
moved out of activation.

## Non-Goals

- No compliance certification.
- No unreviewed public security claim.
- No closure by narrative.
