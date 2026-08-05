# Structured Intent Prompt

Template: 1.0.0

Issue: 5837

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Integrate the separate HTML Observatory and Unity Observatory with the same versioned Runtime v3 HTTP projection and authenticated full-duplex WSS contract while preserving client separation, redaction, explicit failure states, and Runtime authority.

## Required Outcome

Both real clients consume current Runtime snapshots/events, perform only authorized controls, reconnect after Guardian-owned restart, and distinguish live, stale, offline, denied, TLS, version, and backpressure states without fixture substitution or schema forks.

## Scope

- demos/html-observatory Runtime config, transport, status, control, and proof surfaces
- demos/v0.91.6/unity-observatory native client, contract resource, compatibility verifier, batch/live proof, and approved UI bindings
- Narrow Runtime projection/auth/schema compatibility changes only when upstream contracts require them
- Shared client compatibility matrix, redaction/refusal/reconnect evidence, and .csdlc/evidence/5837

## Authority

- Issue 5837 owns HTML and Unity consumer integration only
- Issue 5820 owns Runtime launch/API behavior, issue 5832 owns versioned protocol/WSS, issue 5800 owns browser trust, and WP-18 owns birthday behavior
- Runtime remains API-only and clients remain separate applications
- Clients never own private state, signing keys, provider launch, certificates, or Runtime authorization
- No unapproved UI redesign or Unity-only schema

## Assumptions

- none

## Operator Constraints

- Prepare before execution
- Never edit tracked work on main
- Use one bounded pre-PR review
- Do not substitute fixtures, receipts, or prose for required working behavior
