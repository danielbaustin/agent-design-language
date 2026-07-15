# Issue 5390 design

## Decision

Runtime v3 owns TLS termination for its control API. The production serve path
loads an operator-supplied PEM certificate chain and private key through
axum-server's maintained Rustls integration. Missing, unreadable, or invalid
TLS material fails before the kernel reports readiness.

The init contract carries repository-relative or absolute certificate and key
paths. No private key is committed, generated implicitly, or installed into a
host trust store.

After binding, the listener's local address is captured once and supplied to
the ControlService. The ready event and Observatory feed report that address,
so ephemeral and non-default ports cannot be mislabeled as 20997.

## Invariants

- No production plain-HTTP control listener.
- No external TLS gateway or sidecar.
- No committed private key or automatic trust-store mutation.
- CORS remains an exact configured HTTPS-origin allowlist.
- Signed commands remain mandatory for mutation.
- Runtime v3 remains explicit opt-in; Runtime v2 remains default and rollback.

## Proof

- Configuration rejection tests for missing or invalid TLS path fields.
- Native TLS listener test with ephemeral certificate, ephemeral port, allowed
  origin, and denied origin.
- Ready-event formatting test against an actual bound socket.
- Existing control, configuration, parity, and guardian tests remain green.
