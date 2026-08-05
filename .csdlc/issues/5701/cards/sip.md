# Structured Intent Prompt

Template: 1.0.0

Issue: 5701

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Define first-class OpenAPI 3.1 Runtime Core API v1 and Observatory API v1 contracts suitable for client generation and runtime discovery.

## Required Outcome

Canonical OpenAPI contracts cover every real Runtime v3 and Observatory endpoint without fixture, degraded, receipt-only, or unavailable claims; validation proves route parity and client-generation suitability.

## Scope

- Runtime Core API v1 OpenAPI contract
- Runtime-hosted Observatory API v1 OpenAPI contract
- WSS upgrade and inbound/outbound frame schema documentation
- Route inventory and schema/reference validation
- Runtime discovery endpoint integration only after #5344 releases or transfers protected router/config paths

## Authority

- #5701 owns only its issue-local state plus docs/api/runtime-v3/v1 and adl-runtime-kernel/tests/openapi_contract.rs initially
- Active #5344 owns adl-runtime-kernel router/config/bin/control/lib paths and must not be edited by #5701 without typed transfer or release
- The HTML Observatory application remains separate and out of runtime scope
- Unsupported Provider, ACIP, A2A, Cloud Bridge, or other unavailable feature routes must not be documented as operational

## Assumptions

- none

## Operator Constraints

- Use only typed C-SDLC v2 lifecycle operations
- Never write tracked files on main
- Do not use /private/tmp
- Keep constants such as port numbers, public base URL, TLS paths, and log locations in init/config rather than code
- Use one exact-head subagent review immediately before PR publication
- PR body must contain Closes #5701
