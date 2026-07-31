# Structured Planning Prompt

Template: 1.0.0

Issue: 5719

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add a bounded static podcast/demo path classification, prove #5716-like paths avoid full hosted coverage, preserve producer lanes for behavioral changes, review, and publish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inspect current path-policy coverage selection and reproduce a #5716-like changed path set.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Patch the selector with the smallest static podcast/demo classification.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused tests for static podcast/demo paths and Rust/tooling preservation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run path-policy, workflow contract, and validation-manager focused checks.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run bounded review, record lifecycle truth, and publish with Closes #5719.",
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

- The aggregator job remains the stable coverage check.
- Full hosted coverage remains fail-closed for source/runtime/provider/tooling-policy changes.
- Static podcast/demo page changes do not pay for duplicate hosted producer coverage.

## Risks

- Over-broad static classification could skip coverage for code-bearing demo changes.
- Workflow contract assertions could drift from the selector output.
- Policy tests are large and may hide the focused regression if no specific fixture is added.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5719/retained/design.md

Digest: 669865d6d9df6873d4b8b4f54a492499d00d3173912437883abddd0bebee5b35

## Diagram

.csdlc/issues/5719/retained/diagram.mmd

Digest: 1429f9a11d794b37f2a778dd3691f646b89873013d61a226235092ef77f5cb82

## Stop Conditions

- The selector cannot distinguish static podcast/demo files from code-bearing demo/runtime files.
- A workflow contract proves both hosted producer lanes are still selected after the path-policy change.
- A Rust/runtime/provider fixture regresses to skipped coverage.

## Handoff

Proceed only after doctor readiness.
