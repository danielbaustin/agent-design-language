# Structured Planning Prompt

Template: 1.0.0

Issue: 5715

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Copy the exported studio bundle into a committed reference directory with a clean HTML filename, have the generator copy it to demos/podcast/studio with a small route index, keep the exported HTML bytes unchanged, preserve audio/RSS generation, extend validation, and record proof.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inspect current podcast generator, generated pages, RSS, audio artifact, and the immutable studio export.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Copy the studio export into a clean reference bundle and route bundle without editing the exported HTML content.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Wire the podcast landing page to the studio route.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Preserve and validate audio player and RSS feed wiring.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Update focused validation for route wiring, source/copy digest identity, audio, and RSS.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  }
]

## Invariants

- The exported studio HTML text and referenced images remain unchanged.
- Audio and RSS remain launch blockers.
- Generator-owned output stays reproducible.
- Issue evidence distinguishes local demo wiring from production deployment.

## Risks

- Changing the exported HTML would invalidate the operator-provided design artifact.
- Hand-editing generated output could leave future episodes on stale wiring.
- Visual integration could accidentally regress RSS/audio links.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5715/design.md

Digest: f7d926218fa94dee825b1c0d404fa763ff76238ee2ed4e5cb42dfd16c65aa834

## Diagram

.csdlc/prepared/issues/5715/diagram.mmd

Digest: 60c34fe05b83fe4777a322caad6c5bd978d318b1e7e189c48a544387efc9b681

## Stop Conditions

- A tracked main checkout edit is detected.
- The exported HTML content would need mutation to proceed.
- Audio or RSS validation regresses without a clear bounded fix.

## Handoff

Proceed only after doctor readiness.
