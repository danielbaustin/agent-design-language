# Structured Intent Prompt

Template: 1.0.0

Issue: 5527

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Provide one fail-closed typed repair for stale SOR artifact references in a closed-out record.

## Required Outcome

The typed operation atomically replaces #5390's deleted diagram reference with its byte-identical retained path and refreshes its terminal receipt.

## Scope

- csdlc-v2 terminal repair store and CLI
- focused terminal repair tests
- #5390 terminal SOR and receipt
- #5527 lifecycle records

## Authority

- #5527 is the active repair authority
- #5390 is the closed-out target
- Only exact stale-to-retained SOR artifact reference replacement is allowed

## Assumptions

- none

## Operator Constraints

- none
