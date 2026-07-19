# Structured Planning Prompt

Template: 1.0.0

Issue: 5541

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Correct both stale current guidance surfaces, add a focused guard, validate, review, and publish.

## Plan

Revision 1

## Steps

[
  {
    "id": "fix-guidance",
    "action": "Align current skill and workflow guidance with final v2 authority",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "guard-review",
    "action": "Add stale-guidance guard, validate, and review exact revision",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  }
]

## Invariants

- Final v1_sunset authority remains explicit
- Historical evidence remains immutable
- No v1 command surface is restored
- No AWS or Spot validation

## Risks

- An overbroad text scan could reject immutable historical evidence
- A rewritten workflow could name binaries without the resolver/install contract

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5541/design.md

Digest: c9414eecb80e10539771549706400354580c98762b800aebf0c683bc155d2c79

## Diagram

.csdlc/prepared/issues/5541/diagram.mmd

Digest: 3a249d86dddcf5c290342c46da659600a2224d90f1237c39b179551555c72042

## Stop Conditions

- Any need to restore a v1 wrapper
- Any historical evidence mutation

## Handoff

Proceed only after doctor readiness.
