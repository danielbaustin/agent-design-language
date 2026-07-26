# Structured Planning Prompt

Template: 1.0.0

Issue: 5675

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add two typed hosted routes, prove their request/error contracts with focused Rust tests, run live adapter probes, obtain exact-head review, and publish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Implement Kimi and MiniMax dispatch, endpoints, bounded payloads, and error classification.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Run focused tests, live probes, and exact-head review; repair findings.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Publish the reviewed exact head and verify PR state.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- main remains untouched
- provider credentials never enter records or logs
- billing failures do not retry indefinitely
- MiniMax budgets remain bounded

## Risks

- provider account balance blocks live success
- vendor error envelope drift
- deprecated endpoint compatibility

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5675/design.md

Digest: ae491c4e197cfed06f0d51a0ff65dbcbb1fe0dd458a805c29c0c78659b4dd39f

## Diagram

.csdlc/prepared/issues/5675/diagram.mmd

Digest: afdac2a70b2c02bb1a44bfeed6133f8b314510c373244c63507ed0490472db4d

## Stop Conditions

- credential is missing
- provider endpoint contract cannot be verified
- focused validation fails outside the bounded provider surface

## Handoff

Proceed only after doctor readiness.
