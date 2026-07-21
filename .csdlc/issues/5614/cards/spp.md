# Structured Planning Prompt

Template: 1.0.0

Issue: 5614

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Remove the source literal, prove runtime redaction, review, publish, merge, and resolve the alert.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Replace the literal with runtime synthetic construction",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run focused redaction proof and review",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Publish, merge, resolve alert, and close out",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- The runtime value still matches the sanitizer's AWS-key rule
- No AWS access

## Risks

- Weakening the redaction test while silencing the scanner

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5614/design.md

Digest: 8a80b4faad8381581898dddb5f0e787ff41598f47c0f6ca729c29d585b76ba3f

## Diagram

.csdlc/prepared/issues/5614/diagram.mmd

Digest: 6874ec2e0a151df008e26e66d627ed2b59c5c5f2d471af517a86f408c3ae5d36

## Stop Conditions

- Any need for live credentials or AWS access

## Handoff

Proceed only after doctor readiness.
