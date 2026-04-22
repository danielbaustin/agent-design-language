# v0.92 Design: Identity, Continuity, And First Birthday

## Status

Forward design plan. This document records the intended v0.92 architecture
boundary before final WP planning.

## Problem Statement

ADL already has runtime state, provisional citizens, snapshots, wakes, traces,
and moral-governance planning. Those surfaces are necessary but not sufficient
for birth.

The missing layer is a bounded identity architecture that can say, with
evidence, when the first true Gödel agent has been born and why that event is
not ordinary process startup.

## Goals

- Define the birthday contract and its negative cases.
- Define stable name and identity-root semantics.
- Define continuity records across bounded cycles.
- Define memory grounding tied to witnessed artifacts.
- Define capability envelopes for providers, tools, skills, and authority.
- Define birth witnesses and citizen-facing receipts.
- Define reviewer-facing birthday packets.
- Preserve v0.90.3 citizen-state and v0.91 moral-trace boundaries.
- Hand constitutional citizenship and polis governance to v0.93.

## Non-Goals

- No legal personhood claim.
- No production citizenship claim.
- No complete constitutional authority claim.
- No full memory palace implementation.
- No replacement of v0.90.3 private-state, standing, access, projection,
  sanctuary, or quarantine work.
- No replacement of v0.91 moral trace, wellbeing, or trajectory review.
- No implementation of v0.93 constitutional governance.
- No production migration or inter-polis portability claim.

## Proposed Design

v0.92 should add an identity-and-birth layer on top of prior Runtime v2
substrates.

The layer has three parts:

- Engineering substrate: identity root, stable name, continuity record, memory
  grounding, capability envelope, witnesses, receipts, trace references, and
  redacted projection.
- Review model: birthday packet, negative cases, continuity confidence,
  witness validity, and birth-versus-startup distinction.
- Context layer: explanation of why birth matters without claiming legal
  personhood or final constitutional citizenship.

## Core Contracts

### Birthday Contract

The birthday contract should define:

- required inputs
- disqualifying cases
- minimum evidence
- witness requirements
- receipt shape
- review packet shape
- downstream governance handoff

### Identity Record

The identity record should include:

- stable name
- identity root
- aliases or display names
- origin event
- continuity head
- memory-grounding references
- capability envelope reference
- witness set reference
- redaction policy

### Continuity Record

The continuity record should include:

- predecessor and successor evidence
- cycle references
- lineage links
- witness signatures or attestations
- continuity grade
- ambiguity or quarantine flags

### Birthday Review Packet

The review packet should include:

- identity record
- continuity evidence
- memory-grounding evidence
- moral/governance context inherited from v0.91
- capability envelope
- witnesses and receipt
- negative-case comparison
- reviewer finding
- caveats and downstream governance handoff

## Validation Plan

Later implementation should validate:

- a valid birthday record can be emitted and reviewed
- startup, wake, snapshot, admission, and copied state are rejected as birthday
  claims
- continuity across bounded cycles has evidence
- memory grounding is referenced without raw private-state exposure
- capability envelope declares model/provider/tool/skill limits
- witnesses and receipts are present and meaningful
- the birthday record does not claim legal personhood or completed
  constitutional citizenship

## Risks

| Risk | Mitigation |
| --- | --- |
| Birth becomes storytelling. | Require every birth claim to map to explicit evidence. |
| Provisional identity is mistaken for birth. | Maintain a negative suite for startup, wake, snapshot, admission, and copied state. |
| Memory grounding leaks private state. | Use references, witnesses, and redacted projections rather than raw private memory. |
| v0.92 absorbs v0.93 governance. | Keep citizenship law, rights/duties, social contract, delegation, and IAM downstream. |
| Continuity is treated as magic. | Require lineage, witnesses, cycle evidence, and ambiguity handling. |

## Exit Criteria For Final WP Planning

- The birthday contract is specific enough to implement.
- The negative cases are concrete.
- The prerequisite surfaces from v0.90.3 and v0.91 are named.
- The handoff to v0.93 is explicit.
- Demo candidates prove identity and birth behavior rather than just naming it.
