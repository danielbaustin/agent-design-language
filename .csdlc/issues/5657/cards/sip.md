# Structured Intent Prompt

Template: 1.0.0

Issue: 5657

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make Runtime v3 start reliably through one Guardian-owned Rust launch path with coherent config, Observatory, and readiness behavior.

## Required Outcome

A clean checkout starts a real Runtime v3 kernel under Guardian, exposes coherent health and Observatory routes, performs an authenticated WebSocket exchange, and fails closed before readiness when required production adapters are unavailable.

## Scope

- Guardian-owned Runtime v3 serve path
- Runtime init endpoint/TLS/origin configuration
- Axum health and Observatory routes
- real production adapter readiness
- continuity identity projection
- authenticated WebSocket launch proof
- focused fast launch validation

## Authority

- Guardian is process 0 and owns child lifecycle
- Tokio/Axum/Rustls are the only local serving path
- runtime readiness is not claimed until required adapters are real
- Runtime v2 remains untouched

## Assumptions

- none

## Operator Constraints

- Rust/Tokio/Axum/Rustls only; no Python, shell lifecycle, sidecars, or fixture credit
- No plaintext secrets or private-key material in tracked artifacts
- TLS remains mandatory and fail-closed
- Keep root main clean and work only in the bound worktree
- Do not widen into distributed mesh or AWS
