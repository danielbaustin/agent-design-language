# Structured Planning Prompt

Template: 1.0.0

Issue: 5804

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Repair six bounded documentation surfaces, validate the complete corpus, perform one pre-PR review, and publish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Repair manifest, live issue truth, and portable commands",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run full corpus validation and bounded review",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Publish a docs-only PR that closes #5804 and leaves WP-19 open",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- Dated historical evidence remains historical
- External review performed remains false
- Release approval and v0.92 activation remain false
- WP-19 remains open

## Risks

- A current-truth correction could overwrite a dated historical snapshot
- A manifest entry could point to an absent path
- The handoff could accidentally overclaim review completion

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5804/design.md

Digest: cc06d4e9414753067ce9d9724241fdd7519ec660adfb2f24ca0a9f23dd2d95f9

## Diagram

.csdlc/prepared/issues/5804/diagram.mmd

Digest: 744ab57f8fb5f7d6633cdd822426c1f372f17260d13e8d0906483a09940256f4

## Stop Conditions

- A required implementation or proof surface is absent
- Live issue truth cannot be verified
- A product-code change becomes necessary

## Handoff

Proceed only after doctor readiness.
