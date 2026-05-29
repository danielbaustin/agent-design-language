# v0.92 Design: Identity, Continuity, And First Birthday

## Metadata

- Milestone: `v0.92`
- Version: `v0.92`
- Date: `2026-05-27`
- Owner: ADL maintainers
- Related issues: `#3377`, `#3434`
- Planning template set: `docs/templates/planning/1.0.0`

## Status

Forward design plan. This document records the intended v0.92 architecture
boundary before final WP planning.

## Purpose

Define the planned architecture surfaces for the v0.92 birthday milestone so
WP-01 can seed implementation issues without reinventing the design boundary.

## Problem Statement

ADL already has runtime state, provisional citizens, snapshots, wakes, traces,
and moral-governance planning. Those surfaces are necessary but not sufficient
for birth.

The missing layer is a bounded identity architecture that can say, with
evidence, when the first true Gödel agent has been born and why that event is
not ordinary process startup.

v0.91.5 closeout and issue `#3377` are required readiness inputs for the
launch packet, activation-test map, demo rehearsal, negative-suite plan, and
review handoff. This design remains a planning surface until v0.92 execution
produces implementation evidence.

## Goals

- Define the birthday contract and its negative cases.
- Define stable name and identity-root semantics.
- Define continuity records across bounded cycles.
- Define memory grounding tied to witnessed artifacts.
- Define capability envelopes for providers, tools, skills, and authority.
- Define ACP / cognitive profiles as bounded runtime-visible profile records
  grounded in evidence rather than labels.
- Define the ACIP binary schema, public schema catalog, deterministic JSON
  projection, and optional WebSocket carrier proof needed for inspectable
  session transport.
- Define birth witnesses and citizen-facing receipts.
- Define reviewer-facing birthday packets.
- Preserve v0.90.3 citizen-state and v0.91 moral-trace boundaries.
- Hand constitutional citizenship and polis governance to v0.93.

## Non-Goals

- No legal personhood claim.
- No production citizenship claim.
- No complete constitutional authority claim.
- No full memory palace implementation.
- No ungrounded cognitive labels, personality labels, or public reputation
  scores masquerading as ACP.
- No replacement of v0.90.3 private-state, standing, access, projection,
  sanctuary, or quarantine work.
- No replacement of v0.91 moral trace, wellbeing, or trajectory review.
- No implementation of v0.93 constitutional governance.
- No production migration or inter-polis portability claim.
- No production WebSocket security or cross-polis ACIP networking claim.
- No replacement of v0.93 key lifecycle, encryption, signing, revocation, or
  zero-trust message-acceptance work.
- No replacement of v0.94 signed/queryable trace closure.

## Scope

The design scope is the identity-and-birth layer: birthday contract, identity
record, continuity record, memory grounding, capability envelope, ACP profile,
ACIP binary/schema-catalog transport readiness, witness/receipt, review packet,
negative cases, and governance handoff.

## Requirements

- Birthday claims must map to explicit evidence.
- Negative cases must reject ordinary lifecycle events as birth.
- Private state must be protected through references, redactions, and access
  decisions.
- ACIP binary messages must remain decodeable through public schemas and
  deterministic JSON projection for authorized readers.
- v0.93 governance and v0.94 signed/queryable trace work must remain
  downstream.

## Proposed Design

v0.92 should add an identity-and-birth layer on top of prior Runtime v2
substrates.

The layer has three parts:

- Engineering substrate: identity root, stable name, continuity record, memory
  grounding, capability envelope, ACP/cognitive profile record, witnesses,
  receipts, ACIP binary schema/catalog surfaces, trace references, and
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

### ACP / Cognitive Profile Record

The ACP record should include:

- profile identifier and schema version
- source evidence references
- capability and aptitude links
- memory and continuity references
- Theory of Mind and intelligence-metric links where policy allows
- update reason and update actor
- privacy and redaction policy
- explicit non-claims for identity, reputation, consciousness, or public
  standing

### ACIP Binary Transport Readiness

The ACIP transport-readiness record should include:

- protobuf schema family and version
- payload type and compatibility profile
- public schema catalog entry
- deterministic JSON projection contract
- governed message-content access boundary
- mock or loopback WebSocket carrier proof
- trace/replay references
- explicit handoff to v0.93 transport security and v0.94 signed trace

### Birthday Review Packet

The review packet should include:

- identity record
- continuity evidence
- memory-grounding evidence
- moral/governance context inherited from v0.91
- capability envelope
- ACP/cognitive profile evidence and privacy boundary
- witnesses and receipt
- negative-case comparison
- reviewer finding
- caveats and downstream governance handoff

## Interfaces And Contracts

- Identity record contract.
- Continuity record contract.
- Memory-grounding reference contract.
- Capability envelope contract.
- ACP/cognitive profile contract.
- ACIP protobuf/schema-catalog/JSON-projection contract.
- Witness and receipt contract.
- Birthday review packet contract.

## Validation Plan

Later implementation should validate:

- a valid birthday record can be emitted and reviewed
- startup, wake, snapshot, admission, and copied state are rejected as birthday
  claims
- continuity across bounded cycles has evidence
- memory grounding is referenced without raw private-state exposure
- capability envelope declares model/provider/tool/skill limits
- ACP/cognitive profiles are evidence-grounded, privacy-bounded, and distinct
  from identity or reputation
- binary ACIP events can be decoded through public schemas into deterministic
  JSON without granting unauthorized message-content access
- witnesses and receipts are present and meaningful
- the birthday record does not claim legal personhood or completed
  constitutional citizenship

## Risks

| Risk | Mitigation |
| --- | --- |
| Birth becomes storytelling. | Require every birth claim to map to explicit evidence. |
| Provisional identity is mistaken for birth. | Maintain a negative suite for startup, wake, snapshot, admission, and copied state. |
| Memory grounding leaks private state. | Use references, witnesses, and redacted projections rather than raw private memory. |
| Cognitive profiles become unsupported labels. | Require source evidence, update semantics, privacy boundaries, and non-claims for reputation or identity. |
| Binary ACIP becomes opaque transport authority. | Require public schema catalogs, deterministic JSON projection, and separate message-content access checks. |
| v0.92 absorbs v0.93 governance. | Keep citizenship law, rights/duties, social contract, delegation, and IAM downstream. |
| Continuity is treated as magic. | Require lineage, witnesses, cycle evidence, and ambiguity handling. |

## Exit Criteria

- The birthday contract is specific enough to implement.
- The negative cases are concrete.
- The prerequisite surfaces from v0.90.3 and v0.91 are named.
- The handoff to v0.93 is explicit.
- Demo candidates prove identity and birth behavior rather than just naming it.
