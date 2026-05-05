# A2A External Agent Adapter

## Milestone Boundary

This feature defines how external A2A-discovered agents should be integrated
into ADL without bypassing the communication, identity, authority, and
execution-governance substrate.

It is not a parallel communication model. It is one governed external-agent
adapter layered on top of the broader Agent Communication and Invocation
Protocol work.

## Purpose

A2A provides connectivity-oriented surfaces such as discovery, identity claims,
and invocation shape. ADL needs to turn those claims into governed runtime
behavior.

The purpose of this feature is to:

- ingest and validate Agent Cards as claims rather than authority
- map external-agent identity into ADL trust and policy surfaces
- translate advertised capabilities into bounded ADL capability contracts
- force all external-agent invocation through explicit ADL invocation boundaries
- preserve trace, replay, redaction, and audit discipline

## Core Thesis

A2A by itself should not grant execution.

External agents may describe themselves and request interaction, but all actual
execution authority must remain under ADL governance:

- identity mapping
- trust classification
- capability translation
- policy evaluation
- invocation through `agent.invoke(...)`
- sandboxing
- trace and audit

## v0.91 Scope

The first tracked v0.91 planning boundary should establish:

- Agent Card ingestion and schema expectations
- identity-claim mapping into ADL identity and trust surfaces
- capability translation rules from A2A claims into ADL capability contracts
- trust classification (`Naked`, `Guest`, `Citizen`) as governance input
- mandatory `agent.invoke(...)` boundary rules
- trace, refusal, and failure-taxonomy expectations

## v0.91.1 Scope

The first implementation/hardening follow-on should land in `v0.91.1`:

- bounded runnable adapter slice
- policy-bound invocation path
- sandbox-profile wiring
- negative fixtures for invalid cards, bad signatures, policy denials, and
  capability mismatch
- additional conformance and hardening coverage

## Relationship To Agent Comms

The dependency direction is:

1. land the core Agent Communication and Invocation Protocol substrate
2. land durable policy artifacts such as structured planning and `SRP`
3. land A2A as a governed adapter over that substrate

This prevents A2A from inventing a second communication system beside the one
ADL is already planning.

## Non-Claims

This feature does not claim:

- open-network federation in `v0.91`
- cross-polis transport in `v0.91`
- TLS/mTLS-complete public transport in `v0.91`
- reputation or social-cognition semantics
- direct execution from external agent claims

It claims a narrower result:

ADL should have a planned governed path for external-agent interoperability
that treats A2A as connectivity and ADL as runtime governance.
