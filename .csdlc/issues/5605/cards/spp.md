# Structured Planning Prompt

Template: 1.0.0

Issue: 5605

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Inventory prior podcast evidence, author launch-readiness and topic-planning docs, update v0.91.8 indexes, validate, review, and publish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inventory old podcast evidence and current non-claims",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Author launch-readiness, hosting, studio workflow, and topic-slate docs",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Link v0.91.8 indexes and record proof boundaries",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused validation, bounded review, publish, and close out truthfully",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Historical proof stays labeled historical
- No secrets, absolute host paths, or provider credentials in planning artifacts
- Future launch gates remain explicit and fail closed

## Risks

- Overclaiming a live public podcast before the website route and feed exist
- Treating historical demo packets as current production proof

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5605/retained/design.md

Digest: e3fcb8332789c997907ab91e0c713aebb1c788a5c88fc6b1c284cc058b315edd

## Diagram

.csdlc/issues/5605/retained/diagram.mmd

Digest: 162130e33cd1ee61d03cf0197917489f7419573d0d48c89027df500d765ec173

## Stop Conditions

- Any request to publish live web/audio/RSS surfaces inside #5605
- Any need for provider credentials, AWS, or external guest outreach

## Handoff

Proceed only after doctor readiness.
