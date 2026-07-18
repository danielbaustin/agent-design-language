# Structured Intent Prompt

Template: 1.0.0

Issue: 5455

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Ensure stable C-SDLC v2 binaries cannot silently lag tracked source semantics.

## Required Outcome

Install receipts bind binaries to the exact repository revision and verification rejects stale provenance.

## Scope

- csdlc-v2/src/operator.rs
- csdlc-v2/tests/gate10a.rs

## Authority

- csdlc-v2 operator install and coexistence verification

## Assumptions

- none

## Operator Constraints

- none
