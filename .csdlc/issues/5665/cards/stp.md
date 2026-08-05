# Structured Task Prompt

Template: 1.0.0

Issue: 5665

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement, validate, review, and prepare publication for the bounded Runtime v3 WSS/API proof surface only.

## Deliverables

- authenticated WSS Runtime v3 API implementation
- health-state distinction model
- sink-bounded telemetry model
- clean-checkout init file using port 20997
- feature/adapter matrix
- focused Rust tests and strict Clippy evidence
- LoC measurement evidence

## Acceptance

1. AC-1: WSS handshake, authentication, bidirectional frames, rotation, revocation, and shutdown are tested through the real API path
2. AC-2: Observatory health distinguishes unimplemented, unavailable, failed, and healthy
3. AC-3: telemetry emits only fields supported by configured sinks
4. AC-4: clean-checkout init uses one configured file and port 20997
5. AC-5: feature/adapter matrix has no unresolved claimed feature
6. AC-6: focused tests, strict Clippy, LoC net reduction, and exact pre-PR review complete

## Dependencies

- GitHub issue #5665
- existing adl-runtime Runtime v3 API/auth/observability code
- existing Axum/Tokio/Rustls runtime dependency stack

## Inputs

- adl-runtime/Cargo.toml
- adl-runtime/src/runtime_api.rs
- adl-runtime/src/runtime_api_auth.rs
- adl-runtime/src/observability.rs
- adl-runtime/src/shutdown.rs
- demos/v0.91.7/html-observatory/README.md

## Non Goals

- HTML Observatory UI redesign
- AWS validation or deployment
- Unity Observatory implementation
- Runtime kernel launch/config changes owned by #5657
