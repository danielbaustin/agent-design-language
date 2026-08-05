# Structured Planning Prompt

Template: 1.0.0

Issue: 5845

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Run ten bounded episode waves budgeted at 8 hours and 70,000 tokens each (80 agent-hours and 700,000 tokens aggregate), covering source/script work, audio production, transcript/show notes/artwork/metadata, listen review, revisions, validation, feed-wide consistency, and platform playback; five parallel owners target 20-30 hours wall-clock and stop before deployment or publication.

## Plan

Revision 13

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5819, #3223/#3256, route/storage authority, and budget 10 x 2-hour/24,000-token source-and-script waves",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Budget 10 x 2.5-hour/18,000-token production waves for generation, editing, and mastered audio",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Budget 10 x 1.5-hour/14,000-token packaging waves for transcripts, notes, artwork, metadata, manifests, and RSS enclosures",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Budget 10 x 1.5-hour/10,000-token listen-review and revision waves, including consent and redaction review",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Implement the three owned playback producers and emit source-bound receipts for macOS, Linux, desktop Chromium, and physical-device iOS Safari",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Run digest-recomputing receipt validation, resolve feed-wide consistency and exact-head editorial/audio review, and keep publication gated",
    "acceptance_ids": [
      "AC-3",
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
  "elapsed_seconds": 288000,
  "total_tokens": 700000,
  "validation_seconds": 21600
}

## Design

.csdlc/prepared/issues/5845/design.md

Digest: c1be7df6e178d82e31cb10783e47ac9e7096db34f2aff79ea4e068408cf92553

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
