# Structured Planning Prompt

Template: 1.0.0

Issue: 5778

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Characterize current closeout behavior; implement a pure derived terminal resolver; add csdlc-finish with exact-head merge and retry convergence; make terminal remote state non-blocking for claims while preserving legacy reads; validate focused Gate 4-7 behavior; obtain exact-head review and publish one ready PR.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Add characterization fixtures and freeze the derived terminal input/output contract",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the pure terminal resolver and legacy compatibility classification",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement csdlc-finish exact-head merge and interruption-safe idempotent convergence",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Make proven terminal remote state non-blocking for claim collision and update operator contracts",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run focused validation, exact-head review, ready publication, and CI shepherding",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Never write tracked files on main
- No AWS or external ledger service
- No weaker review, checks, publication identity, or expected-head merge gate
- No terminal projection or second closeout PR
- No worktree deletion in finish
- Machine-readable JSON on stdout and human observability on stderr

## Risks

- Treating a stale or unrelated GitHub observation as terminal could release a live claim
- Review evidence committed after review could recreate exact-head recursion
- Squash merge ancestry differs from head ancestry and must use GitHub merge identity truthfully
- Compatibility logic could accidentally retain the old receipt as canonical

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5778/design.md

Digest: 6d7f225ea3b7b4406de1107c022687fc6ea299ab6616e7f18073fc71a2cae771

## Diagram

.csdlc/prepared/issues/5778/diagram.mmd

Digest: 615d73f98056b6e54226b5157b76305be7a107e719bba03f0c10a76245f884f8

## Stop Conditions

- Any need to weaken exact-head review or required checks
- Any need for raw GitHub lifecycle mutation
- Any collision with a live nonterminal claim
- Any requirement for AWS, external persistence, or worktree deletion
- Any design that still needs a second closeout PR

## Handoff

Proceed only after doctor readiness.
