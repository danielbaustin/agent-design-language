# Structured Planning Prompt

Template: 1.0.0

Issue: 4760

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Seed typed v2 preparation artifacts and leave execution to a later bound session.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inspect existing #4760 task context and preserve the single-concern boundary.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Generate minimal typed v2 cards, design, and diagram.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused csdlc-doctor and report prep-only handoff.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  }
]

## Invariants

- Preparation does not implement or publish #4760.
- Later execution must not close with planning-only claims.
- v0.92 birthday claims remain proof-bound.

## Risks

- Legacy issue version labels may differ from the v0.91.8 preparation wave.
- Later execution may need operator approval for blocker disposition.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/4760/retained/design.md

Digest: 1c0974d04443fd8c73d72e6faf75bc9a3869ef45024f807cd7e8db982f42924a

## Diagram

.csdlc/issues/4760/retained/diagram.mmd

Digest: 6824392dbb71e4ec147551d199e821bbebfaf8f84602d9590b666e1856e0b907

## Stop Conditions

- A live claim collision appears.
- The focused doctor fails on v2 state integrity.
- The task requires implementation or GitHub mutation.

## Handoff

Proceed only after doctor readiness.
