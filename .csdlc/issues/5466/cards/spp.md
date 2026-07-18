# Structured Planning Prompt

Template: 1.0.0

Issue: 5466

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add a narrow merged-publication reconciliation command and prove all identity and review gates.

## Plan

Revision 1

## Steps

[
  {
    "id": "implement",
    "action": "Implement and prove merged-head reconciliation",
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

- Exact-revision review remains mandatory
- Terminal SHA is never fabricated
- Normal draft publication stays fail closed
- No direct card or index edits

## Risks

- A permissive merged-PR path could bypass exact review if identity validation is incomplete.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5466/design.md

Digest: f8da3574c660df3311c94b96773ec15633a26d60074f75415b72982a248b942f

## Diagram

.csdlc/prepared/issues/5466/diagram.mmd

Digest: e03b7f820c397553d8348574be834ea8ba96d980475c4e1edfad789cf1fc9d29

## Stop Conditions

- Any need to fabricate terminal evidence
- Any bypass of typed v2 state mutation

## Handoff

Proceed only after doctor readiness.
