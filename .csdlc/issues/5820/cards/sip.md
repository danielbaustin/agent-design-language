# Structured Intent Prompt

Template: 1.0.0

Issue: 5820

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Consolidate Runtime v3 onto one Guardian-owned, init-file-driven launch path that starts, reports readiness, recovers bounded failures, preserves durable state, and shuts down cleanly across supported platforms.

## Required Outcome

One production Guardian process owns one Tokio/Axum/Rustls Runtime v3 kernel whose configuration, startup, readiness, bounded supervision, recovery, durable restart, observability, and shutdown behavior are reproducible and fail truthfully.

## Scope

- adl-runtime Guardian, supervision, shutdown, resident-agent, API, auth, TLS, and observability modules
- adl-runtime-kernel assembly, config, durable-state, supervisor, time, observability, and kernel entrypoint modules
- infra/runtime-v3/runtime-init.toml
- Focused Runtime launch, recovery, state, API, logging, and platform tests and tools
- .csdlc/evidence/5820

## Authority

- Issue 5820 owns single-node Guardian launch and Runtime resilience only
- Issue 5800 owns browser trust and serializes shared TLS/init edits
- Issue 5821 owns distributed membership, placement, migration, and fencing
- Issue 5832 owns ACIP/A2A schema reconciliation and issue 5837 owns consumers
- Optional network, provider, time, logging, certificate, or Observatory failures may degrade readiness but cannot gain process authority

## Assumptions

- none

## Operator Constraints

- Prepare before execution
- Never edit tracked work on main
- Use one bounded pre-PR review
- Do not substitute fixtures, receipts, or prose for required working behavior
