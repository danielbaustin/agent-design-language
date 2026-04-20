# Kernel Services And Control Plane

## Purpose

Define the bounded Runtime v2 kernel service loop and operator controls.

## Required Services

- clock service
- identity/admission guard
- scheduler
- resource ledger
- trace writer
- snapshot manager
- invariant checker
- operator control interface

## WP-06 Implementation Surface

WP-06 adds the bounded kernel service loop contract to `adl/src/runtime_v2.rs`.

The contract defines:

- `RuntimeV2KernelServiceRegistry`
- `RuntimeV2KernelServiceRegistration`
- `RuntimeV2KernelServiceState`
- `RuntimeV2KernelServiceStatus`
- `RuntimeV2KernelLoopEvent`
- `RuntimeV2KernelLoopArtifacts`

The default prototype is available through `runtime_v2_kernel_loop_contract()`.
It consumes the WP-05 `RuntimeV2ManifoldRoot` refs and emits three reviewable
artifact families:

- `runtime_v2/kernel/service_registry.json`
- `runtime_v2/kernel/service_state.json`
- `runtime_v2/kernel/service_loop.jsonl`

The loop is intentionally one bounded tick across the required service set. It
proves service registration, deterministic activation order, monotonic loop
event sequencing, and service-state projection. It does not schedule live agent
work, materialize citizen records, seal snapshots, or execute operator controls.

## Validation Contract

The kernel loop contract validates:

- schema versions
- repository-relative artifact paths
- required service membership
- duplicate service rejection
- contiguous activation order
- contiguous monotonic event sequence
- registry/state/event manifold id consistency
- service-state completion through the final event sequence

The focused proof hook is:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2::tests::runtime_v2_kernel
```

## WP-10 Operator Control Surface

WP-10 adds a bounded operator control report contract to `adl/src/runtime_v2.rs`
and a CLI hook for reviewers:

```bash
adl runtime-v2 operator-controls --out .adl/state/runtime_v2_operator_control_report.v1.json
```

The report is intentionally deterministic and reviewable. It models the control
surface that a live Runtime v2 operator interface must preserve without claiming
that v0.90.1 already executes a long-lived runtime. The emitted artifact path is:

- `runtime_v2/operator/control_report.json`

The report covers these bounded commands:

- `inspect_manifold`
- `inspect_citizens`
- `pause_manifold`
- `resume_manifold`
- `request_snapshot`
- `inspect_last_failures`
- `terminate_manifold`

Each command records:

- command requested
- pre-state
- post-state
- affected service
- trace event ref
- allowed/refused/deferred outcome
- reason

The focused proof hook is:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2::tests::runtime_v2_operator
```

## Boundary

This is not autonomous release approval, governance voting, or social-contract
execution. These controls are a prototype operator report and CLI/demo hook, not
live kernel mutation or a persistent CSM control session.
