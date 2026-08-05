# Structured Planning Prompt

Template: 1.0.0

Issue: 5832

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

After WP-04 lands, inventory existing ACIP/A2A/WSS/trace contracts, freeze one semantic envelope and compatibility policy, implement protobuf/catalog/deterministic JSON parity, harden authenticated bounded WSS admission and reconnect behavior, then run round-trip, denial, replay, malformed, real-carrier, platform, and exact-head proof.

## Plan

Revision 13

## Steps

[
  {
    "id": "S1",
    "action": "Verify WP-04 and trace/stream baselines, inventory semantic drift, and freeze envelope, version, compatibility, schema, catalog, JSON, and carrier contracts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement protobuf/catalog/JSON parity plus authenticated bounded WSS negotiation, admission, dispatch, errors, reconnect, and backpressure.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run golden round trips, compatibility and denial negatives, real bidirectional WSS exchange, and native-platform proof.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve exact-head review and publish stable consumer handoff with closing linkage.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- One semantic message has equivalent protobuf and deterministic JSON meaning
- Unsupported versions and unknown required fields fail without guessing
- Malformed, oversized, replayed, wrong-runtime, or denied frames fail before dispatch
- Authentication and signed command policy remain Runtime-owned
- Queues and binary payloads remain bounded
- Public catalog cannot silently diverge from the wire schema

## Risks

- Existing ACIP/A2A envelopes may encode conflicting semantics
- JSON number/byte/omission rules could drift from protobuf
- Catalog generation could become stale
- Version negotiation could accidentally accept incompatible majors
- WSS auth, reconnect, or backpressure could widen authority or duplicate dispatch

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5832/design.md

Digest: b718ea4c1edd36ad2ac4cacfdcacd680abf06b043e65e4271d86584da3b1ac7a

## Diagram

.csdlc/prepared/issues/5832/diagram.mmd

Digest: 0ddb64863e3e34f2587da73ca88c9ce88eb77000c199807e0c69639234b28b7f

## Stop Conditions

- Issue 5821 architecture/security gate is not terminal
- Issue 5862 WP-04-IMP or any of issues 5863 through 5878 lacks terminal integrated evidence
- Semantic envelope, schema, catalog, JSON, or WSS authority remains ambiguous
- Any declared path overlaps an active owner
- Native macOS, Linux, or Windows proof cannot be produced

## Handoff

Proceed only after doctor readiness.
