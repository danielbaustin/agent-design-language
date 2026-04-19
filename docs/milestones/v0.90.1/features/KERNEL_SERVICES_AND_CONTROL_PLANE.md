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

## Operator Controls

The operator should be able to:

- inspect manifold status
- inspect citizen status
- pause the manifold
- resume the manifold
- request snapshot
- terminate the manifold
- inspect last invariant/security failures

## Proof Surface

The control plane must emit a report showing:

- command requested
- pre-state
- post-state
- affected service
- trace event ref
- allowed/refused/deferred outcome

## Boundary

This is not autonomous release approval, governance voting, or social-contract
execution. Those remain later work.
