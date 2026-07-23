# Structured Task Prompt

Template: 1.0.0

Issue: 5590

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare, validate, review, commit, and push the complete six-card design package now; do not touch Runtime product code, claim implementation authority, publish acceptance, or claim readiness until #5591 integration eligibility is confirmed.

## Deliverables

- complete six-card typed issue package
- source-grounded Parity-D design and current Runtime v3 diagram
- security and acceptance matrix covering positive and fail-closed behavior
- future disjoint implementation claim-scope proposal
- configured HTTPS HTTP/WebSocket access and live Observatory implementation proof
- guardian launch, pressure-stop, restart, soak, and operational selector transition with restored authenticated HTTPS service-health proof
- Vector telemetry route and degraded-collector proof
- exact-revision COTS, LoC, module-growth, test-count, lint, and evidence report

## Acceptance

1. AC-1: One deny-unknown-fields init file configures listener, HTTPS public base, TLS paths, allowed Observatory origins, agents, and weather; the guardian reads it and invalid or secret-bearing configuration fails before readiness
2. AC-2: Local and remote clients use one configuration-driven rustls Axum router with identical authentication and authorization; plain HTTP, missing or invalid credentials, authority escalation, and unlisted origins fail closed
3. AC-3: The HTML Observatory consumes live admitted-agent and Runtime state through authenticated HTTP and WebSocket routes, while missing bearer, bad origin, malformed or oversized frames, token-in-URL, and stale session cases fail closed
4. AC-4: Readiness, feed, and discovery expose the actual bound or configured listener and public HTTPS base, including ephemeral test ports, without hard-coded IPs or substitution of default port 20997
5. AC-5: The external guardian starts and reaps one canonical kernel child, forwards graceful signals, permits pressure serialization and checkpoint restore, bounds eligible crash restarts, and does not restart intentional stop or invalid configuration
6. AC-6: Redacted structured tracing events reach the checked-in Vector route while absent Vector degrades truthfully without disabling stderr, health, control, or shutdown; secrets, key material, unsafe errors, and host paths never emit
7. AC-7: An executable operational selector transitions the candidate Runtime v3 process/configuration to the previous approved Runtime v3 process/configuration and authenticated HTTPS health passes before and after; report-only selection, metadata assertions, environment echoes, in-memory facades, Runtime v2 edits, sidecars, cloud deployment, and AWS receive no credit
8. AC-8: Exact-revision positive and negative tests, strict lint, dependency inventory, source LoC, module growth, test count, and bounded soak satisfy #5336 without fixture-only, deferred, skipped, stale, or prose-only parity credit

## Dependencies

- #5361 prepared Runtime v3 acceptance umbrella
- #5336 architecture and budget authority
- #5591 clean-reviewed Parity-A contracts and confirmed integration eligibility before any Runtime product edit
- disjoint protected paths from #5592 Parity-B and #5589 Parity-C
- existing Runtime v3 control, configuration, guardian, Observatory, telemetry, and rollback source evidence
- exact preparation base 6d0f6115632a06619544b8ad4792792e741f1f31 and first reviewed preparation head 2f26da4455efd4dfc7ab6c65df5d19327fe765c8

## Inputs

- https://github.com/danielbaustin/agent-design-language/issues/5590
- https://github.com/danielbaustin/agent-design-language/issues/5594
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/telemetry.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/vector/runtime-v3.yaml
- infra/runtime-v3/runtime-init.toml
- demos/v0.91.7/html-observatory/app.js
- docs/architecture/RUNTIME_V3_CONTROL_OBSERVABILITY_ARCHITECTURE.md
- docs/architecture/RUNTIME_V3_GUARDIAN_AND_SOAK.md
- docs/architecture/RUNTIME_V3_SOAK_ROLLBACK_5253.md

## Non Goals

- Runtime product edits before #5591 integration eligibility is confirmed
- Runtime v2 source reuse, modification, deletion, defaulting, or decommission
- AWS execution, cloud deployment, credential use, or public endpoint claim
- plain HTTP Runtime access, hard-coded IP addresses, unauthenticated Observatory access, or token-bearing URLs
- in-process guardian sidecars, custom OpenTelemetry collection, or replacement of COTS Axum/rustls/Vector behavior
- Parity-B reasoning/learning or Parity-C operational adapter implementation
