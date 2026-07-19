# Structured Planning Prompt

Template: 1.0.0

Issue: 4647

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Bind WP-20 to its issue worktree, then execute the future work with focused evidence and truthful review/closeout records.

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

.csdlc/prepared/issues/4647/design.md

Digest: a4b6e3f41688d189535c3af5cb3a025068fd14275ceaad57f22e31c18b4cf62d

## Diagram

.csdlc/prepared/issues/4647/diagram.mmd

Digest: af9be2ca7b68d3384a19add34a14fd2b3e105bfc7e4822e9bd810bcc746f381d

## Stop Conditions

- Required dependency remains open or blocked without operator approval
- Execution would require sibling WP remediation
- Release-readiness claim cannot be backed by retained or fresh proof
- AWS or paid remote validation would be required for preparation

## Handoff

Proceed only after doctor readiness.
