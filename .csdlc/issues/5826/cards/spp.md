# Structured Planning Prompt

Template: 1.0.0

Issue: 5826

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Implement and prove WP-09 canonical identity records with stable labels, immutable root authority, provenance, and redaction-safe negative handling.

## Plan

Revision 17

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5825 terminal proof and inspect adl-runtime-kernel identity_memory.rs and private_state.rs before claiming the exact birthday_identity.rs, lib.rs, tests, fixture, feature, and evidence paths.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the versioned identity record, deterministic derivation, canonical serialization, and valid/negative fixtures.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused replay, alias/provenance negatives, privacy-redaction, and path-portability lanes and retain exact-revision proof.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve one bounded exact-head review and publish only with correct base and Closes #5826 linkage.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Stable name never substitutes for identity root authority.
- Canonical identity derivation and serialization are deterministic.
- Raw private state is unnecessary for review and cannot enter retained projections.

## Risks

- Alias updates could silently replace root identity.
- Continuity references could be accepted without binding prior evidence.
- Fixtures or reports could leak private or host-specific paths.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5826/design.md

Digest: ba2ad2f62ab40d2e0a1b86240ffdac8fe3f07431993fb1f818bc8e36d3e493c1

## Diagram

.csdlc/prepared/issues/5826/diagram.mmd

Digest: d5cc198edc4895057317ca54fe8aa3c676496c00d23ba4718aa88f17ac266f58

## Stop Conditions

- #5825 lacks terminal receipt-backed proof.
- Exact identity paths collide with another live claim.
- Identity requires raw private state or an unversioned shared schema change.

## Handoff

Proceed only after doctor readiness.
