# Structured Planning Prompt

Template: 1.0.0

Issue: 5566

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Recognize the exact registered existing worktree as issue-local, preserve bind guards, and prove match and mismatch behavior.

## Plan

Revision 1

## Steps

[
  {
    "id": "implement",
    "action": "Implement existing-worktree bind recognition and regressions",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  }
]

## Invariants

- No hand-edited cards
- No claim takeover
- No readiness bypass
- Atomic issue state

## Risks

- Loose path matching could activate a claim from the wrong checkout.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5566/retained/design.md

Digest: 3f446bdbb3e2b95665a006199a6ab044a892541cbebe355b5ec37f9d32984981

## Diagram

.csdlc/issues/5566/retained/diagram.mmd

Digest: a715bb042f4ba539419d845f967fabfc23ee238f0d98d7493f008c87ada77ec3

## Stop Conditions

- Any need to hand-edit rendered cards
- Any relaxation of exact claim or collision checks

## Handoff

Proceed only after doctor readiness.
