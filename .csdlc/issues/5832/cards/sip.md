# Structured Intent Prompt

Template: 1.0.0

Issue: 5832

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Define one versioned ACIP/A2A semantic family with canonical protobuf, deterministic JSON, a schema-derived public catalog, and authenticated bounded full-duplex Runtime v3 WSS transport.

## Required Outcome

Runtime v3 admits, transports, projects, traces, replays, and rejects the same ACIP/A2A message semantics consistently across protobuf and JSON, with explicit version negotiation, auth, limits, round trips, denied access, and real WSS exchange.

## Scope

- adl-runtime/src/acip.rs
- adl-runtime/src/runtime_api_auth.rs
- adl-runtime-kernel/src/acip.rs
- adl-runtime-kernel/src/protocol_adapters.rs
- adl-runtime/tests/runtime_api_wss.rs
- schemas/acip/v1/acip.proto
- schemas/acip/v1/catalog.json
- docs/api/runtime-v3/v1/acip.openapi.json
- adl/tools/validate_v092_acip_wss.sh
- adl/tools/validate_v092_acip_native_receipts.rb

## Authority

- Issue 5832 owns ACIP/A2A semantic, schema, projection, catalog, negotiation, and carrier contracts
- Issue 5821 owns distributed Guardian/polis authority and must land first
- Issue 5795 consumes stable command semantics and issue 5837 consumes stable client contracts
- Runtime retains authentication, signed command, capability, and dispatch authority
- No UI, Shepherd behavior, cloud bridge, custom crypto, or custom transport runtime authority

## Assumptions

- none

## Operator Constraints

- Prepare before execution
- Never edit tracked work on main
- Use one bounded pre-PR review
- Do not substitute fixtures, receipts, or prose for required working behavior
