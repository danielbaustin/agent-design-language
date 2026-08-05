# Design: Simple issue preparation and recoverable binding

## Decision

C-SDLC v2 will separate issue publication, semantic preparation, execution
readiness, and resource binding. Preparation owns no execution claim. Binding
derives its owner and Git topology from the current issue and session context,
then acquires path authority at one repository-wide linearization point.

The user-visible states are:

1. `draft`: source intent exists, but no semantic readiness is claimed.
2. `prepared`: one complete immutable generation is current and editable by
   producing a successor generation.
3. `execution_ready`: the current generation has a valid digest-pinned
   readiness receipt and no claim.
4. `bound`: one owner holds the issue claim and matching branch/worktree.

`binding` is a durable transitional protocol state, not a normal resting state.
Doctor exposes it only with the exact recovery or release operation.

## Command Model

- `csdlc-issue create` publishes source intent as visibly `draft`. It creates no
  claim and cannot report readiness.
- `csdlc-prepare sync` resolves source intent and writes a staged immutable
  generation containing design, diagram, and all six typed cards. Placeholders
  are allowed, but the generation remains visibly non-ready.
- `csdlc-prepare seal` validates the current prepared generation and atomically
  publishes a readiness receipt. It rejects placeholders, invalid schemas,
  stale dependency evidence, missing owned paths, non-proving validation lanes,
  and semantic digest drift.
- `csdlc-prepare run` is only `sync` followed by `seal`. A failed seal preserves
  the complete synced generation for editing and reports one next operation.
- A substantive typed edit creates a successor prepared generation and
  invalidates the prior receipt. It does not require network sync or binding.
- `csdlc-bind run` consumes the current receipt, derives owner/claim/branch/
  worktree identity, rechecks volatile predicates, reserves paths atomically,
  writes binding intent, mutates Git, and commits `bound`.
- `csdlc-bind release` is idempotent owner cleanup for an unstarted binding. It
  serializes against bind, clears dangling intent, and removes only artifacts
  proven to have been created by that intent.

## Preparation Transaction

Each issue owns a generation namespace. A generation identifier is unique
within the issue and includes its content digest; sequential display numbers
are advisory only. Sync writes every generation artifact to a staging
directory, fsyncs complete content where supported, and advances the issue's
current-manifest pointer with compare-and-swap. A crash before pointer advance
leaves the prior generation current. A crash after pointer advance observes a
complete immutable generation.

Seal reads one expected manifest generation, resolves dependency versions, and
computes a semantic payload digest over:

- design and diagram digests;
- all six canonical typed card values and schemas;
- normalized owned paths and authority boundaries;
- dependency issue/revision vector;
- validation-plan semantics and acceptance mapping;
- repository identity and base revision constraints.

Comments, reactions, display-only timestamps, and unrelated label changes are
excluded. Tracker fields that alter issue intent, version, dependency routing,
or scope are semantic and must be included. Receipt publication uses
compare-and-swap against the same generation and dependency vector. Any later
substantive edit demotes the issue to `prepared` by making the old receipt
non-current; immutable historical receipts remain audit evidence.

## Binding Linearization And Recovery

The repository binding registry is the authority for path claims. Under one
repository-wide registry transaction, bind:

1. verifies the receipt and dependency vector are current;
2. rechecks root cleanliness, base revision, branch/worktree absence or exact
   recoverable identity, dependency gates, and normalized path overlap;
3. atomically reserves the issue's paths and records a unique binding intent;
4. releases the registry transaction before bounded Git mutation.

The intent records issue, derived owner identity, generation, receipt digest,
base revision, branch, worktree, reserved paths, operation nonce, and a ledger
of Git artifacts created by the operation. Owner recovery proves the same
session identity and exact intent fields through the existing session ledger;
it does not require the operator to copy or reconstruct a hidden claim ID.

Recovery is monotonic:

| Observed state | Exact owner retry | Release |
| --- | --- | --- |
| intent only | continue before Git mutation | clear reservation and intent |
| branch created | create or verify worktree | remove branch only if intent proves ownership |
| worktree created | verify topology and commit `bound` | remove worktree, then owned branch |
| `bound` committed, intent retained | remove completed intent | refuse ordinary release after implementation starts |
| conflicting or ambiguous artifact | fail closed with one typed repair operation | require audited operator repair |

Bind and release use the same registry serialization. Release first marks the
intent `releasing`, preventing bind progress, then compensates only proven
artifacts and finally clears reservations. An overlapping concurrent bind has
exactly one reservation winner; the loser performs no Git mutation.

## Batch Truth

Each child syncs and seals independently. The batch controller records a batch
ID and per-child outcome, checks dependency cycles and intra-batch path
overlaps, and refuses to call the umbrella ready while any child is failed,
blocked, stale, or mutually conflicting. Successful independent child receipts
are retained after partial failure; the batch is not an all-or-nothing
transaction and retry targets only non-ready children.

Intra-batch overlap is a preparation-time blocker for the batch and a warning
on each affected child. It does not create an early path reservation; a child
receipt remains point-in-time semantic proof, and bind still rechecks live
overlap against all repository claims.

## Legacy Migration

Migration classifies every record before mutation:

- initialized with no implementation and a preparation-only claim: preserve an
  immutable audit snapshot, release the legacy claim, and import as `prepared`;
- valid bound or implemented claim: retain unchanged;
- already terminal: retain unchanged;
- ambiguous identity, topology, or execution evidence: quarantine and fail
  closed with one typed `csdlc-migrate repair` action.

The repair action requires expected legacy digest, explicit disposition, and
operator reason. It can restore from the audit snapshot or tombstone stale
preparation artifacts, but cannot silently rewrite a valid active claim.
Migration is restartable per issue and records original plus resulting digests.

## Compatibility And Deletion

The new route ships beside the current commands until focused parity proves
create, prepare, seal, bind, release, migration, doctor, and batch behavior.
After parity, the coupled init/reserve/reacquire path is deprecated and deleted.
No shell wrapper, retry loop, alias that preserves hidden claim coordination,
or dependency on the active WP-01 readiness repair is allowed.

## Validation Strategy

Focused Rust tests must cover:

- draft truth and forged `Status: ready` rejection;
- placeholder sync followed by failed and successful seal;
- semantic versus non-semantic tracker drift;
- typed edit demotion and stale receipt rejection;
- dependency-vector drift and exact generation CAS;
- crash before and after manifest pointer replacement;
- concurrent seals for the same issue without ABA or generation reuse;
- ten-way overlapping bind with exactly one winner and zero loser Git changes;
- crash after intent, branch, worktree, and bound commit;
- release racing bind and release after each recoverable crash point;
- compensation that never deletes pre-existing Git artifacts;
- batch partial success, cycle detection, and intra-batch overlap truth;
- legacy migration interruption, replay, rollback, quarantine, and repair;
- doctor reporting exactly one truthful next operation for every state.

Cross-host mutation of one local repository is unsupported unless the binding
registry provides a proven shared-filesystem lock. Unsupported topology fails
closed before reservation rather than pretending local file locks are global.
