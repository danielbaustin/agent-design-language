# Structured Planning Prompt

Template: 1.0.0

Issue: 5691

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Implement production Runtime v3 tracing-to-Vector observability parity with one pinned Vector pipeline, durable master log, OTLP export proof, redaction, drain/failure observability, and clean-log auditing; review once before publishing.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bind #5691 worktree, verify Runtime v2 parity source, and establish protected paths",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement Runtime v3 tracing subscriber, Vector lifecycle, config, redaction, status/API, and auditor",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run real pinned-Vector proof, strict Clippy, exact-head review, fixes, and ready PR",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "status": "pending"
  }
]

## Invariants

- Vector owns durable and remote routing
- Runtime stdout/stderr machine/human contracts remain preserved
- pipeline failures are first-class runtime health evidence
- no missing capability is reported as healthy or degraded success
- proof logs are issue-local

## Risks

- accidentally recreating a custom logger instead of Vector-owned durability
- OTLP proof becoming fixture-only
- status API overclaiming remote export health
- Windows path or process-lifecycle differences
- scope collision with WP-12 lifecycle harness

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/issues/5691/retained/design.md

Digest: 13f250b1e7c82f3bb0aa7d0dc41f77547639292508eb8c680e48813e8af58002

## Diagram

.csdlc/issues/5691/retained/diagram.mmd

Digest: 9b6e31fa7377a82d1c7ab8f6e2b0b2c6a8b2e4786ceec37bbcedcd7833068a9a

## Stop Conditions

- protected path collision with an active owner
- pinned Vector binary missing or wrong version
- Vector cannot validate the generated config
- real OTLP receiver exchange cannot be proven without widening scope
- strict Clippy or exact-head review finds unresolved production blockers

## Handoff

Proceed only after doctor readiness.
