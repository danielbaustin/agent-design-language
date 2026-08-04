# Structured Planning Prompt

Template: 1.0.0

Issue: 5710

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add typed recovery contracts and tests, validate and review the exact implementation, publish a ready PR closing #5710, then run the live v0.91.8 recovery/prune sweep.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Initialize and bind #5710 in its dedicated worktree",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement terminal reconciliation and safe prune-preparation contracts",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement closed-issue lifecycle repair classification",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused tests and exact-head review; fix all actionable findings",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Publish and shepherd a ready PR that closes #5710",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Run the live v0.91.8 closeout/prune recovery sweep and retain the result",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Terminal evidence remains issue-, repository-, PR-, branch-, and revision-bound
- Dirty worktrees fail closed until every path is classified
- Only retained or reproducible generated state may be cleaned automatically
- Existing receipt validation remains required before worktree prune

## Risks

- Over-broad cleanup could delete evidence or user work
- Ancestry alone could accept the wrong PR or branch without full identity binding
- A repair classifier could accidentally become transition authority
- Live worktrees may contain ambiguous tracked record drift requiring operator disposition

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/issues/5710/retained/design.md

Digest: 7cd919400ee26a10b5302f714a2941a80aaac72913d39448284aab941057459c

## Diagram

.csdlc/issues/5710/retained/diagram.mmd

Digest: d75cf48fc65627f3924847220cb87f40bbc0c0848c33ee0e2b85557b103f0122

## Stop Conditions

- Any claim collision on declared closeout source paths
- Any required force deletion or manual lifecycle-state edit
- Any ambiguous terminal identity or unproved evidence retention
- Any failed focused test or actionable exact-head review finding

## Handoff

Proceed only after doctor readiness.
