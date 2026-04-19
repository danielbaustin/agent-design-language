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
