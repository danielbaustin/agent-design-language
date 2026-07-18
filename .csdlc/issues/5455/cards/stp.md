# Structured Task Prompt

Template: 1.0.0

Issue: 5455

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement and prove stale owner-binary provenance enforcement.

## Deliverables

- Source provenance field and verifier
- Focused regression test

## Acceptance

1. AC-1: Receipt records source revision
2. AC-2: Verification fails closed with explicit stale provenance
3. AC-3: Focused Gate 10A proof passes

## Dependencies

- none

## Inputs

- csdlc-v2/src/operator.rs
- csdlc-v2/tests/gate10a.rs
- csdlc-v2/src/bin/csdlc-install.rs

## Non Goals

- No v1 wrapper restoration
- No shell or Python fallback
