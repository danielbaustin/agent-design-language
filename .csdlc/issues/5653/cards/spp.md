# Structured Planning Prompt

Template: 1.0.0

Issue: 5653

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Refresh the root README from current v0.91.8 evidence, add the homepage link, prove links and wording, review the exact head, and publish the documentation fix.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Update README status, homepage, and badge context from current evidence",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run focused Markdown/link proof, exact review, and publish",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- README claims remain evidence-bound
- homepage link is explicit
- CI badge target remains main
- no release approval is inferred

## Risks

- stale milestone wording
- broken link
- overclaiming release status

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5653/retained/design.md

Digest: 841a852b8267d543686c4241d13eb74bbb2d0678cf6ba1d537bfeb80fa7af0f5

## Diagram

.csdlc/issues/5653/retained/diagram.mmd

Digest: f72588af59f3216fbabd5f8e947778c5f5aad46390bec0f8d043795b8e408269

## Stop Conditions

- source release truth is ambiguous
- homepage link cannot be verified
- focused proof fails

## Handoff

Proceed only after doctor readiness.
