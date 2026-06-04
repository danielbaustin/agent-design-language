# CAV, Threat Model, And CodeFriend Security Scheduling v0.91.5

## Status

Planning packet prepared for issue `#3675`.

This document schedules three related security-architecture drafts into later
ADL and CodeFriend work. It is not an implementation closeout record, runtime
security proof, penetration-test report, external certification claim, or
production security-operations claim.

## Purpose

ADL has three related security-architecture source surfaces captured in the
reviewable source packet
`docs/milestones/v0.91.5/features/CAV_THREAT_MODEL_AND_CODEFRIEND_SECURITY_SOURCE_PACKET_v0.91.5.md`:

- Continuous Adversarial Verification (CAV)
- ADL Threat Model
- CodeFriend Security Model

They share a security thesis: every important capability creates an attack
surface, and important capabilities should eventually have adversarial
counterparts that discover, reproduce, classify, mitigate, and regression-test
failures.

The scheduling problem is that these drafts are related to enterprise security
without being identical to enterprise-security feature work:

- The ADL Threat Model is an ADL-wide architecture input.
- CAV is an ADL-wide adversarial verification architecture input.
- The CodeFriend Security Model is a product/application-lane input that consumes
  ADL security architecture, but should not silently become core ADL runtime
  scope.

## Source Packet

The durable source basis for this scheduling decision is:

- `docs/milestones/v0.91.5/features/CAV_THREAT_MODEL_AND_CODEFRIEND_SECURITY_SOURCE_PACKET_v0.91.5.md`

That source packet summarizes the local planning drafts in a tracked,
reviewer-safe form. The local `.adl` drafts are not required for PR review.

## Source Surfaces

| Draft | Current role | Future owner lane | Scheduling decision |
| --- | --- | --- | --- |
| ADL Threat Model | Defines what can be attacked, how attacks are classified, and what must be defended. | ADL security architecture; v0.93 enterprise/security planning consumer. | Promote into the v0.93 security planning surface as the threat taxonomy and control-area input. |
| Continuous Adversarial Verification | Defines continuous exploit discovery, reproduction, mitigation verification, replay, and security-corpus growth. | ADL adversarial verification architecture; v0.93 WP-S6 planning consumer. | Promote into WP-S6 as the security-operations/adversarial-regression doctrine. |
| CodeFriend Security Model | Defines CodeFriend's correctness/security/adversarial/constitutional review model. | CodeFriend/product lane; consumes ADL security architecture. | Route to CodeFriend planning as an application of CAV and threat modeling, not as a core ADL runtime feature. |

## Relationship To The Enterprise-Security Boundary

Issue `#3538` plans the enterprise-security organization boundary: how ADL should
separate enterprise-security capability bands from core runtime work before large
module, crate, or repository movement.

This packet should account for that boundary, but it does not redefine it.

The relationship is:

- CAV and the ADL Threat Model are upstream security-architecture inputs.
- v0.93 enterprise security consumes those inputs where they affect zero trust,
  policy enforcement, cryptographic trust, audit, isolation, adversarial
  regression, provenance, and incident evidence.
- CodeFriend security consumes those inputs as a product review architecture.
- None of the three source drafts automatically becomes an enterprise-security
  implementation feature merely because it is security-related.

This distinction matters because ADL should preserve a clean architecture:
security doctrine, enterprise-security feature execution, and product-specific
review operations are connected but not interchangeable.

## v0.93 Scheduling

### ADL Threat Model

Schedule as an input to v0.93 security planning.

Expected future work:

- normalize threat categories into a reviewed taxonomy
- map threats to v0.93 control areas
- require each security WP to declare which threat classes it addresses
- preserve non-claims around external certification and production readiness

Primary consumers:

- `docs/milestones/v0.93/features/SECURITY_WP_S1_ZERO_TRUST_ARCHITECTURE_v0.93.md`
- `docs/milestones/v0.93/features/SECURITY_WP_S2_POLICY_ENFORCEMENT_AUTHORIZATION_v0.93.md`
- `docs/milestones/v0.93/features/SECURITY_WP_S5_ISOLATION_DATA_GOVERNANCE_PRIVACY_v0.93.md`
- `docs/milestones/v0.93/features/SECURITY_WP_S6_SECURITY_OPERATIONS_ADVERSARIAL_PROVENANCE_v0.93.md`

