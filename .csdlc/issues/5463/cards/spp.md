# Structured Planning Prompt

Template: 1.0.0

Issue: 5463

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Inventory annotated pins, replace them with reviewed Node 24 commits, strengthen static contracts, and inspect hosted PR annotations.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inventory annotated revisions and verify official Node 24 replacement commits",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Replace pins and strengthen canonical immutable-SHA contracts",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run static proof and inspect hosted PR annotations",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Every third-party action remains pinned to 40-hex commit
- No AWS execution occurs
- Workflow behavior outside action runtime upgrades is unchanged

## Risks

- Major action upgrades may change inputs or outputs
- A partial replacement can leave compatibility forcing in less-used workflows

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5463/retained/design.md

Digest: c7a88a3322b388897fffc0bc336747387e1fdfaebb84b33ca7bad2f93ed5997e

## Diagram

.csdlc/issues/5463/retained/diagram.mmd

Digest: 536eb3739081002b884beb69dc7cae5984544d0cb1217a8fc662726fa9167b71

## Stop Conditions

- Official action metadata does not declare Node 24
- Replacement release has incompatible behavior requiring broader CI design
- Hosted proof requires AWS execution

## Handoff

Proceed only after doctor readiness.
