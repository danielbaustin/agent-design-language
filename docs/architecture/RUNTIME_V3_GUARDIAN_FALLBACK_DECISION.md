# Runtime v3 Guardian Fallback Decision

Issue: #5224
Sprint: #5174
Decision date: 2026-07-12
cutover_authorized: false

## Decision

Do not replace the Runtime v3 guardian with any newly reviewed COTS candidate
as a drop-in dependency.

Horust remains the selected portable Unix guardian candidate, but adoption
remains blocked by the restart-budget defect recorded by #5211 and tracked for
rollout by #5221. The fallback review found useful libraries and design
evidence, but no reviewed candidate directly satisfies the ADL external
guardian contract without adding an ADL-owned OS-process supervision layer.

If the Horust release path remains blocked, #5225 is the next actionable path.
It tests two options against the same contract:

1. adopt or extend `rust-tokio-supervisor` by writing a narrow wrapper task
   whose supervised task owns the runtime kernel as an external OS process
2. otherwise build the smallest ADL-owned external guardian shim using Tokio
   process management and COTS crates for policy, telemetry, serialization,
   and configuration

That follow-on must stay outside Runtime v2 and must preserve the same
`adl-runtime-kernel serve <continuity-path>` child contract.

## Guardian Contract

The runtime kernel process remains the boundary:

- the guardian starts `adl-runtime-kernel serve <continuity-path>`
- the guardian owns environment injection, stdout/stderr capture, signal
  delivery, child reaping, restart delay, and process restart
- the child owns component supervision, typed channels, readiness, continuity,
  graceful shutdown, and serialization
- the control API remains on `127.0.0.1:20997`
- a guardian candidate is not acceptable if it only supervises in-process
  Rust tasks unless ADL explicitly wraps it with an OS-process guardian layer

## Candidate Review

| Candidate | Direct guardian fit | Useful pieces | Disposition |
|---|---|---|---|
| Horust 0.1.13 | Yes, but blocked by unbounded post-start crash loops | Portable Unix process supervision, signal forwarding, restart delay | Retain selected candidate; adoption blocked until fixed release qualifies |
| rustysd | No default fit; prior matrix records macOS build gaps and proof-candidate status | systemd-like unit model, Linux-oriented service-manager ideas | Retain Linux proof candidate only |
| systemd | Linux-only external guardian | cgroups, `DynamicUser`, resource bounds, native journal capture | Retain optional Linux containment adapter |
| rust_supervisor 0.2.0 | No; supervises thread factories, not the runtime kernel OS process | Erlang-style supervision concepts and restart policy vocabulary | Do not adopt as guardian |
| rust-tokio-supervisor 0.1.4 | Not directly; supervises Tokio task factories, so the OS child boundary needs a wrapper task | restart budgets, restart limits, task roles, health, control commands, graceful shutdown model | Try wrapper-task adoption in #5225; retain as component supervisor reference if that fails |
| launchd | macOS-only external guardian | native macOS process ownership and logs | Retain future platform adapter candidate |

## Source Evidence

- `docs/architecture/runtime_v3_guardian_matrix.v1.json` already records
  Horust, rustysd, and systemd bakeoff evidence from #5175.
- `docs/architecture/RUNTIME_V3_GUARDIAN_AND_SOAK.md` records the #5211
  qualification result and the Horust restart-budget blocker.
- `infra/rustysd/adl-runtime-kernel.service` is a compatibility fixture using
  a systemd-style `Restart=always` shape, not a production recommendation.
- `rust_supervisor` 0.2.0 is published at
  `https://crates.io/crates/rust_supervisor/0.2.0`, with documentation at
  `https://docs.rs/rust_supervisor/0.2.0` and repository metadata pointing to
  `https://github.com/roquess/rust_supervisor`. The reviewed crate source uses
  `ChildSpec` factories returning `thread::JoinHandle<()>` rather than
  external OS child processes.
