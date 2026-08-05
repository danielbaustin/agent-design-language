# Structured Planning Prompt

Template: 1.0.0

Issue: 5665

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Complete the real `adl-runtime` WSS API proof path, record truth in health/telemetry/matrix artifacts, validate locally, and publish only after exact review.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inspect and simplify existing API/auth/observability surfaces, measuring physical LoC before edits",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement authenticated bidirectional WSS, auth rotation/revocation, shutdown, health, and telemetry truth",
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
    "action": "Add end-to-end feature/adapter matrix and focused tests without fixture-only or metadata-only proof",
    "acceptance_ids": [
      "AC-1",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused validation, strict Clippy, exact review, and prepare publication",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- API-only runtime boundary is preserved
- HTML Observatory remains a separate client
- proof exercises real Rust API/WSS behavior
- telemetry never claims unsupported sink fields
- protected paths for #5657/#5663/#5664 are not touched

## Risks

- Existing runtime API may lack a complete TLS/WSS listener
- Cross-platform live socket validation may expose OS-specific port or certificate behavior
- LoC reduction may require removing obsolete duplicate paths after implementation scope is understood
- FastWork validation may be unavailable locally

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/issues/5665/retained/design.md

Digest: 5595efc4dc208a4b26239175ca46b9f1d6ba64da63d1c6e19462f8881a4d506b

## Diagram

.csdlc/issues/5665/retained/diagram.mmd

Digest: 11c0edb080ad0e45db25657a7f43bfd7c5c8326cfd10a30920070832c0ddb3f5

## Stop Conditions

- Protected-path collision with #5657/#5663/#5664
- AWS is required
- Only URL, fixture, metadata, Python, or degraded proof is possible
- feature matrix retains an unresolved claimed feature
- strict Clippy or focused tests fail
- actionable exact-review finding remains unresolved

## Handoff

Proceed only after doctor readiness.
