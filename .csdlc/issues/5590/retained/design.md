# Runtime v3 Parity-D Secure Operations Design

Issue #5590 owns the secure operations edge of Runtime v3 under WP-14
acceptance umbrella #5361. This preparation packet defines the complete outcome without
editing product code. Parity-A #5591 must be integrated before this claim may
expand into Runtime source paths.

## Existing source-grounded architecture

- `adl-runtime-kernel/src/config.rs` owns the deny-unknown-fields init model,
  configured listener address, HTTPS public base URL, TLS certificate/key
  paths, allowed Observatory origins, and agent/weather configuration.
- `adl-runtime-kernel` uses Axum and `axum-server` with rustls for the control
  API. Ed25519-signed command envelopes and explicit read/stop capabilities
  remain transport-independent authority.
- `demos/v0.91.7/html-observatory/app.js` consumes `/v1/observatory`, requires
  an HTTPS Runtime API base, and uses a session-scoped bearer token.
- `adl-runtime-kernel/src/telemetry.rs` and the checked-in Vector configuration
  preserve stderr `adl_event` output while Vector owns parsing, buffering,
  retry, transformation, and remote export. Runtime v3 does not embed or
  reimplement OpenTelemetry collection.
- The external guardian launches exactly
  `adl-runtime-kernel serve --init <path> --continuity-root <path>`, owns child
  signals/reaping/restart delay, and never becomes an in-process sidecar.
- Runtime selection must be an operational process/configuration transition,
  not the existing report-only `adl runtime-v3 select` surface. This issue
  neither imports nor edits Runtime v2.

## Revision identity

The preparation corpus starts at exact base
`6d0f6115632a06619544b8ad4792792e741f1f31`. The first reviewed preparation
head is `2f26da4455efd4dfc7ab6c65df5d19327fe765c8`; every repair and re-review must
descend from that head, and retained validation must record both revisions.

## One configuration-driven network model

The init file is the single operator surface for the Runtime listener, public
HTTPS base, TLS material, allowed browser origins, and Runtime agent settings.
Port 20997 is the documented default but discovery and readiness must report the
actual bound/configured listener. Product behavior must not infer a public IP,
embed an address literal, or maintain separate local and remote policy stacks.

Local and remote clients use the same HTTPS router and authorization model.
Locality changes configuration, certificates, routing, and allowed origins;
it does not weaken authentication. Plain HTTP is never an accepted Runtime v3
mode. WebSocket support, when implemented, must share the same TLS endpoint,
origin policy, bearer/session authority, frame bounds, redaction, and lifecycle
truth as authenticated HTTP reads.

Remote routing may terminate at an operator-owned gateway or elastic endpoint,
but this issue proves only a credential-free configuration and contract
boundary. It performs no AWS operation and makes no deployment claim.

## Guardian, telemetry, and rollback

The guardian reads the declared init path and launches one canonical kernel
child. Restart policy must distinguish intentional graceful stop, invalid
configuration, bounded transient failure, and crash-loop exhaustion. Pressure
stop closes admission, drains accepted work, commits the terminal checkpoint,
emits terminal observability, and exits cleanly before any eligible restart.

Vector remains optional to kernel liveness. Its absence may classify telemetry
as degraded but cannot disable stderr events, health, authenticated control, or
graceful shutdown. Retained proof must show redaction and must never contain
tokens, private keys, certificate contents, machine-local absolute paths, or
uncontrolled upstream error text.

Rollback proof must execute an operational selector transition from the
candidate Runtime v3 process/configuration to the previous approved Runtime v3
process/configuration. It must prove candidate health, perform the transition,
then prove restored authenticated HTTPS service health at the prior selector
revision. A report-only selector, environment echo, metadata assertion, or
in-memory facade receives no credit. The transition retains Runtime v3 evidence
and never touches Runtime v2 implementation. No automatic default switch or
decommission is authorized.

## Future implementation scope

After #5591 is integrated and protected-path collision checks pass, a typed
`csdlc-bind` amendment may add only the smallest verified subset of:

- `adl-runtime-kernel/` for HTTPS, WebSocket, discovery, config, control, and
  Observatory behavior;
- `adl-runtime/` for the external-child guardian boundary only;
- `infra/runtime-v3/` for configuration and Vector wiring;
- `demos/v0.91.7/html-observatory/` for live authenticated consumption;
- focused Runtime v3 proof scripts and `.csdlc/evidence/5590/`.

The amendment must exclude Runtime v2, cloud deployment, sidecars, unrelated
demo code, and paths owned by active Parity-B or Parity-C claims.

## Exact-revision acceptance

Acceptance requires one committed revision proving configured HTTPS local and
remote access, authenticated HTTP and WebSocket Observatory consumption, actual
bound-address discovery, guardian launch/restart/pressure-stop behavior, Vector
degradation and routing boundaries, and explicit rollback. Positive and
fail-closed negative cases must execute against production Runtime v3 paths;
fixture-only, prose-only, degraded, deferred, or metadata-only evidence receives
no parity credit. Every filtered Rust test lane inventories the exact filter,
records a positive test count, fails closed on zero matches, and only then runs
the filtered tests.
