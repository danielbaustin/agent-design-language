# Structured Planning Prompt

Template: 1.0.0

Issue: 5590

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare and exact-review the full Parity-D contract, wait only for #5591 integration eligibility, amend the typed claim to collision-free Runtime v3 paths, implement one secure configured API and Observatory plus guardian/Vector/rollback proof, run every positive and negative lane at one revision, then publish only green truth.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Create and typed-validate all six cards, source-grounded design, diagram, security matrix, preparation validator, and issue-local disjoint claim",
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
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "After #5591 integration eligibility, synchronize the accepted contracts, check active claim collisions, and amend only the smallest Runtime v3 product paths",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement one configuration-driven HTTPS Axum router, actual-listener discovery, and identical local/remote authority using existing COTS crates",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Implement authenticated bounded HTTP and WebSocket Observatory consumption against live admitted-agent and Runtime state with fail-closed origin/session/frame handling",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Complete external guardian launch/reaping/restart/pressure recovery, Vector routing/degradation/redaction, and an executable candidate-to-prior Runtime v3 selector transition with authenticated HTTPS health before and after",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Run exact-revision positive and negative tests, strict lint, dependency and budget reports, bounded soak, exact review, remediation, and green publication",
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

- one independent canonical Runtime v3 kernel and one configuration-driven secure API
- TLS and explicit authentication are mandatory for local and remote Runtime access
- network location never grants authority and discovery reports actual listener truth
- external guardian ownership does not create a sidecar or second Runtime control plane
- Vector owns telemetry export while kernel liveness remains independent
- pressure stop serializes accepted work before terminal exit or eligible restart
- retained evidence is exact-revision, bounded, deterministic, relative, and redacted
- Runtime v2 code and AWS execution remain untouched

## Risks

- local convenience could weaken TLS or authentication
- WebSocket upgrade could bypass HTTP capability, origin, frame, or redaction policy
- discovery could report port 20997 instead of an actual non-default listener
- guardian restart policy could loop on invalid configuration or restart intentional shutdown
- collector failure could be mistaken for kernel failure or custom telemetry code could regrow
- rollback evidence could overclaim Runtime v2 modification or automatic cutover
- Parity-D claim expansion could collide with active Parity-B or Parity-C paths

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/issues/5590/retained/design.md

Digest: 7a800211ce03930cd0fe590615cc611084bb346b2923539216dd325800dad005

## Diagram

.csdlc/issues/5590/retained/diagram.mmd

Digest: ef1c6d3a06558528c0037c41274be3d93a43a3dd17e712d8198587b2367c1f6c

## Stop Conditions

- #5591 integration eligibility is not confirmed for Runtime product edits
- typed claim amendment reports protected-path collision or includes Runtime v2, cloud, or unrelated paths
- the accepted #5336 architecture contradicts this design and requires typed replanning
- any required HTTPS, authentication, WebSocket, discovery, guardian, telemetry, rollback, budget, or negative lane is deferred, skipped, degraded, fixture-only, stale, or failing
- completion would require HTTP Runtime access, hard-coded IP, sidecar, AWS operation, or unsupported deployment claim

## Handoff

Proceed only after doctor readiness.
