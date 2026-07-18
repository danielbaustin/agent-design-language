# Structured Planning Prompt

Template: 1.0.0

Issue: 5383

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Repair live issue routing, bind a setup worktree, author the full v0.91.8 planning package, validate docs/YAML/link/placeholder truth, review, and publish.

## Plan

Revision 1

## Steps

[
  {
    "id": "step-1",
    "action": "Repair and verify #4641/#5384/#5383 issue routing truth",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "step-2",
    "action": "Create complete v0.91.8 planned-posture milestone and feature documentation package",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "step-3",
    "action": "Run focused docs/YAML/link/placeholder validation, review, and publish setup PR",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- v0.91.8 is a bridge prerequisite for v0.92, not a replacement for v0.92
- planning docs cannot claim implementation, deployment, parity, deletion, or release approval before evidence exists
- Runtime v3 and C-SDLC v2 ownership must not be pulled back into ADL core
- root checkout remains clean on main for tracked implementation work

## Risks

- live issue wave and local planning package drift
- overclaiming execution readiness from planning text
- forgetting moved WP-14 child issues in the issue wave
- stale v0.91.7 or v0.92 handoff references

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

docs/milestones/v0.91.8/setup/5383/DESIGN.md

Digest: 410f29ed4197e0bdd3c864b78bb0ad82513c4ab1a9eb11f47e86be3dae852c30

## Diagram

docs/milestones/v0.91.8/setup/5383/DIAGRAM.mmd

Digest: 1d2a8731a116cf770665f674f1ab4183276a7386c6c2f0351dc98f383dc17429

## Stop Conditions

- issue routing cannot be verified after repair
- v0.91.8 live issue list changes in a way that invalidates the wave map
- tracked edits would have to occur on main
- validation shows planning package structure is not reviewable

## Handoff

Proceed only after doctor readiness.
