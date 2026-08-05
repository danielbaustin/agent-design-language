# Structured Planning Prompt

Template: 1.0.0

Issue: 5842

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Pin the gate SHA, enumerate every feature and supporting critical path, bind each row to exact accepted evidence, run prohibited-evidence and platform negative audits, emit blockers, and allow downstream review only after an exact-head pass.

## Plan

Revision 5

## Steps

[
  {
    "id": "S1",
    "action": "Verify all named predecessor live/typed/ancestry gates and pin the exact quality-gate revision.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Enumerate the complete canonical feature and critical-path universe without dropping planned or blocked rows.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Resolve each row to exact implementation, validation, review, integration, platform, and terminal evidence.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run prohibited-evidence, stale-SHA, ancestry, platform, provider-identity, and unsupported-claim negative audits.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Emit the gate/blocker packet, resolve exact-head review, and block or permit downstream review truthfully.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- No tracked work on main
- No scope absorption across work packages
- Evidence claims remain exact-revision and source-grounded

## Risks

- Dependency drift
- Scope overlap
- Insufficient real-behavior proof

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5842/design.md

Digest: 2a2f34956c88c9f8f4024f563b8612acac8ab046042d063cd75206202a3e362e

## Diagram

.csdlc/prepared/issues/5842/diagram.mmd

Digest: b1829fae5cf441c5f53f7e5d0184bdbcd7bf61e53855dca5d99241bf03b6c794

## Stop Conditions

- Protected-path collision
- Contradictory dependency evidence
- Required proof cannot be produced within issue scope

## Handoff

Proceed only after doctor readiness.
