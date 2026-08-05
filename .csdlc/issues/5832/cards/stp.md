# Structured Task Prompt

Template: 1.0.0

Issue: 5832

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Reconcile existing ACIP/A2A semantics into one versioned envelope, protobuf schema, deterministic JSON projection, public catalog, compatibility policy, and authenticated bounded full-duplex WSS implementation with round-trip, denial, replay, malformed, and real-carrier proof.

## Deliverables

- Frozen versioned semantic envelope and compatibility/negotiation policy
- Canonical protobuf files plus schema-derived public message catalog
- Deterministic JSON projection and protobuf/JSON semantic round-trip fixtures
- Authenticated Rustls WSS bidirectional carrier with limits, backpressure, reconnect, error, replay, and denied-access proof

## Acceptance

1. One versioned ACIP/A2A semantic envelope defines identity, addressing, correlation/causation, trace/replay, capability/authority, payload, ordering, acknowledgement, error, and size rules
2. Canonical protobuf schemas and a schema-derived public catalog agree on messages, directions, payloads, versions, and auth requirements
3. Deterministic JSON projection round-trips with protobuf without semantic loss under explicit bytes, integer, omission, ordering, and unknown-field rules
4. Version negotiation accepts only declared compatible versions and rejects unsupported majors or required fields
5. Authenticated Rustls WSS performs real bounded full-duplex exchange with correlation, backpressure, reconnect, and typed errors
6. Missing/invalid auth, unsigned control, denied capability, malformed/oversized frame, replay, wrong runtime, and origin refusal fail before dispatch
7. Protocol and real-carrier proof runs on macOS, Linux, and native Windows or retains an explicit blocker
8. One exact-head review has no unresolved actionable findings

## Dependencies

- WP-04 gate issue 5821 terminal
- WP-04-IMP issue 5862 terminal after issues 5863 through 5878 integrate
- Current ACIP stream and trace/replay baselines requalified at the implementation revision
- Stable Runtime API/auth ownership before issues 5795 and 5837 integrate

## Inputs

- docs/milestones/v0.92/features/ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md
- docs/adr/0017-secure-local-agent-comms-and-a2a-boundary.md
- adl-runtime/src/acip.rs
- adl-runtime/src/runtime_api.rs
- adl-runtime/src/runtime_api_auth.rs
- adl-runtime/tests/runtime_api_wss.rs
- adl-runtime-kernel/src/acip.rs
- adl-runtime-kernel/src/protocol_adapters.rs
- adl-runtime-kernel/src/ingress.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/operations.rs

## Non Goals

- Distributed Guardian membership, placement, migration, or fencing
- Observatory/Unity UI implementation
- Shepherd local-model behavior
- SNS/SQS or other cloud bridge completion
- Custom cryptography, TLS, WebSocket, or protobuf runtime
