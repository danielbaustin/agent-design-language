# Structured Planning Prompt

Template: 1.0.0

Issue: 5844

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify dependencies; build ten bounded source packets; author all ten complete articles; validate claims, citations, links, privacy, and historical/current posture; review the series arc; record a stop-before-publish disposition.

## Plan

Revision 8

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5819 and establish ten bounded source packets",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Author all ten complete canonical articles",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run per-article claim, citation, link, privacy, and history/current review",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Review the ten-article series arc and resolve duplication or terminology drift",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Reconcile #5843-dependent truth and record stop-before-publish disposition",
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

Digest: caf97377e1ac67efb5654eed761b04b5b433e194b37af252d10d0a0f5e500264

## Diagram

.csdlc/prepared/issues/5844/diagram.mmd

Digest: fd443e63c054166e3e4c71db483681f52038f49cafe19a5a5353a8cbb24673a1

## Stop Conditions

- #5819 naming/link truth is unresolved
- A material claim lacks support
- Privacy or citation review cannot be completed

## Handoff

Proceed only after doctor readiness.
