# Structured Planning Prompt

Template: 1.0.0

Issue: 5762

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Bind #5762, repair the store.rs terminal SOR validation fixture authority setup, validate focused and full C-SDLC v2 lanes, run bounded review, publish a ready PR, and shepherd it green.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Initialize and bind #5762 in the issue worktree.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Repair the terminal SOR validation fixture authority setup.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused tests, full locked all-target C-SDLC v2 tests, strict Clippy, and diff hygiene.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run bounded gpt-5.5 review, fix findings, publish ready PR, and shepherd checks green.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Production terminal repair semantics remain unchanged.
- Temporary test authority is deterministic and issue-local.
- Closed_out targets remain claim-free.

## Risks

- Fixture copied from live tracked records can drift as issues close out.
- A broad repair could accidentally weaken production active-claim authorization.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/issues/5762/retained/design.md

Digest: ac6d85e2f2ec631133731b7d4f07d0066fc83d19f6b89a7f041cefe20ac3e246

## Diagram

.csdlc/issues/5762/retained/diagram.mmd

Digest: 88fbc0f5ba880e12b0902ae4574bf587f85c5a26d15d3438c842f36e2d1201c1

## Stop Conditions

- typed lifecycle claim collision
- focused terminal SOR validation tests fail after fixture repair
- full locked all-target C-SDLC v2 tests fail with an in-scope regression
- bounded review finds actionable issues

## Handoff

Proceed only after doctor readiness.
