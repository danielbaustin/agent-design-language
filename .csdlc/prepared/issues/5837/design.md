# Issue 5837 Design: Observatory And Unity Consumer Integration

## Outcome And Boundary

Issue 5837 integrates two separate clients, the HTML Observatory and Unity
Observatory, with the same versioned Runtime v3 HTTP projection and
authenticated full-duplex WSS contract. Both consumers must render real
Runtime state, reconnect after Guardian-owned restart, perform only authorized
actions, and expose unavailable/stale/denied states without substituting
fixtures or retained packets for live proof.

Runtime remains an API-only service. HTML and Unity remain separate
applications with their approved designs and cannot move UI code, assets, or
client-specific schema forks into Runtime.

## Source Baseline

- `demos/html-observatory/` already consumes `/v1/observatory`, `/v1/ready`,
  `/v1/control`, and `/v1/observatory/ws` from a versioned config.
- `demos/v0.91.6/unity-observatory/` contains the Unity shell, runtime
  contract resource, compatibility verifier, and batch validation surfaces.
- `adl-runtime/src/runtime_api.rs`, `runtime_api_auth.rs`, and
  `tests/runtime_api_wss.rs` own Runtime HTTP/WSS and authentication behavior.
- `docs/api/runtime-v3/v1/observatory.openapi.json` and architecture projection
  contracts are schema inputs.
- `docs/milestones/v0.92/features/OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md`
  requires real reads/writes, redaction/refusal, reconnect, and browser/native
  proof while preserving separate consumer ownership.

## Design

Both consumers bind through one compatibility contract containing API version,
endpoint discovery, schema/catalog version, projection audience, stable IDs,
event ordering/correlation, reconnect cursor, authentication mode, and
backpressure limits. Public read projections require no write session.
Authenticated WSS login and signed command authority remain explicit and do
not place tokens or signing keys in URLs, assets, screenshots, or Git.

The Runtime provides redacted public/operator/reviewer projections. Clients
render only fields allowed for their audience and never fetch raw private
citizen state, keys, or sealed checkpoints. Proof and packet links open as
independent resources and do not widen session authority.

HTML uses its existing `runtime-v3.config.json` and JavaScript transport. Unity
uses a native client adapter and a versioned contract resource derived from the
same Runtime schema. Neither client may invent fallback success: retained
packets are visibly historical/offline, while live mode requires current
Runtime correlation and freshness evidence.

## Owned Paths

- `demos/html-observatory/app.js`
- `demos/html-observatory/styles.css`
- `demos/v0.91.6/unity-observatory/Assets/Scripts/RuntimeV3Client.cs`
- `demos/v0.91.6/unity-observatory/Assets/Resources/runtime-v3-contract.json`
- `demos/v0.91.6/unity-observatory/Assets/Tests/RuntimeV3ClientTests.cs`
- `adl/tools/validate_v092_html_observatory_live.mjs`
- `adl/tools/validate_v092_unity_observatory_live.sh`
- `adl/tools/validate_v092_observatory_restart_reconnect.sh`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Invariants And Failure Semantics

- One versioned schema and event-ordering contract serves both clients.
- Runtime exposes no private state, signing key, certificate key, or raw token.
- Reads do not imply write authority; denied writes remain denied after
  reconnect or presentation-mode changes.
- TLS trust failure, CORS/origin refusal, API/WSS version mismatch, stale data,
  backpressure, unavailable service, and Runtime restart are explicit states.
- UI code never moves into Runtime and Unity has no schema fork.
- Fixture/static rendering tests cannot satisfy live integration acceptance.

## Dependencies And Coordination

WP-03 issue 5820 supplies stable launch/API behavior. WP-14 issue 5832 supplies
the versioned protocol and WSS contract. WP-18 supplies the first-birthday
interaction surface. All are hard gates for final integration. Issue 5800
supplies trusted local browser TLS. Shared Runtime files serialize with their
owners; consumer-only preparation can proceed independently.

## Validation Boundary

Deterministic lanes validate schema compatibility, redaction, auth state,
ordering, reconnect cursors, stale/unavailable rendering, and denied actions.
The browser lane runs the real HTML client against Runtime HTTPS/WSS and retains
visible interaction evidence. The native Unity lane runs batch contract checks
and live Editor/player interaction against the same Runtime revision. A restart
lane proves both clients reconnect without duplicated events or authority
escalation. macOS browser/Unity proof does not substitute for declared Windows
or Linux client coverage.
The implementation must add three focused live-proof entrypoints:
`adl/tools/validate_v092_html_observatory_live.mjs` drives the real browser
against Runtime HTTPS/WSS; `adl/tools/validate_v092_unity_observatory_live.sh`
launches the approved Unity Editor/player against that same revision and
captures native interaction evidence; and
`adl/tools/validate_v092_observatory_restart_reconnect.sh` coordinates a
Guardian-owned Runtime restart while both clients remain connected, then
verifies bounded replay, no duplicate application, unchanged authorization,
and fresh post-restart correlation in both clients. Retained v0.91.7 packets,
static screenshots, contract-only batch checks, and a Runtime-only soak cannot
satisfy these live lanes.

## Rollback

Rollback restores the prior client configuration/contract resource, logs out
write sessions, reconnects in read-only mode, and verifies Runtime remains
unchanged and healthy. It never embeds a fixture as live state or serves either
client from Runtime.

## Non-Goals

- Observatory redesign or unapproved Unity visual changes.
- Serving HTML/assets from Runtime.
- Runtime launch, protocol, or birthday implementation owned upstream.
- Client-side private-state access, signing, or authorization bypass.
- Unity-only Runtime schemas or provider/AWS work.
