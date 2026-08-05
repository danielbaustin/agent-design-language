# Structured Planning Prompt

Template: 1.0.0

Issue: 5352

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Initialize truthful final lifecycle state on current main, validate the exact ledger and ancestry, complete one exact pre-PR review, publish, run the full WP-21 sprint review, fix findings, and merge #5352.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Initialize truthful final lifecycle state and exact-revision ledger on current main",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Run focused validation and retain current evidence",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run one exact pre-PR GPT-5.5 review and fix all findings",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Publish, run full WP-21 sprint review, remediate findings, and merge",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- recorded baseline equals origin/main at validation time
- table identity is row-bound rather than token-presence proof
- receipts are audit-only and non-blocking
- review and publication claims remain exact-revision
- no tracked main writes

## Risks

- origin/main may advance and require one explicit baseline refresh before publication
- launch planning may be mistaken for an observed birthday
- metadata-only lifecycle commits must not invalidate substantive review truth

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5352/retained/design.md

Digest: 979d5962c723a24d585de30d4a05ffa704378445110598e9dfe22357deb58844

## Diagram

.csdlc/issues/5352/retained/diagram.mmd

Digest: e73fc95bfbe3027fee5c3881ffeca9bedb5cd23fac306cdec9f2f24970deb06a

## Stop Conditions

- recorded baseline differs from origin/main
- any table row or ancestry binding fails
- focused validation or exact review has an unresolved finding
- publication omits Closes #5352 or includes out-of-scope paths

## Handoff

Proceed only after doctor readiness.
