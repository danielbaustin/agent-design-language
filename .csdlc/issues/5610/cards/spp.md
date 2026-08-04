# Structured Planning Prompt

Template: 1.0.0

Issue: 5610

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Transplant the reviewed two-file correction, validate exact positive and negative contracts, review exact head, publish, merge, and close out.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Apply bounded lexical normalization and regressions",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Run focused FastWork contracts and exact-head review",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Publish, merge, and close out through typed v2",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Only canonical owned source roots are merged
- All existing coverage and provenance gates remain intact
- Issue #5602 terminal truth is untouched

## Risks

- Normalization could relabel escaped paths as owned
- Overbroad rejection could reject safe compiler-emitted bin/.. paths

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5610/retained/design.md

Digest: 263277f2da12412a98233272e0d5b6d1a8f57661f01f4179fc7152adf52a577d

## Diagram

.csdlc/issues/5610/retained/diagram.mmd

Digest: 11ad38b26ee71bee5c0281ff0e6c1a08c1389176e51a81c7cfc86765fb832302

## Stop Conditions

- Any need to weaken coverage or provenance gates
- Any need to edit consumer PR branches
- Any need to rewrite #5602 terminal records

## Handoff

Proceed only after doctor readiness.
