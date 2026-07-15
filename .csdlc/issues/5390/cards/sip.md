# Structured Intent Prompt

Template: 1.0.0

Issue: 5390

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Repair the integrated Runtime v3 local control and Observatory path without a gateway or hard-coded port.

## Required Outcome

Runtime v3 serves its control API through native TLS and all discovery data reflects the actual bound socket.

## Scope

- Runtime v3 TLS init configuration
- Native Rustls control serving
- Bound-address propagation
- HTML Observatory local HTTPS documentation and validation

## Authority

- No plain-HTTP production listener
- No external TLS gateway or sidecar
- No committed private key or automatic trust-store mutation
- Runtime v3 remains explicit opt-in and Runtime v2 remains default

## Assumptions

- none

## Operator Constraints

- none
