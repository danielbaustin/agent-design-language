# Structured Task Prompt

Template: 1.0.0

Issue: 5563

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement only the typed initialized stale-design recovery and its focused regression.

## Deliverables

- Bounded approve-design recovery semantics
- Gate 2 regression

## Acceptance

1. AC-1: Stale initialized-approved design cannot reach ready before recovery
2. AC-2: Typed recovery validates CAS, active claim, reviewer, and actual stale authored inputs
3. AC-3: Recovery atomically refreshes SPP/VPP digests and audit truth
4. AC-4: Doctor passes and ready transition succeeds after recovery

## Dependencies

- none

## Inputs

- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate2.rs
- .csdlc/issues/5306

## Non Goals

- No terminal recovery changes
- No v1 wrapper restoration
- No AWS or Spot validation
