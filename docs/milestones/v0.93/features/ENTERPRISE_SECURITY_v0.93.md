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

## Six Security WPs

| WP | Name | Required work product |
| --- | --- | --- |
| WP-S1 | [Zero-trust architecture](SECURITY_WP_S1_ZERO_TRUST_ARCHITECTURE_v0.93.md) | Trust-boundary contract, actor/zone model, default-deny fixtures, and unauthorized-boundary negative cases. |
| WP-S2 | [Policy enforcement and authorization](SECURITY_WP_S2_POLICY_ENFORCEMENT_AUTHORIZATION_v0.93.md) | IAM/delegation/standing/tool-authority policy decision contract, least-privilege fixtures, and fail-closed tests. |
| WP-S3 | [Secrets, keys, and cryptographic trust](SECURITY_WP_S3_SECRETS_KEYS_CRYPTOGRAPHIC_TRUST_v0.93.md) | Key/secrets lifecycle contract covering custody, signing, encryption, rotation, revocation, sealed-state access, and internal ACIP encryption requirements. |
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

## Non-Goals

- No external certification claim.
- No production SOC 2, ISO 27001, FedRAMP, HIPAA, or legal-compliance claim.
- No cross-polis networking before transport-security prerequisites exist.
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

## Demo Candidates

- Zero-trust denied action across a citizen/tool/service boundary.
- Delegated tool action accepted only with standing, identity, capability, and
  policy authority.
- Internal ACIP message accepted or rejected based on key, state, and authority.
- Key rotation and revocation changes signature/message acceptance.
- Governance review packet cites incident and audit evidence without leaking
  raw private state.
- Isolation negative case prevents cross-polis or cross-citizen data leakage.

## Review Boundary

The review packet should answer:

- Which actor acted or attempted to act?
- Which boundary was crossed?
- Which policy allowed or denied the action?
- Which key, signature, encryption, or revocation evidence was used?
- Which audit, incident, provenance, or adversarial evidence exists?
- What was redacted, retained, deleted, or isolated?
- Which certification or production-security claims are explicitly not made?
