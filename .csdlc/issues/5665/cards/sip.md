# Structured Intent Prompt

Template: 1.0.0

Issue: 5665

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make the API-only Runtime v3 launch and separate HTML Observatory demonstrate the complete supported feature surface truthfully.

## Required Outcome

Authenticated bidirectional WSS through the existing Axum/Tokio/Rustls Runtime v3 API, Observatory health-state distinctions, sink-bounded telemetry, clean-checkout port 20997 init, and an end-to-end feature/adapter proof matrix with no unresolved claimed feature.

## Scope

- adl-runtime Runtime v3 API/auth/observability/shutdown surfaces
- real WSS handshake, authentication, bidirectional frames, rotation, revocation, and shutdown tests
- health distinction between unimplemented, unavailable, failed, and healthy
- telemetry fields limited to configured sink capabilities
- one issue-local init/config file on port 20997
- feature and adapter proof matrix

## Authority

- Runtime remains API-only
- HTML Observatory remains a separate client
- C-SDLC v2 Rust lifecycle is the only issue lifecycle authority
- #5657/#5663/#5664 protected paths remain disjoint
- No AWS

## Assumptions

- none

## Operator Constraints

- Use only typed C-SDLC v2 lifecycle and an issue-bound worktree
- Do not write implementation changes on main
- No URL-only, fixture-only, metadata-only, Python, or degraded proof
- Reuse COTS
- Remove obsolete wrappers or duplicate paths when found in scope
- Measure before/after physical LoC with net reduction
- Run focused tests, strict Clippy, and one exact pre-PR review
