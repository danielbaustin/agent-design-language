# Structured Planning Prompt

Template: 1.0.0

Issue: 5836

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify #5825-#5830 and #5832-#5834; bind to landed schemas; implement the positive Runtime harness and negative matrix; run replay, redaction, macOS/Linux, and interruption proof; update D1-D6 only from accepted evidence; then review exact head.

## Plan

Revision 8

## Steps

[
  {
    "id": "S1",
    "action": "Verify all WP-18 dependency revisions, schemas, and commands",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the integrated positive runner and deterministic packet validator",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement and retain the complete not-a-birthday matrix",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run replay, interruption, redaction, platform, and D1-D6 artifact checks",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Resolve exact-head review and retain the reviewer index",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Positive proof is Runtime-produced
- Every negative case emits a typed rejection
- Retained artifacts expose no private state or credentials
- D1-D6 status changes only from accepted exact evidence

## Risks

- Dependency schemas may land with different commands
- Demo may accidentally validate a fixture
- Platform or interruption behavior may be hidden

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5836/design.md

Digest: 6bfcd42e50a0009144b66283310fa4f0fa0511050a86d7c746b727fe1a9b2654

## Diagram

.csdlc/prepared/issues/5836/diagram.mmd

Digest: 039549736c17a86e37d74217aa9f5d20dade32c3a9e12e92c43a9975fd8d1a03

## Stop Conditions

- Any required dependency is unlanded
- No integrated Runtime entrypoint can emit the packet
- A required negative cannot be distinguished within issue scope

## Handoff

Proceed only after doctor readiness.
