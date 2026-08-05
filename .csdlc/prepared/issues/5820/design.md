# Issue 5820 Design: Runtime Launch And Resilience Consolidation

## Outcome And Boundary

Issue 5820 makes Guardian the sole supported process owner for one Runtime v3
kernel launch path. One versioned init file controls binaries, bind/TLS,
shutdown budgets, restart policy, bounded capture, services, state, and
observability. The Runtime must start, become ready, survive bounded failures,
checkpoint and restart durable state, and shut down cleanly without depending
on network time, providers, certificates, logging sinks, or Observatory
availability for process survival.

This issue owns single-node launch and resilience. It does not own browser trust
(5800), distributed placement or relocation (5821), protocol redesign (5832),
or either Observatory/Unity consumer (5837).

## Source Baseline

- `adl-runtime/src/bin/adl-runtime-guardian.rs` already loads an absolute init
  path and derives bounded restart, backoff, capture, and shutdown budgets.
- `adl-runtime/src/guardian.rs`, `shutdown.rs`, `supervision.rs`, and
  `resident_agent.rs` own process lifecycle and bounded task supervision.
- `adl-runtime/src/runtime_api.rs`, `runtime_api_auth.rs`, and `local_tls.rs`
  own the Axum/Rustls API, authorization, and local TLS bootstrap.
- `adl-runtime-kernel/src/bin/adl-runtime-kernel.rs`, `assembly.rs`,
  `config.rs`, `durable_state.rs`, `supervisor.rs`, `observability.rs`, and
  `time.rs` own kernel construction, configuration, state, supervision,
  logging, and trusted-time behavior.
- `infra/runtime-v3/runtime-init.toml` and the Runtime launch/resilience feature
  contract are the canonical configuration and milestone inputs.

## Design

The only production entry is Guardian with `--init <absolute-path>`. Guardian
validates the complete init contract before spawn, launches one kernel, owns
signals and child reaping, applies bounded exponential backoff, distinguishes
configuration exits from recoverable failures, and reports a typed terminal
state. The kernel constructs bounded Tokio tasks and the Axum/Rustls API from
the same init file; source defaults cannot silently override declared values.

Readiness is stricter than liveness. The process may remain alive while an
optional network, time, provider, Vector, certificate, or Observatory
dependency is degraded, but readiness and telemetry must name the degraded
reason. A required production adapter that cannot execute real work fails
before readiness. Shutdown drains the API, checkpoints state, cancels bounded
tasks, and returns control to Guardian within the declared budget.

Candidate implementation ownership is limited to `adl-runtime/`,
`adl-runtime-kernel/`, `infra/runtime-v3/runtime-init.toml`, focused Runtime
tools/tests, and issue evidence. The execution session must claim narrower
files after checking 5800 and active Runtime worktrees.

## Invariants And Failure Semantics

- Guardian remains process 0 and reaps every child; no shell/Python supervisor.
- One init file is authoritative; malformed, incomplete, relative, or unsafe
  paths fail before child spawn.
- Queues, concurrency, retries, capture, and shutdown waits remain bounded.
- Runtime state is transactional and restart-durable, never receipt- or
  temporary-directory authority.
- Network/SNTP/provider/log sink/Observatory failure cannot crash or deadlock
  the kernel; degraded readiness remains truthful.
- TLS and authentication do not downgrade to plaintext or unsigned mutation.
- Machine-readable output stays on stdout and human `adl_event` output on
  stderr according to repository policy.

## Dependencies And Coordination

WP-02A is a hard gate. Issue 5800 supplies the trusted local certificate flow;
shared init/TLS edits serialize with it. WP-04 consumes stable ingress and
lifecycle contracts only after this issue lands. WP-14 and WP-18A are downstream
consumers and cannot broaden this issue into protocol or UI work.

## Validation Boundary

Deterministic lanes prove init parsing, Guardian restart/backoff and signal
handling, bounded capture, readiness/degradation, API drain, state restart, and
clean logs. Integration lanes launch the actual Guardian/kernel, exercise
authenticated HTTPS/WSS, kill/restart the child, and compare retained state.
Platform lanes run start-stop-recovery on macOS, Linux, and native Windows;
unsupported platform behavior is a blocker, not a portable-success inference.

## Rollback

Rollback restores the prior init schema/defaults and previous Guardian/kernel
binaries, restarts from an unchanged durable-state snapshot, verifies health
and shutdown, and records any data-format incompatibility. It never switches
to Runtime v2, plaintext, a shell supervisor, or fixture state.

## Non-Goals

- Distributed Guardian mesh, placement, migration, or fencing.
- Observatory HTML serving, UI redesign, or Unity integration.
- ACIP/A2A schema changes beyond consuming the current API contract.
- Release-scale soak, custom allocator, lock-free rewrite, or Wasm work.
- Claims that optional integrations are healthy without direct evidence.
