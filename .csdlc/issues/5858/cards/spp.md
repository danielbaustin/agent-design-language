# Structured Planning Prompt

Template: 1.0.0

Issue: 5858

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify the full child batch, route safe independent lanes to separate sessions, preserve serial gates, and synthesize one integrated sprint review after child completion.

## Plan

Revision 1

## Steps

[
  {
    "id": "readiness",
    "action": "Validate the Sprint Execution Packet and all child issue cards before handoff",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "coordinate",
    "action": "Route child sessions according to declared lanes and gates",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "review-close",
    "action": "Review integrated results and close the umbrella only after child terminal truth",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Child issues retain all code and proof authority
- No child begins before its declared dependencies
- No umbrella closeout substitutes for child closeout
- Parallel lanes use separate child worktrees and issue-bound goals

## Risks

- A session could mistake coordination authority for child implementation authority
- A parallel lane could start before a serial dependency is complete
- An umbrella could overstate sprint completion while a child remains nonterminal

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5858/design.md

Digest: ee6127bbfc386b040afb32eb48a4a7626123375f24596e87c52996362ca10b59

## Diagram

.csdlc/prepared/issues/5858/diagram.mmd

Digest: cfb0e7343b009dc8a3b847446174a1b99b415270c65ed50642c447ee38764e79

## Stop Conditions

- Any overlapping child write ownership
- Any missing child issue or card bundle
- Any required dependency not represented in the packet

## Handoff

Proceed only after doctor readiness.
