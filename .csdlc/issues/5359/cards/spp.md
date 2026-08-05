# Structured Planning Prompt

Template: 1.0.0

Issue: 5359

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare now; execute only after #5355 is live-merged and ancestral.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Verify live WP-21 and WP-21A merged closure and ancestry, then inventory the complete existing v0.92 package and TBD source set.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Revise the WBS, sprint plan, issue-wave YAML, and WP-22 review packet with the full schedule, explicit deferrals, later backlog, non-claims, and WP-23 disposition.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused structural, cross-file, source-disposition, lifecycle, and diff-hygiene validation and retain exact evidence.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Obtain one bounded exact-revision pre-PR review, fix every actionable finding, and record review truth.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Publish the reviewed planning-only package with Closes #5359 and hand WP-23 the exact post-merge eligibility boundary.",
    "acceptance_ids": [
      "AC-5",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- live merge plus ancestry is the dependency gate
- receipts audit-only
- no preparation review churn
- no implementation in preparation

## Risks

- review may approve stale assumptions
- v0.92 opening may be implied without evidence
- closeout planning gaps may be hidden

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5359/design.md

Digest: 7c5fc27776f34c353580c9d3682023c3ccc8ad5869731f5c171c07048cbdc85a

## Diagram

.csdlc/prepared/issues/5359/diagram.mmd

Digest: b5f44e22483066f54e4178775ba56b6b2db756a89d50631ff58c85c41bf1bed4

## Stop Conditions

- #5355 not live-merged
- #5355 merge not ancestral
- review input missing
- unsupported v0.92 claim found

## Handoff

Proceed only after doctor readiness.
