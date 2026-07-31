# Structured Planning Prompt

Template: 1.0.0

Issue: 5384

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Render and validate the six typed cards, retain a complete issue-local evidence-consumer design, obtain bounded preparation review, approve the design through typed v2, bind only the three lifecycle paths, and push the preparation branch while all implementation steps remain gated.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Normalize the issue-local cards, design, diagram, and direct-input manifest to the operator-approved schedule.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Verify the four direct input issues are closed and record the exact baseline for later acceptance execution.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Prepare the focused three-product fresh-consumer validation and compact acceptance ledger.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "On separate execution authorization, run focused acceptance, obtain one Gemini review, and publish the WP-14A PR.",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- no tracked root-main writes
- no product or shared-document paths in the preparation claim
- no implementation until every predecessor gate fact passes at one refreshed origin/main revision
- no manual edits to rendered cards or lifecycle state
- no fake approvals, waivers, inferred terminal state, or prose-only evidence
- no PR, publication, AWS, Runtime v2, or raw gh

## Risks

- stale issue prose or a closed GitHub issue can be mistaken for typed terminal truth
- a broad preparation claim could accidentally authorize product work
- nested Runtime and workcell inputs can be omitted from the direct WP-14A child list
- planning estimates can be misreported as execution evidence
- a predecessor closed without merged ancestry cannot satisfy the operator's strict promotion rule

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/issues/5384/retained/design.md

Digest: d8d4bdae34791af754e1c9ba389c5181b7063a8b53fbce53af792354854ff4d8

## Diagram

.csdlc/issues/5384/retained/diagram.mmd

Digest: 58a47f484f346d190bc63ac9956e572f524e10e7856c9615fb676767621676a1

## Stop Conditions

- any request requires a path outside the three protected preparation paths
- any declared predecessor lacks merged, typed closed_out, receipt, or ancestry proof
- current-template identity or structure validation fails
- bounded review reports an actionable finding that cannot be fixed within preparation scope
- another claim, branch, or worktree collides with #5384

## Handoff

Proceed only after doctor readiness.
