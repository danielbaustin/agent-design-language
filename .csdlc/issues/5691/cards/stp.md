# Structured Task Prompt

Template: 1.0.0

Issue: 5691

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement Runtime v3 observability parity only; no observatory HTML/UI, no legacy deletion, no provider behavior changes.

## Deliverables

- Runtime v3 observability module
- production startup wiring
- Vector config
- status/API exposure
- clean-log auditor
- focused tests and retained evidence
- ready PR

## Acceptance

1. AC-1: Production Runtime v3 installs tracing once and emits structured events into Vector without silent subscriber absence.
2. AC-2: Vector is the only general log writer/router; no custom logging facade, custom rotation, Python wrapper, or duplicate master JSONL writer is introduced.
3. AC-3: A canonical append-only master log is produced under an explicit absolute state root with Vector-owned retention and buffering.
4. AC-4: Errors carry runtime, guardian, process, lifecycle, component, operation, reason, error chain, revision, trace, span, and parent context.
5. AC-5: Secrets are redacted before durable and remote output, including adversarial regression coverage.
6. AC-6: Normal shutdown drains; Vector startup/death/restart/export failure is observable and fails closed where appropriate.
7. AC-7: Runtime v2 OTEL parity exists for OTLP/HTTP endpoint and timeout config, service resource attrs, trace/span/parent propagation, logs/traces/metrics, status/API exposure, service env propagation, and durable export failures.
8. AC-8: Vector exports real OTLP logs, traces, and metrics with bounded batching, retry/backoff, disk buffering, endpoint health checks, and acknowledged local durable output.
9. AC-9: Clean-log auditor fails on malformed records, sequence gaps, correlation mismatch, unexplained restart, panic/error/degraded/unavailable markers, and incomplete drain.
10. AC-10: WP-12 can consume the Vector master log for lifecycle acceptance without a custom harness log writer.

## Dependencies

- repo-pinned .adl/bin/vector
- Runtime v2 observability implementation
- Runtime v3 Axum/Tokio control API
- tracing and tracing-subscriber crates

## Inputs

- GitHub issue #5691 body
- adl-runtime/src/observability.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/telemetry.rs
- adl-runtime-kernel/vector/runtime-v3.yaml

## Non Goals

- observatory HTML/UI work
- Python wrappers or servers
- custom log rotation
- duplicate general JSONL master writer
- fixture-only or degraded acceptance
- legacy runtime deletion