### Continuous Adversarial Verification

Schedule as the doctrine input to v0.93 WP-S6.

Expected future work:

- define exploit artifact schema
- define security-corpus record shape
- define replay and regression contract
- define red/blue/purple responsibility boundaries
- define security tournament as future research, not first implementation scope
- require bounded targets, stop conditions, and reviewer-safe evidence

Primary consumers:

- `docs/milestones/v0.93/RED_BLUE_ADVERSARIAL_SECURITY_ISSUE_WAVE_v0.93.md`
- `docs/milestones/v0.93/features/SECURITY_WP_S6_SECURITY_OPERATIONS_ADVERSARIAL_PROVENANCE_v0.93.md`

### CodeFriend Security Model

Schedule as a CodeFriend/product-lane input, not as a core ADL security WP.

Expected future work:

- connect CodeFriend review providers to four review perspectives:
  correctness, security, adversarial, and constitutional review
- require independent review findings and synthesis rather than a single
  undifferentiated model opinion
- preserve ADL governance and trace/evidence compatibility
- avoid claiming CodeFriend has complete CAV, SOC, or certification capability

Primary consumers:

- future CodeFriend planning docs and issue waves
- review-provider role/profile work when CodeFriend external review lanes are
  implemented
- v0.93 security work only where product security evidence needs to cite ADL-wide
  threat/CAV doctrine

## Follow-On Issue Candidates

These are issue candidates, not work created by this packet.

| Candidate | Owner lane | Purpose | Non-goals |
| --- | --- | --- | --- |
| `[v0.93][security] Promote ADL threat model into reviewed control taxonomy` | ADL security architecture | Convert the local threat-model draft into reviewed v0.93 threat taxonomy and map it to security WPs. | No production certification claim; no runtime policy rewrite. |
| `[v0.93][security][WP-S6] Define CAV exploit artifact and security corpus schema` | ADL WP-S6 | Define replayable exploit, mitigation, verification, and regression artifact shapes. | No unbounded penetration testing; no security tournament runtime. |
| `[v0.93][security][WP-S6] Add bounded CAV regression seeds to red/blue issue wave` | ADL WP-S6 | Seed adversarial regression cases from the threat taxonomy and CAV loop. | No external targets; no uncontrolled exploit execution. |
| `[CodeFriend][security] Promote CodeFriend security model into product review architecture` | CodeFriend/product lane | Route correctness, security, adversarial, and constitutional review perspectives into CodeFriend planning. | No claim that CodeFriend replaces human security review or compliance audit. |
| `[v0.95+][research] Evaluate security tournaments as scaled CAV research` | research/future | Study multi-agent red/blue tournaments after bounded replay/corpus mechanics exist. | No early tournament implementation inside v0.93. |

## Required Boundaries

- Every red-team or adversarial action must have a declared ADL-owned target,
  allowed mutation boundary, and stop condition.
- Every discovered exploit should become a replayable artifact before it becomes
  release evidence.
- Every mitigation should have a verification result and residual-risk record.
- Every CodeFriend security claim should distinguish product review behavior from
  core ADL runtime security behavior.
- Every public-facing security artifact should avoid raw secrets, raw private
  state, raw exploit payloads when unsafe, host-local absolute paths, and
  certification overclaim.

## Non-Claims

This packet does not claim:

- CAV is implemented
- a security corpus exists
- security tournaments are implemented
- CodeFriend has production security-review operations
- ADL has SOC, ISO, FedRAMP, HIPAA, or other external certification status
- v0.93 enterprise security is complete
- the enterprise-security repo/module boundary from `#3538` is implemented

## Validation Expectations

For this scheduling packet:

- focused Markdown and path hygiene
- source-doc classification review
- no host-local absolute path leakage
- no broad Rust validation
- bounded subagent review for overclaim, routing, and enterprise-security boundary
  confusion

For future implementation issues:

- each issue must declare proof lane, artifact owner, release-gate posture, and
  replay or regression requirements
- each adversarial test must declare target, authority, stop condition, and
  reviewer-safe evidence
- each product-lane CodeFriend issue must preserve the ADL/product boundary
