# ADL Runtime Kernel Proof

Status: independent implementation proof for issue #5170. This is not the
current production runtime and does not authorize migration of existing runtime
components.

## Implemented Boundary

`adl-runtime-kernel` is a new standalone crate. It does not import or modify
`adl-runtime` or `adl/src/runtime_v2`.

The proof has two supervision layers:

```text
rustysd, systemd, launchd adapter, or container supervisor
  -> adl-runtime-kernel process
     -> Tokio kernel supervisor
        -> observability
        -> Chronosense clock authority
        -> scheduler
        -> governed gate
        -> checkpoint/evidence sink
```

The external guardian owns process creation, environment, stdout/stderr
capture, exit classification, restart delay, and process restart. The kernel
owns component topology, readiness, bounded channels, restart/degrade/fatal
policy, coherent runtime state, continuity serialization, and bounded shutdown.

An exhausted critical-component policy terminates the child process. It does
not attempt unlimited recovery inside a process whose invariants may already be
damaged.

## Component Contract

Every component is constructed by a restartable `ComponentFactory`. The
factory provides a stable `ComponentSpec` containing:

- component identity;
- dependencies;
- typed input and output port declarations;
- failure policy.

The kernel validates duplicate identities, missing dependencies, and cycles
before constructing any component. It then starts components in dependency
order and waits for explicit readiness before starting dependents.

Each component instance receives a `ComponentContext` with its identity, a
component-local `CancellationToken`, the shared runtime recorder, and a
single-use readiness signal. Components do not own the Tokio executor or the
global lifecycle policy.

## Bootstrap Logging And Time

Logging and wall-clock synchronization are components, but the kernel needs
minimum observability and ordering before those components are ready. The proof
uses a deliberately small bootstrap substrate:

- lifecycle events are sequenced from process start with monotonic elapsed
  time;
- events are emitted to stderr for guardian capture;
- a bounded in-memory ring retains startup events;
- the observability component promotes and flushes that ring exactly once;
- Chronosense begins in explicit `degraded` wall-clock authority, reports ready,
  and later promotes authority without rewriting prior monotonic ordering.

The proof clock adapter uses local `SystemTime` after a bounded simulated sync
delay. It does not claim live SNTP traffic or ntpd-rs integration. Existing
Chronosense/ntpd-rs work remains owned by its current runtime issues.

## Vector Lessons

The design was checked against the local Vector source tree, especially:

- `src/topology/builder.rs`;
- `src/topology/running.rs`;
- `src/topology/task.rs`;
- `lib/vector-common/src/shutdown.rs`;
- `lib/vector-core/src/fanout.rs`;
- `lib/vector-buffers/src/topology/`.

The proof adopts Vector's separation of topology construction from running
topology, named task ownership, explicit wiring, bounded buffering, readiness
checks, and graceful-then-forced shutdown. It does not adopt Vector's
source/transform/sink taxonomy or dynamic reload machinery.

Vector's application loop converts unrecoverable topology failure into an
orderly process shutdown. Its packaged systemd service uses `Restart=always`.
That process boundary is the model used here.

## rustysd Compatibility

The fixture in `infra/rustysd/adl-runtime-kernel.service` uses the common unit
subset understood by rustysd and systemd: `ExecStartPre`, `ExecStart`, and
`Restart=always`. rustysd does not implement `RestartSec`; the proof uses a
supported `ExecStartPre=/bin/sleep 2` fixed delay. Richer exponential backoff
remains a guardian-specific deployment policy rather than a kernel feature.

The rustysd source reviewed for this issue was commit
`b200759c7c55026eb1fae7a095464af0c6b6d699` from 2025-06-27. Its own README
describes the project as proof-of-concept/work-in-progress and warns against
important production use. Therefore:

- rustysd is the initial compatibility and local guardian target;
- it is not linked into the kernel;
- production deployment may use systemd or a container supervisor with the
  same child-process contract;
- production adoption of rustysd requires a separate hardening decision and
  evidence.

The `fatal-once` binary mode proves the guardian-facing contract: the first
generation emits a classified fatal exit code, and a fresh invocation restores
normal operation and writes a validated continuity capsule. The test invokes
the child twice; it does not claim a live rustysd daemon was installed on the
test host.

### Alternatives Checked

`initd` 0.1.3 was also evaluated. It is a Linux PID 1 implementation whose
stated role is minimal userspace initialization followed by delegation to a
separate `serviced`-compatible manager. Its public documentation says it is in
development, and the referenced `serviced` implementation/specification is not
public/stable. It therefore does not replace the guardian layer in this proof:
ADL needs child supervision and restart policy, not ownership of Linux PID 1.
The reviewed `main.rs` (commit
`2d0151cdf1f24033573fdecf98fbd239233efad4`) does reinforce one applicable
pattern: one outer event loop owns signal handling and child reaping, and loss
of the critical delegated service manager becomes a machine-level recovery
decision. ADL adopts the narrower analogue by making `serve` wait on either an
OS shutdown signal or kernel termination. PID 1 setup, global zombie reaping,
filesystem sync, power-off, and reboot authority remain out of scope.

Horust is closer to the required container-supervisor role and is a reasonable
future comparison candidate, but changing guardian targets is outside this
bounded implementation proof.

## COTS Inventory

| Concern | Selected crate | Local responsibility |
| --- | --- | --- |
| Executor, tasks, bounded queues, deadlines | Tokio | lifecycle policy and component contracts |
| Hierarchical cancellation | tokio-util `CancellationToken` | dependency-ordered cancellation decisions |
| Panic capture around component futures | futures | typed completion classification |
| DAG validation and topological ordering | petgraph | ADL component specifications |
| Async component interface | async-trait | bounded component/factory contracts |
| Errors | thiserror | ADL-specific error taxonomy |
| Records and continuity | serde / serde_json / BLAKE3 | schemas, corruption detection, atomic replacement |
| Instrumentation facade | tracing | bootstrap stderr and retained-event policy |

The continuity digest detects accidental corruption; it is not a signature and
does not authenticate a capsule against a malicious writer.

The crate does not implement an executor, graph algorithm, SNTP protocol,
telemetry SDK, durable queue, or service manager.

## Proof Surface

The focused suite classifies as the PVF `runtime` lane with proof role
`runtime_regression`, deterministic local execution, normal resource profile,
and no release-gate claim.

```sh
cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --all -- --check
cargo check --manifest-path adl-runtime-kernel/Cargo.toml --all-targets
cargo test --manifest-path adl-runtime-kernel/Cargo.toml
cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings
cargo run --manifest-path adl-runtime-kernel/Cargo.toml --bin adl-runtime-kernel -- demo /tmp/adl-runtime-kernel-continuity.json
```

The tests prove bounded-channel rejection, graph validation before startup,
component restart, deadline abort, startup event promotion, qualified clock
authority, typed three-component flow, continuity generations, the unit restart
contract, and fatal-child recovery across fresh process invocations.

## Non-Claims

This proof does not establish:

- migration or replacement of the current ADL runtime;
- production rustysd suitability;
- live SNTP, ntpd-rs, OpenTelemetry, cloud, ACIP, or provider integration;
- dynamic topology reload or distributed supervision;
- durable queue or production persistence semantics;
- full Freedom Gate, reasoning runtime, AEE, or adaptive-learning behavior;
- release readiness for v0.92.

Related issues #5111 through #5120 retain ownership of their existing runtime
acceptance surfaces.
