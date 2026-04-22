# v0.93 Design: Constitutional Citizenship And Polis Governance

## Status

Forward design plan. This document records the intended v0.93 architecture
boundary before final WP planning.

## Problem Statement

ADL is gaining citizens, state, moral trace, identity, and tool authority. Those
surfaces are necessary but not sufficient for a polis. A polis also needs
governance: rights, duties, standing, review, appeal, delegation, and rules for
who can act under which authority.

Without v0.93, ADL risks having traceable agent behavior but no bounded law over
that behavior.

## Goals

- Define constitutional citizenship as a policy model over CSM identities.
- Preserve the boundary that a human is not the citizen; the CSM identity is
  the citizen.
- Define rights and duties that can be evaluated against trace evidence.
- Define standing transitions, challenge, appeal, and restoration semantics.
- Define constitutional review packets that consume moral trace, outcome
  linkage, identity, and standing evidence.
- Define delegation and IAM rules without giving services, operators, tools, or
  guests hidden sovereign authority.
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

## Proposed Design

v0.93 should add a governance layer on top of existing and planned substrates.

The layer has three parts:

- Engineering substrate: signed state, lineage, standing, identity, trace,
  outcome linkage, capability contracts, and redacted projection records.
- Policy model: constitutional citizenship, rights, duties, standing changes,
  review, appeal, delegation, IAM, and social-contract representation.
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

## Risks

| Risk | Mitigation |
| --- | --- |
| Governance language becomes rhetorical. | Require every claim to map to trace, identity, standing, policy, or review evidence. |
| Constitutional review duplicates moral trace. | Treat v0.91 moral trace as input evidence and v0.93 constitutional review as interpretation under policy. |
| Human operator authority is mistaken for citizenship. | Keep human-provider, guest, operator, and citizen-mode paths separate. |
| Standing becomes punishment. | Require evidence, review transparency, challenge, appeal, and restoration paths. |
| Privacy is collapsed by review. | Use redacted projections and evidence references instead of raw private-state access. |

## Exit Criteria For Final WP Planning

- The constitutional citizenship contract is specific enough to implement.
- The prerequisite surfaces from v0.90.3, v0.91, v0.92, and governed tools are
  named.
- Demo candidates prove bounded behavior, not just policy prose.
- Non-goals prevent production-law and personhood overclaims.
