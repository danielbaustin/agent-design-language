# Enterprise Security v0.93

## Status

Forward-planning feature contract. This document schedules the v0.93
enterprise-security tranche. It is not an implementation closeout record and it
does not claim external compliance certification.

## Purpose

v0.93 should make enterprise security a first-class part of ADL polis
governance. Security must not be only a perimeter concern, an operator habit, or
hidden environment configuration. It should be represented through explicit
identity, authority, policy, key, audit, isolation, incident, provenance, and
adversarial-review surfaces.

## Source Inputs

- `docs/milestones/v0.93/README.md`
- `docs/milestones/v0.93/WBS_v0.93.md`
- `docs/milestones/v0.93/CONSTITUTIONAL_CITIZENSHIP_AND_POLIS_GOVERNANCE_PLAN_v0.93.md`
- `docs/milestones/v0.93/features/README.md`
- `docs/planning/ADL_FEATURE_LIST.md`

## Scope

This feature should establish:

- the canonical `v0.93` enterprise-security contract for the ADL polis
- an explicit split of the security band into bounded zero-trust, policy,
  cryptographic-trust, audit, isolation/privacy, and adversarial-ops tranches
- the production-security handoff for v0.92 WebSocket-carried ACIP and
  normalized provider session events
- reviewer-facing proof expectations and demo obligations for the security
  milestone
- a clear truth boundary between internal architecture/security claims and
  external compliance claims

## Organization Boundary

v0.93 enterprise-security implementation should consume the v0.91.5
organization packet:

- `docs/milestones/v0.91.5/features/ENTERPRISE_SECURITY_ORGANIZATION_BOUNDARY_v0.91.5.md`

That packet recommends staged module/proof separation. The six v0.93 security
WPs should identify their target capability band before implementation:
`zero_trust`, `policy`, `trust`, `audit`, `isolation`, or `adversarial_ops`.
Do not move existing core runtime primitives merely to make the tree look
organized; move or add code only when ownership, proof fixtures, dependency
impact, and review gates are explicit.

## Six Security WPs

| WP | Name | Required work product |
| --- | --- | --- |
| WP-S1 | [Zero-trust architecture](SECURITY_WP_S1_ZERO_TRUST_ARCHITECTURE_v0.93.md) | Trust-boundary contract, actor/zone model, default-deny fixtures, and unauthorized-boundary negative cases. |
| WP-S2 | [Policy enforcement and authorization](SECURITY_WP_S2_POLICY_ENFORCEMENT_AUTHORIZATION_v0.93.md) | IAM/delegation/standing/tool-authority policy decision contract, least-privilege fixtures, per-message WebSocket/ACIP authorization, and fail-closed tests. |
| WP-S3 | [Secrets, keys, and cryptographic trust](SECURITY_WP_S3_SECRETS_KEYS_CRYPTOGRAPHIC_TRUST_v0.93.md) | Key/secrets lifecycle contract covering custody, signing, encryption, rotation, revocation, sealed-state access, internal ACIP encryption requirements, and WebSocket-carried ACIP signing/encryption boundaries. |
| WP-S4 | [Audit, compliance, and incident evidence](SECURITY_WP_S4_AUDIT_COMPLIANCE_INCIDENT_EVIDENCE_v0.93.md) | Tamper-evident audit schema, compliance-evidence packet, incident record, redacted reviewer report, and non-certification language. |
| WP-S5 | [Isolation, data governance, and privacy](SECURITY_WP_S5_ISOLATION_DATA_GOVERNANCE_PRIVACY_v0.93.md) | Tenant/polis isolation, data classification, retention, deletion, projection, private-state privacy, and leakage negative cases. |
| WP-S6 | [Security operations, adversarial regression, and provenance](SECURITY_WP_S6_SECURITY_OPERATIONS_ADVERSARIAL_PROVENANCE_v0.93.md) | Security-ops runbook, threat-board hygiene, red/blue regression checks, supply-chain/provenance checks, runtime-hardening evidence, and incident-response drill. |

## Dependencies

- v0.89.1 adversarial runtime and red/blue proof surfaces.
- v0.90.3 citizen state, access control, sealed private state, redacted
  projection, sanctuary, challenge, and quarantine.
- v0.90.5 governed tools, Universal Tool Schema, and ADL Capability Contract.
- v0.91 moral trace, secure Agent Comms, and ACIP planning.
- v0.92 identity, continuity, capability envelopes, and birthday semantics.
- v0.92 ACIP binary schema, public schema catalog, and mock WebSocket carrier
  readiness.

## Non-goals

- No external certification claim.
- No production SOC 2, ISO 27001, FedRAMP, HIPAA, or legal-compliance claim.
- No production WebSocket or cross-polis networking claim until transport
  security, per-message authorization, and trace/replay prerequisites exist.
- No bypass of v0.90.3 private-state, access-control, or projection rules.
- No replacement of v0.91 moral trace or v0.92 identity semantics.
- No hidden environment-only security model.

## Proof Expectations

Each security WP should produce at least one concrete proof surface:

- enforceable policy fixture
- deny-by-default negative case
- cryptographic lifecycle fixture
- tamper-evident audit or incident record
- redacted compliance-evidence packet
- isolation or leakage-prevention test
- adversarial regression or provenance check
- runtime-hardening report
- WebSocket/ACIP negative fixtures for unauthorized, malformed, oversized,
  replayed, duplicated, or out-of-order session events where transport security
  is in scope

## Demo Candidates

- Zero-trust denied action across a citizen/tool/service boundary.
- Delegated tool action accepted only with standing, identity, capability, and
  policy authority.
- Internal ACIP message accepted or rejected based on key, state, and authority.
- WebSocket-carried ACIP/session event denied because the connection, schema,
  message authority, sequence, or policy context is invalid.
- Key rotation and revocation changes signature/message acceptance.
- Governance review packet cites incident and audit evidence without leaking
  raw private state.
- Isolation negative case prevents cross-polis or cross-citizen data leakage.

## Review Boundary

The review packet should answer:

- Which actor acted or attempted to act?
- Which boundary was crossed?
- Which policy allowed or denied the action?
- If the action arrived over WebSocket or ACIP transport, which connection,
  schema, sequence, and message-authority checks were applied?
- Which key, signature, encryption, or revocation evidence was used?
- Which audit, incident, provenance, or adversarial evidence exists?
- What was redacted, retained, deleted, or isolated?
- Which certification or production-security claims are explicitly not made?

## Completion Target

`v0.93`
