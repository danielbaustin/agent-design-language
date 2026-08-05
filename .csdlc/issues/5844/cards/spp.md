# Structured Planning Prompt

Template: 1.0.0

Issue: 5844

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Run ten 3.5-hour/68,000-token article waves plus a separately scheduled 5-hour/60,000-token cross-series integration wave inside the fixed 40-agent-hour/740,000-token aggregate; five parallel owners produce complete reviewed articles and stop before publication.

## Plan

Revision 16

## Steps

[
  {
    "id": "S1",
    "action": "Budget 10 x 50-minute/18,000-token source-research waves and establish ten bounded source packets",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Budget 10 x 80-minute/28,000-token drafting waves and author all ten complete canonical articles",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Budget 10 x 40-minute/10,000-token editorial waves for claim, citation, link, privacy, and history/current review",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Budget 10 x 25-minute/8,000-token revision waves and resolve all per-article findings",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Budget 10 x 15-minute/4,000-token validation waves for per-article proof",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Use the allocated 5-hour/60,000-token integration wave for series-arc review, final revisions, #5843 reconciliation, validation, and stop-before-publish disposition",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Define and verify article rollback: restore disposition and series matrix, remove only issue-owned packets, retain source/editorial evidence, and perform no external publication action.",
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

- Each article is grounded in a declared bounded source packet
- Each citation is real, resolvable, and claim-relevant
- Historical evidence is not presented as current delivery truth
- Publication status remains review-ready until separately authorized

## Risks

- Ten articles may duplicate claims or drift in terminology
- Late #5843 truth may invalidate release language
- External links or citations may be unavailable

## Estimates

{
  "elapsed_seconds": 144000,
  "total_tokens": 740000,
  "validation_seconds": 18000
}

## Design

.csdlc/prepared/issues/5844/design.md

Digest: e86b30babe31424f9a42a6b1cfb82ffc572d24a6c503d463f4eecbf6cd977d86

## Diagram

.csdlc/prepared/issues/5844/diagram.mmd

Digest: fd443e63c054166e3e4c71db483681f52038f49cafe19a5a5353a8cbb24673a1

## Stop Conditions

- #5819 naming/link truth is unresolved
- A material claim lacks support
- Privacy or citation review cannot be completed
- Rollback would delete cited upstream evidence or require an external publish, unpublish, or scheduling action.

## Handoff

Proceed only after doctor readiness.
