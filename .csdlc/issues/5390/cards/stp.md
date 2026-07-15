# Structured Task Prompt

Template: 1.0.0

Issue: 5390

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair only the Runtime v3 control transport and truthful discovery path.

## Deliverables

- Fail-closed TLS configuration
- Native HTTPS control listener
- Actual bound-port readiness and feed data
- Focused TLS and ephemeral-port tests
- Updated init and entrypoint documentation

## Acceptance

1. Production serve path has no plain-HTTP control listener
2. TLS certificate and private-key paths are required and validated
3. Local HTTPS Observatory fetch succeeds without a gateway
4. Ephemeral listener reports its actual port
5. Disallowed origins receive no CORS grant
6. No Runtime v3 cutover or Runtime v2 deletion claim changes
7. Focused tests, formatting, diff checks, and review pass

## Dependencies

- Runtime v3 explicit opt-in control path
- axum-server 0.8.0 Rustls integration

## Inputs

- adl-runtime-kernel
- infra/runtime-v3/runtime-init.toml
- demos/v0.91.7/html-observatory/app.js
- docs/architecture/RUNTIME_V3_ENTRYPOINT_SWITCH.md

## Non Goals

- External gateway or sidecar
- Certificate authority automation
- Host trust-store mutation
- Default Runtime v3 cutover
- Runtime v2 deletion or decommission
