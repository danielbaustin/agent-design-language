# WP-10A Task And Context-Handoff Adapter Preparation Design

## Status

Preparation only. Issue #5498 must not implement product code until conductor
#5499 and final WP-09 interface gate #5349 are live-merged into `origin/main`,
their dependency revisions are ancestors of the execution base, and adjacent
path owners confirm disjoint reservations. Retained receipts and typed closeout
records are audit evidence only; they must not block execution readiness when
live merge and ancestry truth is satisfied.

## Purpose

The adapter executes explicit conductor-approved Codex task operations. It can
create or attach, send a bounded message or handoff, inspect, cancel, and
escalate. It cannot invent assignments, widen scope, mutate lifecycle state, or
treat a private task transcript as canonical project truth.

## Contract Boundary

Each versioned request envelope contains:

- operation kind and idempotency key;
- issue, live claim, branch, worktree, and normalized protected-path identity;
- conductor assignment digest and exact dependency snapshot;
- bounded context provenance, scope, expected output, validation contract, and
  freshness token;
- deadline, cancellation policy, and caller authority.

Each result is one of:

- a typed task reference and sanitized operation receipt;
- a bounded read-only observation for #5500;
- a typed handoff/output reference for #5502;
- a machine-readable refusal or transport failure.

Retained records may include task identifiers, operation kinds, digests,
timestamps supplied by the transport, status classes, and evidence references.
They must not include credentials, provider secrets, private transcript bodies,
or unrelated context.

## Authority And Ownership

- C-SDLC v2 owns issue claims and lifecycle transitions.
- #5499 owns assignment planning and admission.
- #5498 owns task transport only.
- #4760 owns durable Memory Palace context handoff at WP-14; this adapter passes
  bounded references and does not duplicate that store.
- #5500 owns read-only dashboard presentation.
- #5502 owns output convergence and deterministic replanning.

No adapter operation can grant review, publication, merge, closeout, issue
creation, or scope-widening authority. Cancellation and escalation return typed
results to the authorized caller; they do not mutate C-SDLC state directly.

## Planned Product Boundary

After the gates open, implementation is isolated under
`adl-v2/crates/adl-workcell-task-adapter/`. The normalized planning inventory
reserves `adl-v2/crates/adl-workcell-conductor/` for #5499,
`docs/tooling/milestone-dashboard/` for #5500, and
`adl-v2/crates/adl-workcell-convergence/` for #5502. The preparation validator
proves these four sets are pairwise disjoint. Because #5502 has not yet bound
its product claim, its reservation is a fail-closed planning constraint rather
than authority over #5502: implementation cannot amend the #5498 claim until
the adjacent owners confirm non-overlapping normalized sets. Any
workspace-manifest edit also requires a later typed claim amendment.

The public surface should remain small:

```text
execute(TaskOperation) -> Future<Result<TaskReceipt, TaskTransportError>>
observe(TaskRef) -> Future<Result<TaskObservation, TaskTransportError>>
```

The transport implementation is behind a trait so deterministic fixtures can
prove behavior without live Codex sessions or network access.

## COTS Strategy

- `serde` and `serde_json` for versioned request, result, and fixture records.
- `tokio` for bounded asynchronous execution, deadlines, and cancellation.
- `futures` for shared future/stream combinators instead of custom polling.
- `blake3` for content-derived idempotency and context digests.
- `thiserror` for explicit refusal and transport-error classes.
- `secrecy` for secret-bearing inputs if a future transport requires them;
  values remain non-serializable and non-debuggable.

Exact versions must use the accepted ADL v2 workspace dependency set after
#5349 freezes the interface. No paid service, new database, custom executor,
custom cryptography, or transcript store is planned.

## Idempotency, Privacy, And Security

- Canonicalize and hash the complete authority-bearing request before dispatch.
- Repeating an identical idempotency key returns the prior typed outcome;
  reusing it with different content fails closed.
- Revalidate claim owner, generation, branch, worktree, paths, dependency
  digest, and freshness immediately before every writable operation.
- Reject absolute paths, parent traversal, empty paths, aliases, and protected
  path overlap with another live claim.
- Bound context by declared fields and byte limits; never serialize arbitrary
  task transcript history into retained evidence.
- Redact transport errors before retention and test that secret-like values do
  not appear in Debug, Display, JSON, logs, or fixtures.
- Deadlines and cancellation are explicit; unknown completion after timeout is
  classified as indeterminate and cannot be retried as a fresh create.
- Cancellation uses typed `cancelled`, `already_cancelled`, `completed_before_cancel`,
  `cancel_rejected`, and `indeterminate` outcomes. Repeated cancellation is
  idempotent. The transport's final inspect result is authoritative when cancel
  races completion; no message or handoff is admitted after a terminal cancel.

## Budgets

- Product implementation: at most 2,500 physical Rust lines.
- Tests and fixtures: at most 2,500 physical lines and fewer than 100 focused
  tests.
- Focused validation: at most 180 seconds on FastWork.
- Aggregate local validation: at most 600 seconds on FastWork.
- Complete issue validation including deferred hosted CI: at most 3,600
  seconds. Hosted CI is integration proof, not a substitute for the 600-second
  local aggregate.
- Direct crates: only the six reviewed COTS families above, reusing workspace
  dependencies where present.

Exceeding a budget requires exact-revision review and an explicit typed
exception. The repository-wide 20K ceiling does not authorize local growth.

## Validation Plan

1. Preparation validation proves six cards, design/diagram, exact dependency
   gates, issue-local scope, COTS, budgets, privacy boundaries, and no product
   changes.
2. The dependency gate checks live remote merge and ancestry for #5499 and
   #5349 against refreshed `origin/main`, then reports retained receipts as
   audit-only evidence.
3. Future deterministic fixtures cover every operation, identical and
   conflicting retries, stale owner/task/revision/claim/dependency/path state,
   transcript redaction, repeated cancellation, cancellation-versus-completion,
   post-cancellation message and handoff rejection, authoritative terminal
   inspection, timeout, unknown completion, and transport failure.
4. Future focused tests prove sanitized #5500 observations and #5502 handoff
   references without retaining transcript bodies.
5. Strict Clippy, formatting, line/test budgets, network-denied tests, diff
   hygiene, and exact-revision subagent review complete before publication.
6. The future product lane remains unselected until both terminal gates pass
   and the product manifest exists; Cargo commands use `--offline`.

## Non-Goals

- Product implementation or live task execution during preparation.
- Conductor planning, dashboard rendering, output convergence, or live-workcell
  proof.
- Durable Memory Palace storage or private transcript retention.
- Autonomous issue creation, review, publication, merge, closeout, or scope
  expansion.
- Runtime v2 edits, AWS, provider calls, or provider-independent federation.

## Stop Conditions

- #5499 or #5349 lacks live merge into `origin/main` or ancestry to the
  execution base.
- Product scope overlaps another active claim.
- An operation lacks exact authority, ownership, freshness, or output binding.
- Duplicate or stale task state cannot be classified deterministically.
- Safe proof requires private transcript persistence, a new lifecycle store, or
  hidden authority.
- Validation or growth exceeds the reviewed budget.
