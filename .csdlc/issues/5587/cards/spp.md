# Structured Planning Prompt

Template: 1.0.0

Issue: 5587

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Implement native execution and exact readback, add recursive mirroring, validate locally and live, then review and publish through typed v2.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Implement native execute transport and exact content verification",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement recursive path-preserving Markdown mirroring and automation result semantics",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused and live proof, review, publish, and close out",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- All writes require explicit approval
- Every reported write is content-verified
- Remote content is never deleted

## Risks

- Remote write targets the wrong folder
- Metadata-only verification masks stale content
- Recursive traversal escapes declared roots

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5587/design.md

Digest: 196bd587d9658475dab103e296fb196be3adb7428841932b3863c862741b8eb0

## Diagram

.csdlc/prepared/issues/5587/diagram.mmd

Digest: 0a1ebe4263daff456c834757e1a3324001489f71a08a25651298529cf6ba4874

## Stop Conditions

- Credentials resolve outside approved sources
- Target folder identity is ambiguous
- Readback differs from local bytes

## Handoff

Proceed only after doctor readiness.
