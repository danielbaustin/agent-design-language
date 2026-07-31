# Structured Task Prompt

Template: 1.0.0

Issue: 5701

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement the smallest complete OpenAPI contract and validation surface for reachable Runtime v3 and Observatory APIs, then publish a ready PR if route-serving integration is not blocked by #5344 ownership.

## Deliverables

- Canonical OpenAPI 3.1 Runtime Core API v1 document
- Canonical OpenAPI 3.1 Observatory API v1 document
- API versioning and compatibility note
- Focused Rust validation proving route parity and contract integrity
- Exact-head subagent review evidence
- Ready PR with Closes #5701

## Acceptance

1. AC-1: Canonical OpenAPI documents declare OpenAPI 3.1.x, stable Runtime Core API v1 and Observatory API v1 identities, security schemes, paths, operations, components, errors, and examples
2. AC-2: Every production Runtime v3 HTTP route is represented exactly once in the correct contract and every documented operation maps to a real Axum handler
3. AC-3: Observatory API versions snapshot/query responses, feature and health state, telemetry/events, and implemented Observatory data resources independently from the HTML app
4. AC-4: Every production WSS endpoint documents the authenticated HTTP upgrade plus inbound/outbound frame schemas, close/error behavior, correlation identifiers, subscriptions, and commands
5. AC-5: Contracts contain no fixture-only, receipt-only, simulated, degraded, unavailable, or unimplemented operational claims
6. AC-6: Runtime startup exposes canonical contracts at /v1/openapi.json and /v1/observatory/openapi.json once the protected router/config path gate is available
7. AC-7: Focused validation parses OpenAPI, resolves references, checks route parity, and proves deterministic client-generation suitability
8. AC-8: One exact-head subagent review passes before publication
9. AC-9: Ready PR includes Closes #5701 and does not wait for post-merge typed closeout

## Dependencies

- GitHub issue #5701
- Current origin/main 26464ab54f81714478973556a9f56d3c77c6c203
- Active WP-12 #5344 branch ca242a5ab96a43ea1ed1ca6a3fedd61bde2f8b6d and protected path claim

## Inputs

- adl-runtime-kernel/src/control.rs route inventory
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs runtime startup
- adl-runtime-kernel/src/config.rs init/config constants
- adl-runtime-kernel/tests/guardian_soak.rs live HTTP/WSS behavior
- infra/runtime-v3/runtime-init.toml configured API port and TLS shape
- OpenAPI Initiative OpenAPI 3.1 specification

## Non Goals

- Embedding the HTML Observatory app in the runtime
- Claiming APIs for unreachable or unimplemented behavior
- Replacing Axum, Tokio, Rustls, or the authenticated transport stack
- Committing generated SDKs for every language
