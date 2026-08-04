# Structured Intent Prompt

Template: 1.0.0

Issue: 5678

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make the Opus review procedure match the current Rust provider adapter interface and remain resistant to CLI drift.

## Required Outcome

A source-grounded runbook and focused contract check document and verify the structured JSON adapter invocation.

## Scope

- docs/tooling/OPUS_REVIEW_RUNBOOK.md
- adl/tools/test_opus_review_runbook.sh

## Authority

- Issue 5678 owns only the runbook and its focused contract check
- Provider implementation and lifecycle authority remain unchanged

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 only
- No main checkout edits
- No AWS
- Never expose provider credentials
- No live provider call required
