# Structured Task Prompt

Template: 1.0.0

Issue: 5678

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair the bounded runbook and verify its CLI/schema claims.

## Deliverables

- Correct structured-request Opus runbook
- Focused runbook contract check
- Exact review and publication evidence

## Acceptance

1. AC-1: Runbook documents --request, --out, and --log
2. AC-2: JSON example names Anthropic Opus, model identity, bounded attempts, and exact-head review input
3. AC-3: Credential handling is one-command and secret-free
4. AC-4: Focused contract check detects stale flags or missing request fields
5. AC-5: Docs-only validation and exact review pass before publication

## Dependencies

- Current adl-provider-adapter CLI source and provider communication request schema

## Inputs

- adl/src/bin/adl-provider-adapter.rs
- adl/src/provider_communication.rs
- docs/milestones/v0.91.4/review/provider_communication_substrate/PROVIDER_ADAPTER_RUNBOOK.md
- Issue 5678

## Non Goals

- No provider implementation changes
- No live credential or network probe
- No broad docs rewrite
- No tracked .adl/docs/TBD mirror mutation
