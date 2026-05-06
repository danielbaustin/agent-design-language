# v0.93 Design: Constitutional Citizenship And Polis Governance

## Status

Forward design plan. This document records the intended v0.93 architecture
boundary before final WP planning.

## Problem Statement

ADL is gaining citizens, state, moral trace, identity, social cognition, and
tool authority. Those surfaces are necessary but not sufficient for a polis. A
polis also needs governance: rights, duties, standing, review, appeal,
delegation, and rules for who can act under which authority.

Without v0.93, ADL risks having traceable agent behavior but no bounded law over
that behavior.

## Goals

- Define constitutional citizenship as a policy model over CSM identities.
- Preserve the boundary that a human is not the citizen; the CSM identity is
  the citizen.
- Define rights and duties that can be evaluated against trace evidence.
- Define Theory of Mind as private evidence-grounded social cognition, distinct
  from reputation, standing, and constitutional judgment.
- Define shared social memory as redacted, challengeable governance evidence
  rather than leaked private models.
- Define standing transitions, challenge, appeal, and restoration semantics.
- Define constitutional review packets that consume moral trace, outcome
  linkage, identity, and standing evidence.
- Define delegation and IAM rules without giving services, operators, tools, or
  guests hidden sovereign authority.
- Define enterprise-security foundations for zero-trust, policy enforcement,
  key/secrets lifecycle, audit/compliance evidence, isolation/data governance,
  and security operations.
- Produce reviewer-facing proof candidates that separate engineering substrate,
  policy model, and contextual claims.

## Non-Goals

- No legal personhood claim.
- No production constitution or real-world legal authority.
- No complete social-contract theory.
- No replacement of v0.90.3 citizen-state, access, projection, or quarantine
  work.
- No replacement of v0.91 moral trace or v0.92 identity/birthday work.
- No economics, payment rails, or market settlement unless later milestone
  planning deliberately creates a narrow bridge.
- No scalar reputation, karma score, or moral leaderboard.
- No private ToM model treated as public reputation, standing, or constitutional
  review outcome.
- No claim of external enterprise certification or production compliance
  approval.
- No external or cross-polis communication without the required transport
  security and authority prerequisites.

## Proposed Design

v0.93 should add a governance layer on top of existing and planned substrates.

The layer has three parts:

- Engineering substrate: signed state, lineage, standing, identity, trace,
  outcome linkage, capability contracts, and redacted projection records.
- Social-cognition model: private ToM, reputation projections, shared social
  memory, uncertainty, conflict, decay, and redaction policy.
- Policy model: constitutional citizenship, rights, duties, standing changes,
  review, appeal, delegation, IAM, and social-contract representation.
- Security model: zero-trust boundaries, least-privilege policy enforcement,
  cryptographic trust, secrets/key lifecycle, audit/compliance records,
  isolation, data governance, incident response, provenance, and adversarial
  regression.
- Context layer: philosophical and civic vocabulary that explains why the polis
  needs law, without treating those explanations as implemented behavior.

## Core Contracts

### Constitutional Citizen Record

The citizen-facing governance record should reference:

- citizen identity and continuity evidence
- current standing state
- applicable rights and duties
- relevant policy set
- active restrictions or delegations
- challenge or appeal state
- redaction and projection permissions

This record should reference private state and moral trace evidence without
embedding raw private state.

### Constitutional Review Packet

A review packet should include:

- review scope
- citizen identity
- standing snapshot
- challenged or reviewed conduct
- moral trace event references
- outcome and attribution references
- applicable rights, duties, and policy
- delegation/IAM authority chain when relevant
- finding, severity, uncertainty, and appeal status
- redaction disposition

### Theory Of Mind And Social Cognition Contract

The social-cognition contract should reference:

- target actor identity and standing
- model owner or maintaining actor
- evidence references for beliefs, intentions, capabilities, and uncertainty
- confidence and confidence basis for every modeled entry
- conflict, decay, and freshness state
- policy and authority context for creation, inspection, and projection
- signed update events and model-version links
- redaction class for any public, reviewer, operator, or citizen-facing view

Private ToM may inform arbitration, Freedom Gate evaluation, delegation, and
constitutional review only through authorized projections. It must not bypass
standing, access control, redaction, appeal, or policy.

