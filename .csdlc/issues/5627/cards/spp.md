# Structured Planning Prompt

Template: 1.0.0

Issue: 5627

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare and bind minimally; implement deletion-first combined operations; prove atomic failures and measured reduction; exact-review once; publish ready once; merge, validate, and close out.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Initialize six cards, design, and typed claim binding",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement combined validation, review, and direct-ready publication with bounded draft compatibility",
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
    "action": "Run FastWork proof and one exact review; fix every finding",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Publish once, shepherd green CI, merge, post-merge prove, and typed-closeout",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Every write is claim-bound and generation-checked
- Review evidence names the exact scoped revision
- Publication identity is observed rather than fabricated
- Closeout receipt semantics and retained bytes remain unchanged

## Risks

- Combining transitions could write partial state on failure
- Assignment removal could weaken exact revision binding
- Direct-ready publication could accept the wrong PR identity
- Compatibility code could remain broader than active draft records

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5627/design.md

Digest: 2b73dd1a448490348950b976c921d865eb959bc4085375775920376af5faeae5

## Diagram

.csdlc/prepared/issues/5627/diagram.mmd

Digest: 1a27ec1e31399527f10a806742a78e68feff01495275d220f202f4b523242ca8

## Stop Conditions

- Any Runtime or AWS scope
- Any need for a new dependency or broad redesign
- Any protected-path collision not explicitly owned by this fourth-writer authorization
- Any proposal that weakens atomic failure, exact scope, PR identity, or receipts

## Handoff

Proceed only after doctor readiness.
