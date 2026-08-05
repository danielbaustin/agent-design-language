# Structured Planning Prompt

Template: 1.0.0

Issue: 5827

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Implement and prove WP-10 canonical multi-cycle continuity with predecessor binding, deterministic head derivation, and complete discontinuity negatives.

## Plan

Revision 14

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5826 terminal proof and inspect adl-runtime-kernel continuity.rs and live_continuity.rs before claiming the exact birthday_continuity.rs, lib.rs, tests, fixture, feature, and evidence paths.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the continuity record, canonical head derivation, two-cycle chain fixtures, and stable rejection reasons.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run deterministic replay, substitution/discontinuity negatives, privacy, and repo-relative portability lanes.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve one bounded exact-head review and publish only with correct base and Closes #5827 linkage.",
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

- Each continuity head binds its predecessor, current cycle evidence, and one identity root.
- Restart, wake, restore, snapshot, or copied state is never sufficient alone.
- No raw private state or host-specific path enters review evidence.

## Risks

- Cycle ordering or duplicate acceptance could fork continuity.
- Copied state could be mistaken for lineage continuity.
- Shared lineage paths may collide with adjacent implementation.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5827/design.md

Digest: f87f8806c1cec122f19612fefe1523f4b966ce9952109c4fa1a2e8e70dd22836

## Diagram

.csdlc/prepared/issues/5827/diagram.mmd

Digest: b8e8902ce03c1fd254d2be626f03fb412db939612b74f42de3942fcfd6cdbbb4

## Stop Conditions

- #5826 lacks terminal receipt-backed proof.
- A protected Runtime path collides with another claim.
- Deterministic replay requires altering predecessor evidence.

## Handoff

Proceed only after doctor readiness.
