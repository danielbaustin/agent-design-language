# ACIP Binary Schema And WebSocket Transport

## Metadata

- Feature Name: ACIP Binary Schema And WebSocket Transport
- Milestone Target: `v0.92`
- Status: planned
- Source Docs:
  - `.adl/docs/TBD/acip/ACIP_SCHEMA_CATALOG_AND_MESSAGE_ACCESS_RULES_2026-05-20.md`
  - `.adl/docs/TBD/WEBSOCKET_TRANSPORT_SUPPORT_PLAN_2026-05-20.md`
  - `docs/explainers/ACIP.md`
  - `docs/milestones/v0.91.1/features/ACIP_HARDENING.md`
- Proof Modes: schema, fixtures, round-trip tests, mock transport proof

## Purpose

Promote ACIP from a local structured JSON/serde communication substrate into a
binary, schema-governed session transport surface without losing
inspectability, citizen access boundaries, or ADL authority semantics.

The first real network/session ACIP carrier should use protobuf bytes on the
wire. JSON remains the deterministic projection and fixture format used for
debugging, review, trace, and citizen-facing inspection when access is allowed.

## Scope

In scope:

- Canonical ACIP protobuf schema family.
- Public polis schema catalog rules.
- Binary envelope metadata for schema family, schema version, compatibility
  profile, payload type, payload digest, and transport/session identity.
- Deterministic protobuf-to-JSON projection.
- JSON/protobuf round-trip fixtures for in-scope ACIP messages and invocation
  events.
- Optional WebSocket carrier for binary ACIP session events.
- Mock or loopback transport proof before any live-provider proof.
- Fail-closed behavior for missing, unknown, unavailable, private, deprecated,
  or mismatched schemas.

Out of scope:

- Production cross-polis networking.
- Live provider WebSocket dependency as the first proof.
- Browser WebRTC.
- Replacing HTTP/local providers.
- Treating provider session state as ADL memory, identity, trace, or truth.
- Runtime implementation of v0.93 key lifecycle, encryption, rotation, or
  revocation.
- v0.94 signed/queryable trace completion.

## Core Rules

Schemas are public. Message contents are governed.

Every polis that speaks binary ACIP must expose a public schema catalog. Any
citizen, reviewer, runtime, or authorized tool that may inspect a binary ACIP
message must be able to locate the public schema needed to decode it and render
deterministic JSON.

Schema access does not grant message-content access. Message contents remain
subject to sender/recipient identity, standing, visibility, redaction, payload
classification, lifecycle state, authority basis, consent/disclosure policy,
and trace/audit requirements.

## Acceptance Criteria

- A tracked ACIP `.proto` schema exists for the first binary envelope and
  payload set.
- Binary envelopes carry schema family, schema version, compatibility profile,
  payload type, payload digest, session id, transport type, provider id, and
  monotonic sequence.
- Public schema catalog rules define current, deprecated, accepted-read-only,
  and rejected versions.
- Authorized readers can render binary ACIP messages into deterministic JSON
  using only tracked public schemas.
- Unauthorized readers cannot use schema access to inspect protected content.
- JSON/protobuf round-trip tests prove semantic equivalence for in-scope
  message, invocation, policy-block, and trace-reference fields.
- Malformed bytes, unknown schema versions, missing schemas, mismatched payload
  types, duplicate events, missing sequence numbers, and out-of-order events
  fail closed.
- A mock or loopback WebSocket proof carries binary ACIP events without a live
  provider dependency.
- The proof records trace/replay-compatible session event evidence without
  claiming v0.94 signed/queryable trace completion.

## Demo Candidate

Run a boring binary ACIP session:

1. Open a mock WebSocket session.
2. Send one binary ACIP message event.
3. Send one binary invocation-event proposal.
4. Deny one unauthorized message-content inspection request.
5. Render an authorized event to deterministic JSON through the public schema
   catalog.
6. Close the session.
7. Produce the session trace/replay packet.

What it proves:

- ACIP can use binary protobuf for the wire format.
- Public schema access preserves inspectability.
- Message-content access remains governed.
- WebSocket is a carrier, not an authority source.

What it does not prove:

- Production network security.
- Cross-polis federation.
- Browser realtime support.
- Signed/queryable trace completion.
- Provider-side state trust.

## Downstream Handoff

- `v0.93` should own ACIP transport security, internal encryption, key custody,
  signing, rotation, revocation, and zero-trust message acceptance.
- `v0.94` should own signed/queryable trace and replay closure for
  WebSocket-carried ACIP events.
- `v0.95` can consume the stable carrier as part of MVP provider/operator
  hardening only after the prior security and trace gates are satisfied.

