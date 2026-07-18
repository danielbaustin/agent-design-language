# Structured Planning Prompt

Template: 1.0.0

Issue: 5464

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Pin nextest steps to an installer manifest that supports 0.9.140, disable fallback, strengthen static contracts, and inspect hosted proof.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Verify official release asset and supported installer manifest",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Update nextest installer pins and enforce fallback none",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run static proof and inspect hosted CI output",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- Every third-party action remains pinned to a 40-hex commit
- Checksum verification remains enabled
- No AWS execution occurs
- Test-lane behavior is otherwise unchanged

## Risks

- A partial update could leave one lane on the stale manifest
- A future nextest version could be selected before the pinned manifest supports it

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5464/design.md

Digest: 7b4911b3d2706584d0f30221dc602dd7e4928f6749d0945ab70fb700dcf3d503

## Diagram

.csdlc/prepared/issues/5464/diagram.mmd

Digest: 65c1ffffce9f6c0f25abce99a100d1909fcf88d5598b56de08a69194a422f259

## Stop Conditions

- The selected official manifest does not contain nextest 0.9.140
- The selected action revision requires unrelated workflow changes
- Hosted proof requires AWS execution

## Handoff

Proceed only after doctor readiness.
