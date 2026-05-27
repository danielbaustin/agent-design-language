# ACIP Binary Schema And WebSocket Transport

## Metadata

- Feature Name: ACIP Binary Schema And WebSocket Transport
- Milestone Target: `v0.92`
- Status: planned
- Source Docs:
  - `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`
  - `docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml`
  - `docs/explainers/ACIP.md`
  - `docs/milestones/v0.91.1/features/ACIP_HARDENING.md`
  - `#3377`
- Proof Modes: schema, fixtures, round-trip tests, mock transport proof

The earlier local-only ACIP schema-catalog and WebSocket notes are provenance
inputs, not canonical public source paths. WP-01 should either promote their
remaining requirements into tracked v0.92 docs or record them as explicit gaps.

## Template Rules

This is a planning feature doc. It records the transport-readiness scope for
v0.92 without claiming production networking, security, or signed trace.

## Purpose

Promote ACIP from a local structured JSON/serde communication substrate into a
binary, schema-governed session transport surface without losing
inspectability, citizen access boundaries, or ADL authority semantics.

The first real network/session ACIP carrier should use protobuf bytes on the
wire. JSON remains the deterministic projection and fixture format used for
debugging, review, trace, and citizen-facing inspection when access is allowed.

## Context

ACIP already exists as an ADL communication substrate. v0.92 should define the
binary/protobuf and schema-catalog shape needed for future session transport
while preserving inspectability.

## Coverage / Ownership

v0.92 owns schema, catalog, JSON projection, fixtures, and mock/loopback
carrier proof. v0.93 owns transport security and key lifecycle. v0.94 owns
signed/queryable trace completion.

## Overview

Binary ACIP should be efficient on the wire and still reviewable through
public schemas and deterministic JSON projections for authorized readers.

## Design

The design uses protobuf envelopes with schema family/version metadata,
payload classification, digest, session identity, monotonic sequence, and
public catalog lookup.

## Execution Flow

1. Select schema and compatibility profile.
2. Encode binary ACIP envelope.
3. Decode through public schema catalog.
4. Render deterministic JSON for authorized readers.
5. Reject unauthorized content inspection.

## Determinism and Constraints

Round trips must preserve semantic fields. Schema access must not bypass
message-content authorization. Unknown, missing, malformed, duplicate, or
out-of-order events fail closed.

## Integration Points

- `docs/explainers/ACIP.md`
- v0.91.1 ACIP hardening.
- v0.92 birthday evidence packet.
- v0.93 transport security.
- v0.94 signed/queryable trace.

## Validation

Validation should include protobuf/JSON round trips, schema-catalog lookup,
denied-access cases, malformed payloads, sequence checks, and mock WebSocket
carrier proof.

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
- Optional server-side OpenAI Realtime WebSocket adapter spike only after the
  mock/loopback proof is stable.
- Fail-closed behavior for missing, unknown, unavailable, private, deprecated,
  or mismatched schemas.

Out of scope:

- Production cross-polis networking.
- Live provider WebSocket dependency as the first proof.
- Browser WebRTC.
- Browser/mobile realtime client implementation.
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

## Risks

- Binary transport could become opaque. Mitigation: require public schemas and
  deterministic JSON projection.
- Schema access could be confused with content access. Mitigation: keep access
  rules separate and fail closed.

## Future Work

v0.93 should harden transport security. v0.94 should close signed/queryable
trace. v0.95 can consume the mature carrier if prior gates pass.

## Notes

WebSocket is a carrier proof, not an authority source.

## Optional OpenAI Realtime Spike

After the mock or loopback WebSocket carrier proof passes, v0.92 may add a
bounded server-side OpenAI Realtime WebSocket spike when credentials and
environment are available.

Scope:

- server-to-server only
- provider JSON events normalized into ADL transport/session events
- no browser WebRTC or mobile-client claim
- no production availability, security, latency, or provider-state-trust claim
- no bypass of ACIP, ACC, Freedom Gate, policy, trace, or replay boundaries

Proof expectation:

- connection opens and closes deterministically enough for a bounded spike
- session update or equivalent provider event can be sent
- provider events are recorded as untrusted inputs before ADL normalization
- failures are classified as provider/auth/network/environment versus ADL
  normalization failures
- trace evidence records session lifecycle and event ordering without depending
  on hidden provider state

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
  signing, rotation, revocation, WebSocket upgrade/session hardening,
  per-message authorization, and zero-trust message acceptance.
- `v0.94` should own signed/queryable trace and replay closure for
  WebSocket-carried ACIP events and normalized provider session events.
- `v0.95` can consume the stable carrier as part of MVP provider/operator
  hardening only after the prior security and trace gates are satisfied.
