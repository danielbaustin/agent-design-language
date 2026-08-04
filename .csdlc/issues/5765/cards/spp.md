# Structured Planning Prompt

Template: 1.0.0

Issue: 5765

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add the smallest source-grounded v0.92 planning reference, validate it, review it, and publish a PR without performing migration work.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Add the bounded migration scheduling reference to the v0.92 issue-wave YAML",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run focused YAML/diff validation and exact-head review before publication",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- Planning-only status remains explicit
- asksifu is never a migration candidate
- Unknown sixth candidate blocks execution
- No tracked edit occurs on main

## Risks

- The TBD source plan is intentionally local and ignored; the YAML reference must remain truthful about that boundary

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5765/design.md

Digest: a9efa5d35de1d00dcd47d31009de46f6db5a009128c1767afe05565d5812d187

## Diagram

.csdlc/prepared/issues/5765/diagram.mmd

Digest: 6dc96fc975be9e1bcaae4933fe273b7cfcf6c42f514f642ce96e7937fa5d0c33

## Stop Conditions

- Any request to transfer or configure GitHub
- Any need to edit files outside the declared scope
- Any work on main

## Handoff

Proceed only after doctor readiness.
