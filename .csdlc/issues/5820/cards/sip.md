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

- adl-runtime/src/bin/adl-runtime-guardian.rs
- adl-runtime/src/guardian.rs
- adl-runtime/src/shutdown.rs
- adl-runtime/src/supervision.rs
- adl-runtime/src/resident_agent.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/durable_state.rs
- adl-runtime-kernel/src/supervisor.rs
- infra/runtime-v3/runtime-init.toml
- adl-runtime/tests/runtime_guardian_lifecycle.rs
- adl/tools/validate_v092_runtime_guardian_lifecycle.sh
- adl/tools/validate_v092_runtime_native_receipts.rb

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
