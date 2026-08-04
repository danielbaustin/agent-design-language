# Runtime v3 API Contract Design for #5701

## Boundary

#5701 defines machine-readable OpenAPI 3.1 contracts for the Runtime v3 Core API and Runtime-hosted Observatory API. The HTML Observatory app remains external. The runtime contract may describe only reachable production HTTP/WSS endpoints and schemas; fixture-only, simulated, degraded, unavailable, or aspirational features are excluded or marked fail-closed as unavailable.

## Product Surface

The canonical artifacts live under repository-owned API specification paths and are served by Runtime v3 through configured Axum routes:

- `docs/api/runtime-v3/v1/openapi.json`
- `docs/api/runtime-v3/v1/observatory.openapi.json`
- `docs/api/runtime-v3/v1/API_VERSIONING.md`
- `adl-runtime-kernel/src/openapi.rs`
- focused route-parity and contract-validation tests

The OpenAPI documents define independent API identities for Runtime Core API v1 and Observatory API v1. WSS endpoints are represented as authenticated upgrade operations with vendor extensions for inbound and outbound frame schemas.

## Runtime Integration

The runtime should expose:

- `GET /v1/openapi.json`
- `GET /v1/observatory/openapi.json`

Both endpoints return the exact canonical contracts on the configured Runtime API port. Port numbers, public base URL, TLS paths, and observatory origins remain init/config-driven, not hard-coded into the contract implementation.

## Validation

Validation must prove:

- both OpenAPI artifacts parse as OpenAPI 3.1 JSON;
- every declared path/method maps to a real Axum route;
- every real public Runtime v3 route is represented exactly once;
- schemas and examples resolve;
- WSS upgrade operations document authentication plus inbound/outbound frames;
- bounded generated-client smoke exercises real Guardian-launched HTTP and WSS behavior when route integration is available.

## Dependency Coordination

Active WP-12 #5344 currently protects Runtime v3 launch, config, control, and observatory code paths. #5701 starts with disjoint spec/validation paths. If route serving requires paths protected by #5344, #5701 must stop for typed path transfer or wait until #5344 releases those paths.
