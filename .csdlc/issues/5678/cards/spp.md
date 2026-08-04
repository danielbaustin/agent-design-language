# Structured Planning Prompt

Template: 1.0.0

Issue: 5678

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Read the actual CLI and request schema, write a concise tracked runbook, add a focused contract check, validate docs hygiene, obtain exact review, and publish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bind issue 5678 with disjoint documentation and test paths",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Repair the tracked Opus runbook from current Rust source evidence",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add and run the focused CLI/schema contract check",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run exact review and publish only after findings are resolved",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Runbook claims are source-backed
- No secrets or absolute local paths are recorded
- Provider success is never inferred from adapter invocation alone
- The control-plane lifecycle remains typed

## Risks

- CLI or request schema drift can stale the procedure
- A prompt summary can be mistaken for source evidence
- The ignored operator-local TBD mirror can diverge from the tracked canonical runbook

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5678/retained/design.md

Digest: a495000977ebfb479c685463ea410aca58567690f653a268cc32b43281d91d48

## Diagram

.csdlc/issues/5678/retained/diagram.mmd

Digest: ecdb19c370cdd57aa510e1430d85c09453a2eaac9b9895b57a36c56310fe6db8

## Stop Conditions

- Current CLI/schema cannot be verified
- A requested change needs provider credentials or AWS
- Work would touch provider implementation or lifecycle code

## Handoff

Proceed only after doctor readiness.
