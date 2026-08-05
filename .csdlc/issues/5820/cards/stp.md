# Structured Task Prompt

Template: 1.0.0

Issue: 5820

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver one Guardian process-0 launch path and one authoritative init contract for Runtime v3, with bounded supervision, truthful liveness/readiness, durable restart, clean shutdown, failure recovery, and macOS/Linux/native Windows proof.

## Deliverables

- One absolute-init Guardian entrypoint with complete validation and typed terminal states
- One kernel assembly path with bounded tasks, Axum/Rustls API, transactional state, and explicit blocking boundaries
- Truthful readiness and tracing for startup, dependency degradation, recovery, and shutdown
- Real start-stop, child-failure restart, durable-state recovery, authenticated API/WSS, clean-log, and platform evidence

## Acceptance

1. Guardian is the sole production process owner and launches exactly one Runtime v3 kernel from one validated absolute init file
2. The init contract controls binaries, bind/TLS, state, services, limits, restart, capture, shutdown, and observability without hidden source overrides
3. Bounded supervision, backoff, cancellation, API drain, checkpoint, child reaping, and typed terminal states behave deterministically
4. Runtime durable state survives real Guardian-owned restart without receipt or temporary-directory authority
5. Network/SNTP/provider/log sink/certificate/Observatory failures preserve process liveness and report truthful degraded readiness or startup failure
6. Authenticated HTTPS/WSS, health/readiness, startup, recovery, shutdown, and clean-log evidence use production paths
7. Start-stop-recovery behavior is proven on macOS, Linux, and native Windows or a named platform remains blocked with evidence
8. One exact-head pre-PR review has no unresolved actionable findings and the PR closes issue 5820

## Dependencies

- WP-02A issue and proving substrate terminal before execution
- Issue 5800 trusted local TLS contract for browser-facing proof
- Sprint 5855 ownership coordination
- No overlapping active Runtime worktree on claimed implementation paths

## Inputs

- docs/milestones/v0.92/features/RUNTIME_LAUNCH_AND_RESILIENCE_v0.92.md
- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
- adl-runtime/src/bin/adl-runtime-guardian.rs
- adl-runtime/src/guardian.rs
- adl-runtime/src/shutdown.rs
- adl-runtime/src/runtime_api.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/durable_state.rs
- infra/runtime-v3/runtime-init.toml

## Non Goals

- Distributed Guardian mesh, placement, migration, or fencing
- Observatory HTML serving, UI redesign, or Unity integration
- ACIP/A2A schema redesign
- Runtime v2 fallback, Python server, shell supervisor, or plaintext API
- Release-scale soak, allocator, lock-free, Wasm, or unrelated optimization work
