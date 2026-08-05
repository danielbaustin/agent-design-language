# Structured Planning Prompt

Template: 1.0.0

Issue: 5845

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify #5819, #3223/#3256, and route/storage authority; lock episode and guest truth; produce ten complete packages; validate audio, metadata, RSS parity, redaction, consent, and platform playback; resolve exact-head editorial/audio review.

## Plan

Revision 8

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5819, #3223/#3256, route/storage authority, and ten episode briefs",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Produce ten complete scripts, audio masters, transcripts, notes, metadata, artwork, and manifests",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Generate RSS enclosure records and prove episode-to-feed parity without deployment",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run audio, redaction, consent, metadata, missing-asset, and platform checks",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Resolve exact-head editorial/audio review with publication still gated",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Every final audio digest matches its manifest and enclosure
- Guest state never exceeds consent evidence
- Feed records contain stable public-safe values, not local paths
- Temporary validation state remains inside issue evidence
- Publication status remains unclaimed

## Risks

- Audio generation or mastering may vary by provider/platform
- Route/storage authority may remain unresolved
- Guest and artwork rights may block individual episodes
- Large binary artifacts may exceed repository policy

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5845/design.md

Digest: 48be0a274ea1dc4ae4cef401ff486919f975323374808ac3e440564583822e71

## Diagram

.csdlc/prepared/issues/5845/diagram.mmd

Digest: f93e767ad01814319f03d67e71121eeca0bbbaf725dbabb2e0997aaf7f37ccce

## Stop Conditions

- Route/storage or binary-artifact policy is unresolved
- Required consent or source rights are unavailable
- Audio QA cannot produce deterministic manifests
- A credential would need to enter tracked evidence

## Handoff

Proceed only after doctor readiness.
