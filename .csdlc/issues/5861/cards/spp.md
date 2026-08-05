# Structured Planning Prompt

Template: 1.0.0

Issue: 5861

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Characterize the coupled current contracts, add claim-free generation and sealing primitives, implement linearized recoverable binding and release, migrate legacy records, update typed operator routes, and prove each crash, race, drift, and batch boundary before deleting old coupling.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Characterize current init, edit, doctor, bind, claim, and migration behavior with focused regression fixtures",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-6",
      "AC-10"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement immutable prepared generations, semantic receipts, edit demotion, and doctor routing",
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
    "action": "Implement repository-linearized binding intent, idempotent recovery, release, and compensation",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-10"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Implement truthful batch preparation and audited legacy migration and repair",
    "acceptance_ids": [
      "AC-9",
      "AC-10"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Update typed commands, operator skills, schemas, and architecture docs; prove parity and delete coupled legacy behavior",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10",
      "AC-11",
      "AC-12"
    ],
    "status": "pending"
  }
]

## Invariants

- No preparation operation creates or reserves an execution claim
- No readiness status exists without a current semantic receipt
- No losing overlapping bind mutates Git
- No compensation deletes an artifact not proven to be intent-owned
- No operator must copy or reconstruct a hidden claim identifier
- Every durable transition has one explicit linearization point and recovery action

## Risks

- Compatibility migration can strand or over-release ambiguous legacy claims
- Incorrect digest scope can make receipts either fragile or unsound
- Lock ordering mistakes can deadlock bind and release
- Git compensation can delete pre-existing operator work without exact provenance
- Batch summaries can overstate readiness after partial success
- A broad rewrite can destabilize terminal lifecycle behavior outside this issue

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5861/design.md

Digest: a206164ee7ec641ddbf61895973c31b1e2534c8f35ad3a798945f7a15411bc9f

## Diagram

.csdlc/prepared/issues/5861/diagram.mmd

Digest: 0a59a5969a03e84aa7e87f99da68355a036661b3f9cb88b7b7fd4259503b05d3

## Stop Conditions

- The implementation requires weakening issue-bound worktree or overlap safety
- A migration path cannot distinguish valid active claims from preparation-only claims
- A crash window lacks a deterministic owner recovery or safe operator repair action
- Scope expands into implementation, review, publication, or closeout orchestration
- Focused parity cannot prove the replacement before legacy deletion

## Handoff

Proceed only after doctor readiness.
