# Structured Planning Prompt

Template: 1.0.0

Issue: 5702

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Write a source-backed launch plan that turns the existing Podcast Studio proof into a next-week launch path with required audio, required RSS, ten prepared episodes, guest support, Deepgram investigation, site design alignment, validation gates, and Gemini review.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inspect existing podcast studio docs, artifacts, and audio outputs",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Write the reviewable launch plan in `.adl/docs/TBD/`",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Request Gemini review and record suggestions truthfully",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused document validation and prepare for operator review",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- No tracked edits on main
- All claims about launch/audio/RSS/guests remain planned until proven
- The plan must not demote audio or RSS to optional launch work
- The plan must remain consistent with existing source artifacts

## Risks

- Deepgram may not outperform or may add launch risk
- Audio quality proof may take longer than one day
- RSS validation and Apple/Spotify readiness may have external timing constraints
- Ten prepared episodes can drift stale if guest/support metadata is too specific

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5702/design.md

Digest: d7d3096e844b656cf47fc8c34af24a60f1fa5d7ec27160a28d16d56492ba4fed

## Diagram

.csdlc/prepared/issues/5702/diagram.mmd

Digest: 155686d447184504672b14379d2fbcf083694195a6093ca1c629e59d818460e8

## Stop Conditions

- Unable to write in a bound worktree
- Gemini provider unavailable after bounded attempt
- Source evidence contradicts required launch assumptions

## Handoff

Proceed only after doctor readiness.
