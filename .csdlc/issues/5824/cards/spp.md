# Structured Planning Prompt

Template: 1.0.0

Issue: 5824

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Audit current v2 restricted fields end to end, classify typed completion and extensibility, implement only the smallest proven finite gap, preserve durable strings, and prove schema/editor/validator/Markdown round-trip parity and invalid-value rejection.

## Plan

Revision 11

## Steps

[
  {
    "id": "S1",
    "action": "Inventory every restricted field and declare the exact prompt_card_enum_typing integration target",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "If a finite gap exists, implement the smallest shared enum authority across current v2 boundaries",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run inventory, exact no-tests-fail target, no-duplicate-work, and exact-revision proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Durable card values remain canonical strings
- One typed authority supplies parsing, formatting, serde, schema, editor choices, and validation for a finite field
- Existing valid cards round-trip without drift
- Extensible identifiers are not prematurely closed
- No code change occurs when the audit proves no remaining gap

## Risks

- The issue duplicates enum work already delivered in v2
- An extensible policy identifier is incorrectly closed
- Parser normalization silently rewrites durable truth
- Schema, editor, validator, and Markdown variants diverge
- Historical v1 code is mistaken for current authority

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5824/design.md

Digest: d121563fd852950c2b62b0f48f5d217034d7288eec5a55b9598a5215ad1c99cd

## Diagram

.csdlc/prepared/issues/5824/diagram.mmd

Digest: 5846fe328a23646041deee2956de2d777e77a496fc82d707f3976361df4c5249

## Stop Conditions

- WP-05 typed-card dependency is incomplete
- The inventory cannot prove a current finite gap
- A change requires template redesign or wire-format migration
- Valid existing cards fail byte-stable round trips
- The proposed field is policy-extensible

## Handoff

Proceed only after doctor readiness.
