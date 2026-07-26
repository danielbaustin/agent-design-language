# Structured Planning Prompt

Template: 1.0.0

Issue: 5664

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Bind #5664 on a disjoint issue worktree, add a dedicated Runtime v3 protocol-adapter module reusing existing operation primitives and Rustls/Tokio COTS dependencies, prove local authenticated transport behavior through black-box tests, run focused validation and strict Clippy, measure LoC, review exact head, and stop before publication unless explicitly requested.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Initialize and bind disjoint #5664 C-SDLC v2 ownership",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement shared authenticated protocol transport and replay cache",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement Provider, ACIP, A2A, and Cloud Bridge executor builders",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Add deterministic Tokio black-box tests for success and fail-closed cases",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Measure LoC, run focused tests, strict Clippy, exact review, and fix findings",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- All protocol calls require authenticated identity and request freshness
- Replay is rejected before remote actuation
- Timeout, cancellation, retry, and shutdown are bounded
- Credentials are represented only by opaque runtime secret handles or in-memory test values
- Cloud Bridge capability declarations are fail-closed
- No AWS path is executed
- No #5657/#5663/#5665 protected path is modified

## Risks

- Existing operation adapter receipt paths could be mistaken for real external transport proof
- Runtime API/WSS work in #5665 could overlap if protocol code is placed in runtime_api files
- Retry or idempotency could allow duplicate side effects if replay cache is late
- TLS configuration could be claimed without a typed Rustls boundary

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5664/design.md

Digest: f147b4efce2025891338fe9668a0cabe923b75dd24f98b8c2af6226e1b534b17

## Diagram

.csdlc/prepared/issues/5664/diagram.mmd

Digest: 709cd35e9d8f190aa93ed4b0e5597a202600b922e255d6c932023122a1e80b3e

## Stop Conditions

- Typed claim collision with #5657, #5663, #5665, or another active issue
- Current main no longer contains PR #5659 merge ancestry
- Implementation requires editing #5657/#5663/#5665 protected paths
- Implementation requires AWS provisioning or execution
- Any production path remains receipt-only or degraded

## Handoff

Proceed only after doctor readiness.
