# Structured Planning Prompt

Template: 1.0.0

Issue: 5657

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Collapse the local launch path to Guardian -> one Axum/Tokio/Rustls kernel -> one init file and prove it live.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Make production readiness and endpoint/configuration truthful",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Make Observatory routes and authenticated WebSocket proof live",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Prove Guardian shutdown/restart and focused fast launch gate",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Review, publish, merge, and closeout",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Guardian is the sole process-0 owner
- one init file is the endpoint source of truth
- no degraded executor receives production readiness credit
- TLS is mandatory
- private keys never enter continuity identity or tracked evidence

## Risks

- existing adapter implementations may be incomplete
- browser route and API route may currently be separate surfaces
- legacy supervisor assumptions may be embedded in tests
- full workspace coverage may mask the focused launch result

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5657/design.md

Digest: 9822d093a85df8714dbf1f074a01275dbcaf6382c09ac427561e02b4b5161d52

## Diagram

.csdlc/prepared/issues/5657/diagram.mmd

Digest: 8acb6d2ea2e0cb00a3bdd58514f1dd3be99215325c2b90a4a4d991437b41c59c

## Stop Conditions

- required production behavior cannot be proven
- a second supervisor or plaintext credential is required
- protected-path collision or stale claim
- actionable exact-review finding remains open

## Handoff

Proceed only after doctor readiness.
