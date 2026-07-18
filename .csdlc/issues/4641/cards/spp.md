# Structured Planning Prompt

Template: 1.0.0

Issue: 4641

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Bind WP-14 to its issue worktree, then execute the future work with focused evidence and truthful review/closeout records.

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
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Produce the declared deliverables on protected tracked paths",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused validation and record fresh/retained/skipped proof truth",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Update SRP/SOR truth and preserve non-claims before publication",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
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

.csdlc/issues/4641/retained/design.md

Digest: 81201aebef52effe741c3ecf69e531133aec9526caf8823f13699bf10b210d1a

## Diagram

.csdlc/issues/4641/retained/diagram.mmd

Digest: b67777fae192af9512bdd1ebc5b12e541355b120c064c94fd13f7264863c9f3d

## Stop Conditions

- Required dependency remains open or blocked without operator approval
- Execution would require sibling WP remediation
- Release-readiness claim cannot be backed by retained or fresh proof
- AWS or paid remote validation would be required for preparation

## Handoff

Proceed only after doctor readiness.
