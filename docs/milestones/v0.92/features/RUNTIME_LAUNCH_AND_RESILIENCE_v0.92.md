# Runtime Launch and Resilience

## Status

Planned for WP-03. This document combines the local launch-recovery and
long-lived Agent OS plans into one bounded feature contract. It does not claim
the current Runtime satisfies the contract.

## Purpose

Provide one reliable Runtime v3 launch and survival path:

```text
Guardian process 0
  -> one Tokio/Axum/Rustls kernel
    -> bounded resident-agent and service tasks
      -> transactional durable state
        -> tracing and Vector observability
```

## Required Behavior

- Guardian is the sole supported process owner and reaps every child.
- One init file controls bind address, port, TLS, state, logs, limits, and
  service configuration; source constants do not override it.
- Axum owns the versioned HTTP and full-duplex WSS API; the separate
  Observatory consumes that API and is not served by the Runtime.
- Production adapters execute real work or fail startup before readiness; no
  degraded placeholder receives operational credit.
- Tokio bounded queues, concurrency permits, cancellation, backpressure, and
  explicit blocking boundaries protect the async scheduler.
- Runtime state survives restart through the selected transactional store and
  does not depend on temporary directories or receipt files.
- Host time may bootstrap startup, while Chronosense/SNTP begins immediately
  and refreshes trusted time without making network availability a startup
  dependency.
- Every startup, readiness, recovery, adapter failure, and shutdown cause is
  emitted through tracing and the configured Vector route.

## Proof

WP-03 must prove a real configured launch, readiness, health/API access,
authenticated WSS exchange, adapter execution, graceful shutdown, restart,
failure recovery, clean logs, and repeated lifecycle behavior on macOS, Linux,
and native Windows. Release-scale soak is separate from the first deterministic
launch proof.

## Non-Goals

- No Python server, shell supervisor, fixture credit, plaintext fallback, or
  custom HTTP/TLS/WebSocket implementation.
- No distributed relocation or mesh authority; WP-04 owns that work.
- No custom allocator, lock-free rewrite, Wasm engine, or zero-copy claim
  without measured need and separate proof.
