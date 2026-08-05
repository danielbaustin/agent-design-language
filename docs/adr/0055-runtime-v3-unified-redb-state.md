# ADR 0055: Runtime v3 Unified redb State

- Status: Accepted
- Date: 2026-07-30
- Accepted in: v0.91.8
- Related issues: #5663, #5698
- Related ADRs: ADR 0011, ADR 0013, ADR 0053, ADR 0054
- Source evidence:
  - `adl-runtime-kernel/Cargo.toml`
  - `.csdlc/issues/5698/`
  - merge commit `892c87bb9`

## Context

Runtime v3 adapters previously used separate files and ad hoc persistence
surfaces. Independent writers, partial updates, and divergent recovery rules
made consistent checkpoints and restart behavior harder to guarantee.

## Decision

Runtime v3 uses redb for the migrated local checkpoint, lifelog, and
governed-operation namespaces.

Those migrated namespaces share the configured state root and database
transaction boundary. State identity, schema version, principal ownership,
integrity metadata, replay position, and migration behavior are explicit.
Callers do not silently fall back to cwd- or temporary-directory state.

Runtime v3 still has file-backed durable surfaces outside redb. These include
signed continuity generation directories, signed continuity binary snapshots,
signed continuity `manifest.json`, and observability spool, audit, master-log,
configuration, and sequence files. A backup of `runtime-kernel.redb` captures
only the redb-managed namespaces; it does not capture all Runtime v3 state and
must be paired with the remaining file-backed durable surfaces for whole-state
recovery.

The kernel remains responsible for serialization and policy; redb supplies the
embedded transactional storage mechanism, not business authority.

## Consequences

- Atomic transactions replace multi-file partial-update windows for migrated
  redb namespaces.
- Restart and checkpoint restoration use one storage boundary for migrated
  redb namespaces.
- Schema migrations and backup/restore require explicit versioned handling.
- A single writer database simplifies consistency but makes transaction scope
  and long-running read/write behavior important performance concerns.
- Whole-state Runtime v3 backup and restore still require explicit handling for
  file-backed durable surfaces outside `runtime-kernel.redb`.

## Alternatives Considered

### Continue per-adapter files

Rejected. File-local atomicity does not provide a unified kernel transaction.

### Introduce a network database

Rejected for the local Runtime v3 baseline. It would add an availability and
deployment dependency without improving single-node ownership.

## Validation Notes

Validate atomic commit/rollback, process restart, corruption rejection,
identity and integrity checks, concurrent access behavior, migration failure,
redb-scoped backup/restore, file-backed durable-surface recovery, and configured
absolute state-root handling.

## Non-Claims

- This ADR does not define distributed replication or consensus.
- This ADR does not make redb records a public wire protocol.
- This ADR does not claim `runtime-kernel.redb` backup captures all Runtime v3
  durable state.
