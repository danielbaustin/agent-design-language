# Structured Planning Prompt

Template: 1.0.0

Issue: 5632

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Replace the skill prompt, add source-grounded design artifacts, install into a temporary and operator skill directory, then validate parity and prohibited-command absence.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Update canonical skill and design artifacts",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Install and validate the generated skill",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Review the exact revision before publication",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  }
]

## Invariants

- canonical source and installed copy remain byte-identical
- v2 is sole operational authority
- review truth precedes publication

## Risks

- other legacy skills may still contain stale v1 guidance and require a separate bounded migration issue

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5632/retained/design.md

Digest: fed7a7b36183bd305af4e66d0f093808967dea7082f5b872cf7969ee3c9cbc1c

## Diagram

.csdlc/issues/5632/retained/diagram.mmd

Digest: 91cddb3902e14c12f23b125a2fbb01e2d2077e04713733e004772750894cba82

## Stop Conditions

- stop on missing v2 binary or stale claim
- stop before merge without explicit authorization

## Handoff

Proceed only after doctor readiness.
