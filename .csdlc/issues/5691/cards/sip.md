# Structured Intent Prompt

Template: 1.0.0

Issue: 5691

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make production Runtime v3 emit through Rust tracing into one pinned Vector pipeline with durable master logging and Runtime v2 OTEL parity.

## Required Outcome

Runtime v3 has real Vector-owned durable logs, OTLP logs/traces/metrics export, status/API exposure, failure observability, and a clean-log auditor.

## Scope

- adl-runtime-kernel Runtime v3 observability implementation
- pinned Vector config and validation tests
- issue-local evidence under .csdlc/evidence/5691

## Authority

- Use existing Rust tracing, tracing-subscriber, Axum/Tokio, and pinned Vector; do not add a parallel logging framework.
- Do not touch the #5344 lifecycle harness path.
- All evidence and generated logs for this issue live under the issue worktree, never /private/tmp.

## Assumptions

- none

## Operator Constraints

- never write tracked changes on main
- never use /private/tmp
- use typed C-SDLC v2 lifecycle tools
- use repo-native GitHub tools and approved token resolver
- one bounded pre-PR review
