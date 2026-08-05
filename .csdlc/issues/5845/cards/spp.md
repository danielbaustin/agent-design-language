# Structured Planning Prompt

Template: 1.0.0

Issue: 5845

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Run ten 7.5-hour/64,000-token episode waves plus a separately scheduled 5-hour/60,000-token integration wave inside the fixed 80-agent-hour/700,000-token aggregate; five parallel owners produce complete reviewed packages and stop before deployment or publication.

## Plan

Revision 19

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5819, #3223/#3256, route/storage authority, and budget 10 x 1.75-hour/21,000-token source-and-script waves",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Budget 10 x 2.25-hour/16,000-token production waves for generation, editing, and mastered audio",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Budget 10 x 1.5-hour/13,000-token packaging waves for transcripts, notes, artwork, metadata, manifests, and RSS enclosures",
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
    "action": "Budget 10 x 30-minute/4,000-token machine-validation waves and implement the three owned playback producers",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Use the allocated 5-hour/60,000-token integration wave for feed consistency, digest-recomputed platform playback, exact-head review, and final revisions",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Define and verify podcast rollback: restore feed/readiness records, remove only issue-owned packages and producers, retain QA evidence, and perform no deployment or publication action.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-2",
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

Digest: e577c1d95f46d4de635fce4f2afc6d5697cde482e44599380b9b15698a73715f

## Diagram

.csdlc/prepared/issues/5845/diagram.mmd

Digest: f93e767ad01814319f03d67e71121eeca0bbbaf725dbabb2e0997aaf7f37ccce

## Stop Conditions

- Route/storage or binary-artifact policy is unresolved
- Required consent or source rights are unavailable
- Audio QA cannot produce deterministic manifests
- A credential would need to enter tracked evidence
- Rollback would delete upstream production proof, mutate consent, or require feed deployment/publication.

## Handoff

Proceed only after doctor readiness.
