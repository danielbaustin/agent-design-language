# WP-10A Output Convergence And Replanning Preparation Design

## Status

Preparation only. Product implementation is forbidden until both #5499 and
#5498 have live GitHub merged revisions and those revisions are ancestors of
the #5502 execution base. Typed `closed_out`, retained receipts, and claim
release are audit-only signals; they must not block readiness once live
merge and ancestry are true.

## Purpose

#5502 consumes bounded task outputs and conductor assignments, validates their
identity and authority, and returns either a deterministic integration plan or
a typed blocked/replan decision. It does not create tasks, mutate GitHub,
merge branches, close issues, or become another lifecycle database.

## Contract Boundary

Inputs are immutable, versioned records:

- the exact #5499 assignment plan and dependency/interface-freeze graph;
- #5498 task results bound to issue, claim, branch, worktree, source revision,
  protected/write paths, artifacts, validation, and review evidence;
- current typed C-SDLC v2 claim and lifecycle observations;
- a deterministic correlation seed and declared integration authority.

Outputs are canonical records:

- `ConvergenceDecision::Integrate(IntegrationPlan)` with stable ordering;
- `ConvergenceDecision::Replan(ReplanRecord)` with changed assumptions and
  admissible remaining work;
- `ConvergenceDecision::Blocked(BlockedRecord)` with machine-readable reasons;
- a read-only projection carrying partial successes and residual blockers for
  #5500 and #5501.

The component fails closed on missing, stale, forged, overlapping,
out-of-scope, ambiguously reviewed, or revision-discontinuous outputs. Review,
publication, merge, post-merge validation, and closeout remain serialized and
independently authorized.

## Planned Product Boundary

After the dependency gate opens, implementation should be isolated under
`adl-v2/crates/adl-workcell-convergence/`. Workspace-manifest edits require a
later typed claim amendment after current-main integration. This preparation
claim grants no product-write authority.

The public interface remains pure and small:

```text
converge(ConvergenceInput) -> Result<ConvergenceDecision, ConvergenceError>
```

No runtime, network, filesystem, GitHub, task-client, or merge client belongs
inside this component.

## Determinism And Security

- Canonicalize issue ids, dependency edges, revisions, artifact digests,
  validation/review references, and repository-relative path sets.
- Derive decision identity from schema version and canonical input bytes using
  BLAKE3; do not use clocks or random identifiers.
- Require exact assignment/output binding and reject duplicate identity with
  conflicting content.
- Reject absolute paths, parent traversal, malformed revisions, secret-bearing
  fields, private transcript content, and undeclared artifacts.
- Stable topological integration order comes from the admitted #5499 plan;
  changed assumptions produce a typed replan and never silent scope expansion.

## COTS Strategy

- `serde = 1.0.228` and workspace-compatible `serde_json` for typed records.
- `blake3 = 1.8.5` for content-derived decision identifiers.
- `thiserror = 2.0.18` for explicit fail-closed error categories.
- Reuse the #5499 conductor's maintained graph/order contract; do not add a
  second graph algorithm or scheduler.

No paid service, database, network client, or new orchestration framework is
required.

## Budgets

- Product implementation: at most 2,500 physical Rust lines.
- Tests and fixtures: at most 2,500 physical lines and fewer than 100 focused
  tests.
- Modules: each below 500 physical lines unless exact review approves a split.
- Focused validation: at most 120 seconds on FastWork.
- Complete issue validation: at most 600 seconds on FastWork.
- New direct crates: only the four reviewed COTS dependencies above, counting
  `serde_json` separately, unless an exact design review approves a change.

## Validation Plan

1. Preparation proof checks all six cards, design/diagram, exact dependencies,
   preparation-only paths, COTS, budgets, PVF, and no-product-change truth.
2. The dependency gate proves #5499 and #5498 live merged revisions and
   ancestry. Typed closeout receipts and claim release are reported only as
   audit observations.
3. Future property/fixture tests cover stale or forged outputs, path overlap,
   partial success, changed assumptions, deterministic order and ids, replan
   convergence, and serialized review/closeout authority.
4. Format, strict Clippy, exact line/test budgets, offline tests, diff hygiene,
   exact-revision review, CI, and post-merge proof are mandatory.

## Non-Goals

- Product implementation during preparation.
- Task creation or transport owned by #5498.
- Conductor planning owned by #5499.
- Dashboard rendering owned by #5500 or live workcell proof owned by #5501.
- Automatic merge, review approval, publication, closeout, or issue mutation.
- Runtime v2 edits, AWS, provider calls, credentials, or network execution.

## Stop Conditions

- #5499 or #5498 lacks a live GitHub merged revision ancestral to the #5502
  execution base.
- A future product path overlaps another active typed claim.
- The design requires hidden mutation, a second state store, scheduler, or
  lifecycle authority.
- Output identity or authority cannot be checked deterministically.
- Any budget or required proof would be deferred rather than completed.
