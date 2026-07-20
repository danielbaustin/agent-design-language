# Structured Planning Prompt

Template: 1.0.0

Issue: 4650

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Bind WP-23 to its issue worktree, then execute the future work with focused evidence and truthful review/closeout records.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Confirm live issue and dependency truth before execution",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Produce the declared deliverables on protected tracked paths",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run focused validation and record fresh/retained/skipped proof truth",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Update SRP/SOR truth and preserve non-claims before publication",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  }
]

## Invariants

- Work stays issue-local
- Findings and claims remain evidence-bound
- No release or activation readiness is inferred from prep
- No AWS use for preparation

## Risks

- Dependencies may not yet be clean enough for execution
- Review/remediation scope can be mistaken for sibling work
- Retained evidence can become stale if live issue truth changes

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/4650/design.md

Digest: fda1682220e3d4185e4331f1b3d50a98dedb2511efafde223471e4b8e68a6ed4

## Diagram

.csdlc/prepared/issues/4650/diagram.mmd

Digest: ca29f06a0b2ee20135d367052ec17467b2a9ffd3fb9342c4d8c66820b28acc8f

## Stop Conditions

- Required dependency remains open or blocked without operator approval
- Execution would require sibling WP remediation
- Release-readiness claim cannot be backed by retained or fresh proof
- AWS or paid remote validation would be required for preparation

## Handoff

Proceed only after doctor readiness.
