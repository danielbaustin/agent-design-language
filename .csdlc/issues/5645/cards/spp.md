# Structured Planning Prompt

Template: 1.0.0

Issue: 5645

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add the binary and contracts, enforce readiness and exact-head checks, test with a credential-free GitHub fixture, then review.

## Plan

Revision 1

## Steps

[
  {
    "id": "merge-command",
    "action": "Implement typed merge command and tests",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- Exact reviewed head is required
- Canonical merge_ready state is required
- Merge result includes exact SHA
- Secrets never enter output or artifacts

## Risks

- GitHub permission denial
- Remote head drift
- Octocrab API shape changes

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5645/retained/design.md

Digest: 50bd7dfe835d21649f690eac40bb0b23f34c01249e36fcd3f6243aaf112fd3a3

## Diagram

.csdlc/issues/5645/retained/diagram.mmd

Digest: 2ecc58425db1ab78512fbced65a0a3cb65a45a8e04830d721f02e8fec0645a93

## Stop Conditions

- Any readiness mismatch
- Any token or secret leakage
- Any test failure

## Handoff

Proceed only after doctor readiness.
