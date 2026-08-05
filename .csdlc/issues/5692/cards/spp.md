# Structured Planning Prompt

Template: 1.0.0

Issue: 5692

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add a small reusable closing-keyword predicate in csdlc-v2 publication validation, replace raw issue mention checks with it, document the policy in AGENTS.md, and prove it with focused gate6 tests.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inspect current publication and GitHub support surfaces",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement policy and verifier guard",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused proof and bounded review",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Publish ready PR with Closes #5692 and shepherd checks",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Primary checkout remains clean on main
- Implementation edits happen only in the #5692 worktree
- Publication must fail closed if the governed PR body lacks the issue closing keyword
- Post-merge typed closeout is truthful but nonblocking

## Risks

- Over-broad body parsing could reject valid GitHub close keywords
- Under-broad parsing could allow non-closing issue mentions

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5692/design.md

Digest: 3cca33ade693a39c39e24c7316370f9c2350a844aebd089abd7e1f481a746260

## Diagram

.csdlc/prepared/issues/5692/diagram.mmd

Digest: 4b70df8eeefd31cfd5c704217c1b1e81ee5d4297bda32805a7f81903f77e6a50

## Stop Conditions

- Protected path collision
- Focused publication verifier tests fail
- Bounded review finds actionable policy/tooling defects

## Handoff

Proceed only after doctor readiness.
