# Structured Task Prompt

Template: 1.0.0

Issue: 5837

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Bind the existing HTML and Unity Observatory clients to one stable Runtime v3 API/WSS compatibility contract, implement real read/write/reconnect behavior and explicit failure states, and prove redaction, refusal, restart recovery, and platform-visible interaction without moving UI into Runtime.

## Deliverables

- One shared compatibility matrix for API/schema version, endpoints, audience, IDs, ordering/correlation, reconnect cursor, auth, and backpressure
- Real HTML Observatory Runtime HTTPS/WSS reads, authenticated controls, failures, and browser evidence
- Real Unity native Runtime reads, authenticated controls, reconnect behavior, contract validation, and Editor/player evidence
- Redaction/refusal/stale/offline/TLS/version/restart evidence proving no fixture substitution or authority widening

## Acceptance

1. HTML and Unity use one compatibility matrix for API/schema version, endpoints, audiences, IDs, ordering/correlation, reconnect cursor, auth, and backpressure
2. The real HTML client reads Runtime snapshots/events and performs only authorized controls over trusted HTTPS/WSS
3. The real Unity client reads the same Runtime state and performs only authorized controls without a schema fork
4. Public/operator/reviewer redaction and denied write cases prevent private-state or authority exposure in both clients
5. TLS/origin failure, version mismatch, stale/offline data, unavailable Runtime, auth refusal, backpressure, and malformed events are explicit client states
6. After Guardian-owned Runtime restart both clients reconnect with bounded replay, no duplicate application, and no authority escalation
7. Browser plus native Unity evidence uses live Runtime paths; fixture/static rendering proof is classified separately and platform limits are explicit
8. One exact-head review has no unresolved actionable findings

## Dependencies

- WP-03 issue 5820 stable Runtime launch/API/readiness
- WP-14 issue 5832 stable versioned HTTP/WSS schema and auth contract
- WP-18 first-birthday interaction surface terminal
- Issue 5800 trusted browser HTTPS for HTML live proof
- Approved existing HTML and Unity designs remain unchanged unless separately authorized

## Inputs

- docs/milestones/v0.92/features/OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md
- demos/html-observatory/app.js
- demos/html-observatory/runtime-v3.config.json
- demos/html-observatory/README.md
- demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryShellController.cs
- demos/v0.91.6/unity-observatory/Assets/Resources/observatory_contract.json
- demos/v0.91.6/unity-observatory/Assets/Editor/UnityObservatoryCompatibilityVerifier.cs
- adl-runtime/src/runtime_api.rs
- adl-runtime/src/runtime_api_auth.rs
- docs/api/runtime-v3/v1/observatory.openapi.json

## Non Goals

- Observatory redesign or unapproved Unity visual changes
- Serving HTML/assets from Runtime or moving UI code into Runtime
- Runtime launch, protocol, TLS trust, or birthday implementation owned upstream
- Client-side private-state, signing-key, certificate-key, or authorization authority
- Unity-only Runtime schema, provider integration, or AWS work
