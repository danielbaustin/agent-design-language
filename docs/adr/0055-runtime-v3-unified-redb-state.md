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

Runtime v3 durable kernel state uses redb as the single embedded transactional
store.

All durable adapter namespaces share the configured state root and database
transaction boundary. State identity, schema version, principal ownership,
integrity metadata, replay position, and migration behavior are explicit.
Callers do not silently fall back to cwd- or temporary-directory state.

The kernel remains responsible for serialization and policy; redb supplies the
embedded transactional storage mechanism, not business authority.

## Consequences

- Atomic transactions replace multi-file partial-update windows.
- Restart and checkpoint restoration use one storage boundary.
- Schema migrations and backup/restore require explicit versioned handling.
- A single writer database simplifies consistency but makes transaction scope
  and long-running read/write behavior important performance concerns.

## Alternatives Considered

### Continue per-adapter files

Rejected. File-local atomicity does not provide a unified kernel transaction.

### Introduce a network database

Rejected for the local Runtime v3 baseline. It would add an availability and
deployment dependency without improving single-node ownership.

## Validation Notes

Validate atomic commit/rollback, process restart, corruption rejection,
identity and integrity checks, concurrent access behavior, migration failure,
backup/restore, and configured absolute state-root handling.

## Non-Claims

- This ADR does not define distributed replication or consensus.
- This ADR does not make redb records a public wire protocol.

