# Structured Planning Prompt

Template: 1.0.0

Issue: 5829

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Implement and prove WP-12 canonical capability envelopes with explicit providers, tools, skills, grants, denials, limits, provenance, and secret-safe failure handling.

## Plan

Revision 8

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5825, #5826, and #4761 evidence and inspect exact provider/profile surfaces before narrowing paths.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the versioned envelope, canonical ordering, explicit grants/denials/limits, validator, and fixtures.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused canonicalization, stale/escalation/limit negatives, secret scanning, and path portability lanes.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve one bounded exact-head review and publish only with correct base and Closes #5829 linkage.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Capability description never grants authority or proves invocation.
- Canonical ordering makes equivalent envelopes deterministic.
- Credential material, private state, and host-specific paths never enter retained evidence.

## Risks

- Stale #4761 claims could be copied as current capability.
- Envelope parsing could treat missing limits as unlimited.
- Provider metadata or fixtures could leak secrets or machine-local paths.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5829/design.md

Digest: a4a7cfc5283f886defec6b25dc9b3e3fed519d27eef59b892c249efc1e4e7688

## Diagram

.csdlc/prepared/issues/5829/diagram.mmd

Digest: 78acf0d1432d60e5c4c8b3eba8f887a035f2f55abeee4cc19537b6495dc5de3a

## Stop Conditions

- #5825, #5826, or #4761 evidence is not verifiable.
- A provider adapter change exceeds the narrow envelope boundary.
- Secret-safe or authority-escalation negatives cannot be proven.

## Handoff

Proceed only after doctor readiness.
