# Structured Planning Prompt

Template: 1.0.0

Issue: 5829

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Implement and prove WP-12 canonical capability envelopes with explicit providers, tools, skills, grants, denials, limits, provenance, and secret-safe failure handling.

## Plan

Revision 15

## Steps

[
  {
    "id": "S1",
    "action": "Reconcile the WP-08 dependency mismatch, verify #5825/#5826/#4761 evidence, and claim the exact capability_envelope.rs, lib.rs, tests, fixture, feature, and evidence paths.",
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
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- Capability description never grants authority or proves invocation.
- Canonical ordering makes equivalent envelopes deterministic.
- Credential material, private state, and host-specific paths never enter retained evidence.

## Risks

- Capability could be mistaken for authority or invocation proof.
- Retained #4761 provenance could be copied without current digest verification.
- A required provider adapter edit could exceed the exact capability-envelope boundary.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5829/design.md

Digest: e0c98b4bbad7618c697213e6e5f76e42394d12fee8330c942d04d164766baacf

## Diagram

.csdlc/prepared/issues/5829/diagram.mmd

Digest: 78acf0d1432d60e5c4c8b3eba8f887a035f2f55abeee4cc19537b6495dc5de3a

## Stop Conditions

- WP-08/#5825, WP-09/#5826, or #4761 evidence is stale or unverifiable.
- The WP-08 dependency mismatch among the sprint gate, canonical wave row, live issue, and cards has not been reconciled.
- Execution requires provider-adapter or credential changes outside the exact declared paths.
- Secret-like data or unbounded authority cannot be excluded.

## Handoff

Proceed only after doctor readiness.
