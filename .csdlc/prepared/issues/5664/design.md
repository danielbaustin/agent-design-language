# Issue #5664 Design: Runtime v3 Protocol Adapters

## Outcome

Implement real Runtime v3 external-facing protocol adapters for Provider, ACIP,
A2A, and Cloud Bridge using Rust, Tokio, and Rustls-ready transport boundaries.

## Boundary

This issue owns protocol-adapter implementation and black-box tests only. It
does not own the local launch/Guardian paths from #5657, durable local adapters
from #5663, or Runtime API/WSS/Observatory feature proof paths from #5665.

## Design

Add a dedicated `protocol_adapters` module in `adl-runtime-kernel` that exposes
four production adapter builders implementing the existing `OperationExecutor`
contract:

- Provider dispatch: authenticated request/response transport with bounded
  timeout, cancellation, retry classification, and idempotent replay handling.
- ACIP: bidirectional exchange over the same framed transport contract.
- A2A: authenticated message exchange with replay rejection.
- Cloud Bridge: capability-declared forwarding with explicit unsupported,
  unauthorized, malformed, timeout, and shutdown semantics.

The module will reuse the existing Runtime v3 operation envelope, authority
permits, Tokio cancellation primitives, Rustls client/server configuration
types, and bounded retry policy. Credentials remain opaque secrets supplied by
callers at runtime and are never serialized into tracked artifacts.

## Proof

Black-box tests will stand up deterministic local Tokio services and exercise
real network message exchange without AWS or external providers. The tests must
cover success, unauthorized requests, malformed frames, timeout/cancellation,
replay rejection, retry exhaustion, unsupported cloud capability, and shutdown.

## Protected-Path Disjointness

#5664 does not touch:

- `adl-runtime-kernel/src/bin/adl-runtime-kernel.rs`
- `adl-runtime-kernel/src/config.rs`
- `adl-runtime-kernel/src/control.rs`
- `adl-runtime-kernel/tests/configuration.rs`
- `adl-runtime-kernel/tests/observatory.rs`
- `adl-runtime-kernel/tests/guardian_soak.rs`
- `infra/runtime-v3/runtime-init.toml`
- `adl-runtime/src/runtime_api.rs`
- `adl-runtime/src/runtime_api_auth.rs`
- `adl-runtime/src/observability.rs`
- `adl-runtime/src/shutdown.rs`
- `adl-runtime/tests/runtime_api_wss.rs`

