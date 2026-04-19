# Manifold And Snapshot Contract

## Purpose

Define the persistent world root and the snapshot/rehydration evidence needed
for Runtime v2 foundation proof.

## Manifold Minimum

The manifold record should include:

- manifold id
- schema version
- lifecycle state
- clock anchor
- citizen registry refs
- kernel service refs
- trace root
- snapshot root
- invariant policy refs

## WP-05 Implementation Surface

WP-05 introduces the Rust-owned `runtime_v2.manifold.v1` contract in
`adl/src/runtime_v2.rs`.

The contract defines:

- `RuntimeV2ManifoldRoot`
- `ManifoldClockAnchor`
- `CitizenRegistryRefs`
- `KernelServiceRefs`
- `TraceRootRef`
- `SnapshotRootRef`
- `ResourceLedgerRef`
- `InvariantPolicyRefs`
- `RuntimeV2ManifoldReviewSurface`

The default prototype root is available through
`runtime_v2_manifold_contract()` and serializes to the reviewable artifact path
`runtime_v2/manifold.json`.

WP-05 deliberately records references to later artifact families without
materializing them. WP-06 owns live kernel loop behavior, WP-07 owns citizen
record materialization, WP-08 owns snapshot writing and rehydration, and WP-09
owns invariant violation artifacts.

## Validation Contract

The manifest contract validates:

- schema version
- non-empty stable manifold id
- lifecycle state
- repository-relative artifact paths
- positive next trace event sequence
- invariant enforcement mode
- non-empty blocking invariant set
- explicit downstream WP boundaries

The focused proof hook is:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2::tests
```

## Snapshot Minimum

The snapshot should include enough information to validate wake:

- manifold state
- citizen lifecycle state
- resource ledger state
- last trace cursor
- invariant status
- snapshot hash or structural checksum

## Wake / Rehydration Proof

Rehydration must prove:

- no duplicate active citizen instance
- restored manifold id matches snapshot
- trace continues after restore
- invariant checks run before active state resumes

## Non-Goals

- cross-machine migration
- full cross-polis state transfer
- rich narrative memory restoration
