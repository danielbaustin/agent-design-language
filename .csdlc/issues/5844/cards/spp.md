# Structured Planning Prompt

Template: 1.0.0

Issue: 5844

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Run ten bounded article waves budgeted at 4 hours and 74,000 tokens each (40 agent-hours and 740,000 tokens aggregate), with source research, complete drafting, editorial review, revisions, validation, cross-series review, #5843 reconciliation, and a stop-before-publish disposition; five parallel owners target 12-18 hours wall-clock without reducing any article to an outline.

## Plan

Revision 10

## Steps

[
  {
    "id": "S1",
    "action": "Budget 10 x 60-minute/20,000-token source-research waves and establish ten bounded source packets",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Budget 10 x 90-minute/30,000-token drafting waves and author all ten complete canonical articles",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Budget 10 x 45-minute/12,000-token editorial waves for claim, citation, link, privacy, and history/current review",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Budget 10 x 30-minute/8,000-token revision waves and resolve all per-article findings",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Budget 10 x 15-minute/4,000-token validation waves, review the series arc, and resolve duplication or terminology drift",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Reconcile #5843-dependent truth and record stop-before-publish disposition within the 4-6 hour integration reserve",
    "acceptance_ids": [
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
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5844/design.md

Digest: cc4ee89a1ccf570fb46098e8736c6e31288385e840d9769cbbae761951255b48

## Diagram

.csdlc/prepared/issues/5844/diagram.mmd

Digest: fd443e63c054166e3e4c71db483681f52038f49cafe19a5a5353a8cbb24673a1

## Stop Conditions

- #5819 naming/link truth is unresolved
- A material claim lacks support
- Privacy or citation review cannot be completed

## Handoff

Proceed only after doctor readiness.
