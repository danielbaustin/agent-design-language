# Structured Planning Prompt

Template: 1.0.0

Issue: 5547

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Initialize #5547, inspect the C-SDLC revision identity implementation and #4645 findings, choose the smallest truthful disposition, then validate either focused code changes or the retained plan/card truth.

## Plan

Revision 1

## Steps

[
  {
    "id": "AC-1",
    "action": "Inspect current C-SDLC review/publication revision identity behavior and decide the contract.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "AC-2",
    "action": "Apply the chosen identity disposition through code/docs or route an exact v0.91.8 residual.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "AC-3",
    "action": "Write the ownership-first split plan for the large modules named by IR-4645-012.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "AC-4",
    "action": "Record deferred scope and validation truth before publication.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "completed"
  }
]

## Invariants

- Tracked implementation happens only in the bound #5547 worktree.
- Revision identity claims must match actual code and retained proof.
- Ownership split planning must preserve behavior-first boundaries and avoid cosmetic-only moves.
- Deferred work must be explicitly routed and not described as complete.

## Risks

- Changing revision identity semantics late in v0.91.7 may affect publication safety.
- Leaving whole-tree identity undocumented may continue to surprise scoped review operators.
- Ownership split planning can become too broad unless it stays issue-bound.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/issues/5547/retained/design.md

Digest: 0206fe7bcd87b35f0d9da1150602e4e992fb42b312a4328012264ea399d6842b

## Diagram

.csdlc/issues/5547/retained/diagram.mmd

Digest: 13f960e77769ab414b3254c4712d2d5436f52911e38563fcf1d43e5e822a2b09

## Stop Conditions

- C-SDLC v2 doctor or bind reports corrupt state that cannot be repaired without manual lifecycle mutation.
- Protected path collision with another active issue blocks the intended #5547 surface.
- Implementation would require broad module refactors outside #5547 non-goals.

## Handoff

Proceed only after doctor readiness.
