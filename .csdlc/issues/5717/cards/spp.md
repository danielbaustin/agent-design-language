# Structured Planning Prompt

Template: 1.0.0

Issue: 5717

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Edit the reference studio HTML, add the correct logo asset, update the digest manifest, regenerate served studio output, run focused podcast validation, review, and publish a stacked PR with Closes #5717.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inspect current studio HTML, logo/assets, digest surfaces, and podcast validation path.",
    "acceptance_ids": [
      "AC-1",
      "AC-10"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Apply requested copy, episode, FAQ, contact, footer, logo, and spacing fixes to the reference studio HTML.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Regenerate served studio output and digest files from the reference bundle.",
    "acceptance_ids": [
      "AC-10"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused studio/audio/RSS validation.",
    "acceptance_ids": [
      "AC-10"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Run exact-head subagent review, record lifecycle truth, and publish a stacked PR.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "status": "pending"
  }
]

## Invariants

- The studio route remains under clean podcast-studio.html filenames.
- Reference HTML remains the generator source for served studio output.
- Audio and RSS remain functional and validated.
- Proposed episode topics are not represented as already-recorded episodes.

## Risks

- Editing only generated output could make future generation revert the copy fixes.
- Changing logo paths could break local asset loading.
- Fake episode metadata could make launch status untruthful.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5717/design.md

Digest: 1957aeebaa7648bba86dc24501cc9578d8e3c95fb5f5b80108aaedc5b5511d7a

## Diagram

.csdlc/prepared/issues/5717/diagram.mmd

Digest: f10566b18ec1325d250c0849a44f5c7aca3bb33e54b5781095f1b9770a1fa2b1

## Stop Conditions

- A tracked main checkout edit is detected.
- The correct logo asset cannot be located or copied.
- Audio or RSS validation regresses without a bounded fix.

## Handoff

Proceed only after doctor readiness.
