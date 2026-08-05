# Structured Planning Prompt

Template: 1.0.0

Issue: 5852

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify WP-29 and all terminal gates, assemble evidence-linked final release artifacts, dry-run and review the ceremony, then at the exact merged commit create/push tag, publish/verify release, recover partial state idempotently, and close typed issue/sprint/milestone truth.

## Plan

Revision 6

## Steps

[
  {
    "id": "S1",
    "action": "Verify WP-29 pass, all terminal/claim/check/head/tag/release entry gates, and pin the ceremony candidate SHA.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Assemble and validate the evidence manifest, final notes/plan/checklist/handoff, assets, hashes, risks, and non-claims.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused ceremony script tests and dry-run; resolve exact-head review before merge.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "At the exact merge commit create/push the annotated tag, create/publish the release, and identity-check every retry.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Verify live tag/release/assets, complete typed issue/sprint/milestone closeout, and accept the v0.93 handoff without activation.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- No tracked work on main
- No scope absorption across work packages
- Evidence claims remain exact-revision and source-grounded

## Risks

- Dependency drift
- Scope overlap
- Insufficient real-behavior proof

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5852/design.md

Digest: 7d7fd77cd7abd13264c637b3af8ce76a6f4eeb23a64b2323d14e2efd444aa755

## Diagram

.csdlc/prepared/issues/5852/diagram.mmd

Digest: 6457d352e4b029f960adb90912d556e38a0a16ac88e9c3712435b72c47bcc647

## Stop Conditions

- Protected-path collision
- Contradictory dependency evidence
- Required proof cannot be produced within issue scope

## Handoff

Proceed only after doctor readiness.
