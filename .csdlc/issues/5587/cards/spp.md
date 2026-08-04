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

.csdlc/issues/5587/retained/design.md

Digest: ec76c9c8fa6de4828b2408759195fea8ec6f77b178ef815694d96effee17affe

## Diagram

.csdlc/issues/5587/retained/diagram.mmd

Digest: 2157c19e2e59df171450026697330c9cb77042ead664151a33e8cd57abad7744

## Stop Conditions

- Credentials resolve outside approved sources
- Target folder identity is ambiguous
- Readback differs from local bytes

## Handoff

Proceed only after doctor readiness.
