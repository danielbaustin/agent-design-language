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

- adl-runtime and adl-runtime-kernel ACIP semantic envelopes and governed protocol adapters
- Runtime API/auth full-duplex Rustls WSS carrier and focused tests
- Versioned protobuf schema, deterministic JSON projection, public catalog, and compatibility fixtures
- Trace/replay identity, frame limits, negotiation, errors, reconnect, and backpressure contracts
- .csdlc/evidence/5832

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
