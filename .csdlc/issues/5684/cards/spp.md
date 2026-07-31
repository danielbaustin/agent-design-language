# Structured Planning Prompt

Template: 1.0.0

Issue: 5684

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add shared resilience crate, narrow GitHub owner binaries, install/coexistence enforcement, and focused validation.

## Plan

Revision 1

## Steps

[
  {
    "id": "step-1",
    "action": "Add shared resilience retry/backoff crate and wire C-SDLC/runtime users.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "step-2",
    "action": "Add split GitHub issue and PR owner binaries while preserving compatibility facade.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "step-3",
    "action": "Update install/coexistence manifests and Gate 10A tests for the full required binary set.",
    "acceptance_ids": [
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "step-4",
    "action": "Update current docs and operator skill guidance for the split GitHub surface.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "step-5",
    "action": "Repair current bootstrap validation guidance away from deleted structured-prompt shell wrappers.",
    "acceptance_ids": [
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "step-6",
    "action": "Run focused validation and stable install/coexistence proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- GitHub mutation paths must reconcile exact markers after writes
- Stable binary install must be generated from reviewed source, not target-cache assumptions
- csdlc-merge remains present in required binary set

## Risks

- Old stable csdlc-install embeds stale manifest until refreshed from new source
- Compatibility facade could mask split-binary regressions without explicit rejection tests

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5684/design.md

Digest: 71c69c042f97526ace7f3ad9dc7c42d109e1b04d274e8afdf59893176c698eb8

## Diagram

.csdlc/issues/5684/diagram.mmd

Digest: c25ed1b99d9a34cbda12b144c73bd082ce22349de612069c8ec588c9c122f0a6

## Stop Conditions

- C-SDLC owner-binary install fails closed
- Focused GitHub split tests fail
- Protected-path conflict appears

## Handoff

Proceed only after doctor readiness.
