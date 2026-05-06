# v0.93 Sprint Plan

## Status

Forward-planning sprint outline. The final sprint and WP sequence will be
authored during v0.93 WP-01.

## Sprint Goal

Build the first reviewable constitutional governance and social-cognition layer
for the ADL polis: citizenship, rights, duties, Theory of Mind, reputation
boundary, shared social memory, standing, challenge, appeal, delegation, IAM,
enterprise security, and governance evidence over prior trace and identity
substrates.

## Planned Phases

| Phase | Focus | Expected outcome |
| --- | --- | --- |
| 1 | Planning promotion | Reviewed milestone docs, issue wave, and cards. |
| 2 | Constitutional citizenship core | Citizenship contract, human/citizen boundary, rights, duties, and standing states. |
| 3 | Social cognition | ToM schema, signed update events, reputation boundary, shared social memory, conflict, decay, and projection rules. |
| 4 | Review and appeal | Constitutional review packet, challenge flow, appeal flow, and evidence rules. |
| 5 | Delegation and IAM | Authority-chain rules across citizens, guests, operators, services, tools, and external actors. |
| 6 | Enterprise security foundation | Zero-trust architecture, enforcement, key/secrets lifecycle, audit/compliance evidence, isolation/data governance, and security operations. |
| 7 | Social contract and communication | Bounded social-contract representation and communication-without-inspection proof. |
| 8 | Demo and review tail | Governance and enterprise-security demos, demo matrix, quality gate, review handoff, and release ceremony. |

## Dependencies To Check Before WP-01

- v0.90.3 citizen-state and standing outputs are stable enough to consume.
- v0.91 moral trace and review planning is available.
- v0.92 identity and birthday planning remains a prerequisite, not absorbed
  into v0.93.
- Governed-tool and economics lanes are either available as prerequisites or
  explicitly deferred.
- v0.91 secure Agent Comms, v0.90.5 governed-tool authority, and v0.92 identity
  provide enough substrate for zero-trust, key/secrets, and policy-enforcement
  work.
- ToM source material is reconciled with v0.90.3 standing/access and no longer
  carries stale late-roadmap-only targeting.

## Demo And Review Plan

At least one later demo should show constitutional review of a challenged
citizen action. Strong secondary demos include standing restoration, human
guest versus mediated citizen-mode action, ToM/reputation boundary,
delegation/IAM, zero-trust deny-by-default behavior, audit/compliance evidence,
key rotation/revocation, isolation leakage prevention, and communication without
inspection.

The review packet should make it easy to answer:

- What actor acted?
- Under what standing and authority?
- What evidence supports the governance finding?
- Was any ToM-derived signal used, and was it authorized, redacted, and
  challengeable?
- What was hidden or redacted?
- What appeal or restoration path remains?
- Which security boundary was crossed or denied?
- Which key, policy, audit, isolation, or provenance evidence proves the
  decision?

## Exit Criteria For Active Planning

- The WBS is converted from candidate areas into concrete WPs.
- Every implementation WP has a code, fixture, test, demo, or reviewable docs
  output.
- Every demo maps to a governance claim.
- Every security WP maps to a concrete enforcement, fixture, audit, incident,
  isolation, provenance, or adversarial-regression proof surface.
- Non-goals prevent overclaiming legal personhood or production citizenship.
