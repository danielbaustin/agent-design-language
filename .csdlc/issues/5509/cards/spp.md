# Structured Planning Prompt

Template: 1.0.0

Issue: 5509

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Recognize the exact Runtime v3/CSM family, execute each crate's focused tests independently, compose coverage, and preserve the existing fallback.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Add the bounded mixed-crate classifier and execution plan",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Align focused coverage routing and summary composition",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run contract tests, review, publish, and merge",
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

- Every accepted path belongs to the declared Runtime v3 or CSM family
- Both crates run when both crates change
- All other mixed-crate shapes retain existing fail-closed behavior
- No Runtime v2 source is modified

## Risks

- An overly broad classifier could suppress required full validation
- A single-crate runner could silently omit half of the change
- Coverage summary composition could report only one crate

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/issues/5509/retained/design.md

Digest: b62620c42c4c68aa5ab6b70aec61c63346098008aeea9ad3751820d2e07d770a

## Diagram

.csdlc/issues/5509/retained/diagram.mmd

Digest: 574be616d68a4104dc22f7ff9836dd5a2ac93f84dcb1731268bc1384a263c68c

## Stop Conditions

- The route requires modifying Runtime v2
- The accepted family cannot be expressed as a closed path set
- Validation requires AWS

## Handoff

Proceed only after doctor readiness.
