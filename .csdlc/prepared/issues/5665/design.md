# Runtime v3 Authenticated WSS And Feature Proof Design (#5665)

## Outcome

Complete the API-only Runtime v3 launch proof surface by adding a real authenticated
bidirectional WSS path through the existing Axum/Tokio/Rustls runtime API, truthful
Observatory health distinctions, sink-bounded telemetry fields, one clean-checkout
init file on port 20997, and an end-to-end feature/adapter proof matrix.

## Scope

- `adl-runtime` Runtime v3 API/auth/observability/proof code and focused tests.
- TLS WebSocket handshake, authentication, bidirectional frames, token rotation,
  token revocation, and shutdown behavior.
- Observatory-facing health classification values for unimplemented, unavailable,
  failed, and healthy runtime capabilities.
- Telemetry fields constrained to configured sink capabilities.
- One issue-local init/config proof file for clean-checkout launch on port 20997.
- Feature and adapter proof matrix that marks every claimed Runtime v3 surface as
  proven or blocks the issue before publication.

## Boundaries

- Runtime remains API-only.
- HTML Observatory remains a separate client and is not redesigned here.
- No AWS, Python proof, fixture-only proof, URL-only proof, metadata-only proof,
  or degraded proof.
- Do not touch #5657 protected launch/config paths:
  `adl-runtime-kernel/src/bin/adl-runtime-kernel.rs`,
  `adl-runtime-kernel/src/config.rs`, `adl-runtime-kernel/src/control.rs`,
  `adl-runtime-kernel/tests/configuration.rs`,
  `adl-runtime-kernel/tests/observatory.rs`,
  `adl-runtime-kernel/tests/guardian_soak.rs`, or
  `infra/runtime-v3/runtime-init.toml`.

## Plan

1. Inspect existing `adl-runtime` API/auth/observability surfaces and remove
   obsolete duplicate wrapper paths where they overlap the requested WSS proof.
2. Implement the WSS exchange with COTS Axum/Tokio/Rustls crates already present
   or added narrowly to `adl-runtime`.
3. Add health-state and telemetry capability models that avoid overclaiming sink
   support.
4. Add one clean-checkout init file configured for port 20997.
5. Add focused tests and a feature/adapter matrix proof that exercises the real
   API path end to end, including auth rotation, revocation, and shutdown.
6. Measure before/after physical LoC for touched runtime surfaces and require a
   net reduction.
7. Run focused tests, strict Clippy, and one exact pre-PR review.

## Validation

- Focused `adl-runtime` Rust tests for WSS auth, bidirectional frames,
  rotation/revocation, shutdown, health distinctions, telemetry limits, and the
  feature/adapter matrix.
- Strict Clippy for `adl-runtime`.
- FastWork validation where configured and available without AWS.
- Exact pre-PR review of the bound worktree revision.