### Authority Boundary

The design must represent these actor classes distinctly:

- citizen
- guest
- human provider
- operator
- service actor
- tool or tool adapter
- external counterparty

Human input is allowed, but citizen action requires identity binding, Freedom
Gate mediation, signed trace, and temporal anchoring. Direct out-of-band human
action is operator or guest activity, not citizen conduct.

### Enterprise Security Boundary

Enterprise security in v0.93 should be implemented as polis governance, not as
an afterthought around the edges of the runtime. The security model should make
every actor, tool, service, citizen, operator, policy decision, key, secret,
communication, data projection, and audit record part of the same reviewable
authority surface.

The six planned security WPs are:

- Zero-trust architecture: no implicit trust across citizen, operator, service,
  tool, polis, communication, or data boundaries.
- Policy enforcement and authorization: IAM, delegation, standing, tool
  authority, and capability checks fail closed.
- Secrets, keys, and cryptographic trust: signing, encryption, key custody,
  rotation, revocation, and sealed-state access are lifecycle-managed.
- Audit, compliance, and incident evidence: review packets can prove what
  happened without leaking private state or claiming external certification.
- Isolation, data governance, and privacy: tenant/polis boundaries, retention,
  deletion, projection, and redaction are explicit.
- Security operations, adversarial regression, and provenance: red/blue tests,
  supply-chain evidence, runtime hardening, threat-board hygiene, and incident
  drills feed release review.

## Validation Plan

Later implementation should validate:

- a citizen action can be reviewed against policy with trace evidence
- a malformed or evidence-free review cannot be accepted as constitutional
  evidence
- a standing change requires cited evidence and preserves appeal context
- a human guest transcript does not become citizen action without mediation
- delegated action is allowed or denied based on explicit authority chain
- communication does not grant inspection
- redacted projections do not leak private state
- zero-trust boundaries deny unauthenticated, unauthorized, stale, or
  overprivileged actions
- key rotation and revocation change what signatures, messages, and sealed
  state can be accepted
- audit and incident packets are tamper-evident and redaction-safe
- isolation and data-governance fixtures prevent cross-polis, cross-tenant, and
  cross-citizen leakage
- adversarial regression and provenance checks are present in the release
  evidence
- a ToM update requires evidence, confidence, signed trace, and authority
- reputation projection does not expose the private ToM model
- conflict and decay are explicit rather than silently overwritten

## Risks

| Risk | Mitigation |
| --- | --- |
| Governance language becomes rhetorical. | Require every claim to map to trace, identity, standing, policy, or review evidence. |
| Constitutional review duplicates moral trace. | Treat v0.91 moral trace as input evidence and v0.93 constitutional review as interpretation under policy. |
| Human operator authority is mistaken for citizenship. | Keep human-provider, guest, operator, and citizen-mode paths separate. |
| Standing becomes punishment. | Require evidence, review transparency, challenge, appeal, and restoration paths. |
| Privacy is collapsed by review. | Use redacted projections and evidence references instead of raw private-state access. |
| ToM becomes hidden reputation. | Keep private ToM, public reputation, standing, and constitutional review separate, with authorized projections between them. |
| Social cognition becomes prompt theater. | Require signed update events, model versions, evidence references, conflict handling, and decay semantics. |
| Enterprise security becomes a checklist label. | Require each security WP to produce concrete enforcement, fixture, audit, incident, isolation, provenance, or adversarial-regression evidence. |
| IAM and zero-trust drift apart. | Treat IAM as one enforcement layer inside the broader zero-trust trust-boundary model. |
| Secrets and keys stay hidden in environment folklore. | Define lifecycle contracts for custody, rotation, revocation, signing, encryption, and sealed-state access. |
| Compliance language overclaims certification. | Describe evidence packets and controls only; do not claim SOC 2, ISO 27001, FedRAMP, HIPAA, or external approval. |

## Exit Criteria For Final WP Planning

- The constitutional citizenship contract is specific enough to implement.
- The prerequisite surfaces from v0.90.3, v0.91, v0.92, and governed tools are
  named.
- The six enterprise-security WPs are explicit enough to become issue cards
  with proof surfaces.
- Demo candidates prove bounded behavior, not just policy prose.
- Non-goals prevent production-law and personhood overclaims.
