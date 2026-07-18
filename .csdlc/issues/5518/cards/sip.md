# Structured Intent Prompt

Template: 1.0.0

Issue: 5518

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Provide one fail-closed typed repair for stale completed work in a closed-out SPP.

## Required Outcome

The typed operation atomically completes #5516 SPP S3 and refreshes its terminal receipt without permitting general terminal mutation.

## Scope

- csdlc-v2 terminal repair store and CLI
- focused terminal repair tests
- #5516 terminal plan and receipt
- #5518 lifecycle records

## Authority

- #5518 is the active repair authority
- #5516 is the closed-out target
- Only forward SPP step completion is allowed

## Assumptions

- none

## Operator Constraints

- none