- `rust-tokio-supervisor` 0.1.4 publishes the user-provided source at
  `developerworks/rust-supervisor` commit
  `66d6f691f5679d5253682473d50eb912ef003997`. Its child runner builds a task
  factory and runs it with `tokio::spawn(factory.build(ctx))`; the checked
  source does not provide a direct `tokio::process::Command` guardian for
  `adl-runtime-kernel`.

## Why Not Adopt rust-tokio-supervisor Directly

`rust-tokio-supervisor` is the strongest reviewed COTS design reference. It
has useful concepts for Runtime v3's component plane: restart budgets, typed
control commands, readiness, health state, restart limits, role defaults, and
graceful shutdown.

It is not a direct guardian because the ADL guardian must supervise an external
runtime kernel process. The crate's current child runner supervises async task
factories inside the same Rust runtime. Adopting it directly would either move
guardian responsibilities back inside the child process or require ADL to write
the missing process wrapper anyway.

That makes it a good reference for the component supervisor and a plausible
guardian dependency only through a wrapper task that owns the external process
boundary. It is not, by itself, the replacement for Horust or systemd at that
boundary.

## Follow-On Shape If Horust Remains Blocked

#5225 should first attempt the `rust-tokio-supervisor` adoption path.
The most promising shape is a wrapper task: `rust-tokio-supervisor` supervises
one guardian task, and that task owns a `tokio::process::Child` for
`adl-runtime-kernel serve <continuity-path>`. The wrapper translates child
exit, configuration exit, signal handling, readiness, output capture, and
restart-budget results into the crate's task-result model.

Adoption is acceptable only if the wrapper stays narrow and the actual runtime
kernel remains an external OS child process. If that requires a substantial
local fork or a broad adapter that is effectively a new service manager, stop
calling it adoption and build the small shim directly.

Either path should be constrained to the smallest surface that satisfies the
existing contract:

1. load a small declarative child spec
2. spawn `adl-runtime-kernel serve <continuity-path>` with `tokio::process`
3. capture stdout and stderr through the existing logging contract
4. forward `SIGINT` and `SIGTERM`
5. reap the child and fail closed on unsupported platform guarantees
6. enforce a bounded restart budget with jittered backoff
7. stop restarting after configuration exits and exhausted budgets
8. expose a tiny status surface for local qualification
9. use existing crates for serialization, tracing, sysinfo, and policy where
   they reduce code
10. keep Runtime v2 files untouched

This is intentionally smaller than a general service manager. Backpressure,
disk cleanup, cloud telemetry, component health, checkpointing, and adaptive
runtime behavior remain child/runtime component responsibilities unless a later
issue explicitly moves a narrow piece across the boundary.

## #5225 Adoption Result

#5225 re-checked `rust-tokio-supervisor` before implementation and did not add
it as a dependency. The crate remains a useful design reference for
task-supervision concepts, but adopting it for this issue would still require
ADL to write the external `tokio::process::Command` wrapper, output capture,
signal forwarding, child reaping, restart-budget classification, and
configuration-exit handling. That wrapper would be the actual guardian, while
the crate would supervise only the wrapper task inside the same process.

The implemented fallback therefore uses the smaller direct boundary:
`adl_runtime::guardian` owns one external child process with `tokio::process`,
keeps the canonical Runtime v3 control endpoint at `127.0.0.1:20997`, captures
stdout and stderr, forwards shutdown to the child, reaps it, and stops
restarting on configuration exits or exhausted restart budgets. This keeps the
fallback smaller than a service manager while preserving the child-process
contract for later cutover proof.

## Non-Claims

- This packet does not authorize Runtime v3 cutover.
- This packet does not switch the default runtime.
- This packet does not delete or modify Runtime v2.
- This packet does not claim Horust 0.1.13 enforces bounded restart attempts.
- This packet does not claim rustysd is cross-platform production ready.
- This packet does not claim `rust_supervisor` or `rust-tokio-supervisor`
  directly supervise external OS child processes.
- The original #5224 packet did not implement a new ADL-owned guardian; #5225
  adds the bounded `adl_runtime::guardian` fallback described above without
  authorizing default Runtime v3 cutover.
