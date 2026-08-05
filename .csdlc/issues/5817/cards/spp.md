# Structured Planning Prompt

Template: 1.0.0

Issue: 5817

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Audit prerequisite truth, reconcile canonical v0.92 surfaces, validate the dependency graph, open the issue wave idempotently, generate typed card bundles, and publish after one exact review.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Audit prerequisite and candidate issue-wave truth",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Reconcile canonical version, milestone, feature, ADR, and demo surfaces",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Validate and open the final issue wave with six-card bundles",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run one exact pre-PR review and publish",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Primary main checkout remains clean
- No duplicate issue is created
- No child implementation begins
- Every claim is evidence-bound

## Risks

- Candidate docs may contain stale pre-release assumptions
- Issue-wave size can amplify duplicate or dependency errors
- Historical loop-runtime proof may not satisfy current Runtime v3 contracts
- Version changes may unintentionally churn Cargo.lock

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5817/design.md

Digest: 354299a3bdd065516fd0ab8c91f4a0f9a1aa60e35971d5c8b00897721e31d739

## Diagram

.csdlc/prepared/issues/5817/diagram.mmd

Digest: 5bbfd94b3a5d0d0757ceb1b0d092c1e05feb838b088ef92a1683a267298163c2

## Stop Conditions

- A protected-path collision with an active owner
- A prerequisite cannot be classified from current evidence
- Opening an issue would duplicate a live or completed issue
- The dependency graph is cyclic

## Handoff

Proceed only after doctor readiness.
