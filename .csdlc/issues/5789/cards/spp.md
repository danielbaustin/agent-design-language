# Structured Planning Prompt

Template: 1.0.0

Issue: 5789

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Reproduce the broken default and explicit Observatory paths, repair live Runtime v3 selection and state rendering, fix or truthfully classify WebSocket streaming, implement governed operator-to-agent communication, validate controls/links/export/events/process liveness in browser and CLI, review, publish, and shepherd.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Capture baseline browser/API failures for default route, explicit v3 route, WebSocket, controls, links, and operator messaging.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Patch Observatory live Runtime v3 selection, feed rendering, mode synchronization, link/export/events behavior, and retained evidence labels.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Patch the governed operator-to-agent communication path and UI controls, including fail-closed auth and stale-agent diagnostics.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Add focused CLI/browser validation for live/default/explicit/offline/message/control behavior and process liveness.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run focused proof, exact-head review, fix findings, publish PR, and shepherd to green.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Live Runtime v3 feed truth beats retained fallback when available.
- Operator messaging never bypasses runtime policy/auth gates.
- Machine-readable process and endpoint evidence must match UI claims.
- No AWS operation or live CloudWatch claim is introduced.
- Every user-visible control is either working or truthfully disabled.

## Risks

- WebSocket 1006 may be a backend protocol issue rather than front-end wiring.
- Existing operator write API may be incomplete or only partially wired.
- Browser tests may need to tolerate self-signed HTTPS while still proving real endpoints.
- Retained v0.91.7 evidence paths can mask current Runtime v3 failures if fallback is too eager.

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5789/design.md

Digest: 3d0324231826480c8d72f2a7c03787b7c4746e123f83f559eadc67d21ffb4a91

## Diagram

.csdlc/prepared/issues/5789/diagram.mmd

Digest: fef525498931bb2ad265fd0c079f0b9f62147abba59a5871f3086148a18bcb87

## Stop Conditions

- The Runtime v3 service is unavailable and cannot be started without mutating unrelated state.
- Operator-to-agent messaging requires a new authority model beyond #5789 scope.
- A required control cannot be made truthful without broad runtime redesign.
- Focused proof cannot distinguish live feed truth from retained fallback.

## Handoff

Proceed only after doctor readiness.
