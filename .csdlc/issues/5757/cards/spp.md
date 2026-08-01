# Structured Planning Prompt

Template: 1.0.0

Issue: 5757

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Patch the Observatory client security/state guards, add real shared TLS/WSS proof, run focused tests, conduct bounded pre-PR review, then publish a ready corrective PR.

## Plan

Revision 1

## Steps

[
  {
    "id": "step-1",
    "action": "Inspect #5757/#5722 source context and relevant product paths only",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "in_progress"
  },
  {
    "id": "step-2",
    "action": "Implement trusted-origin and monotonic generation guards",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "step-3",
    "action": "Implement real shared-certificate/browser-control/authenticated-WSS proof",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "step-4",
    "action": "Run focused UI/runtime/TLS tests and diff hygiene",
    "acceptance_ids": [
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "step-5",
    "action": "Run bounded pre-PR review, fix findings, and publish ready PR",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Bearer token is not attached for untrusted Runtime API origins
- Only the latest operator generation may update live/retained/WSS/fallback state
- HTTPS host and Runtime WSS proof share the same localhost certificate identity

## Risks

- Browser trust behavior may require local certificate setup already present on origin/main
- Async Observatory state changes can regress silently without focused tests
- Proof must avoid fixture-only acceptance

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/evidence/5757/DESIGN.md

Digest: 401af5cfb67e2a7352a888f7ea92651d0e6ccc48c6145a9e592a69ad97548c1f

## Diagram

.csdlc/evidence/5757/diagram.mmd

Digest: 54881b043b9cfd0b58dc407cb2dcbc612a6838f60317b32f800034304ebf37c1

## Stop Conditions

- Typed lifecycle claim collision
- Missing current local TLS support needed for real proof
- Focused proof failure that cannot be fixed within #5757 scope

## Handoff

Proceed only after doctor readiness.
