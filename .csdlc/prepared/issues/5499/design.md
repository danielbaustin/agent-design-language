# WP-10A Conductor Preparation Design

## Status

Preparation only. Issue #5499 must not implement product code until WP-09
#5349 and the direct issue dependencies #5340, #5341, and #5342 are live-merged
and their merged revisions are ancestors of the execution base. Typed closeout
and retained shared-Git receipts are audit-only evidence and must not block
readiness by themselves.

## Purpose

The conductor converts a typed C-SDLC v2 issue graph plus an ADL v2 execution
plan into a deterministic set of admitted assignments and refusals. It plans;
it does not create tasks, mutate GitHub, merge branches, close issues, or become
a second lifecycle database or scheduler.

## Contract Boundary

Inputs are immutable snapshots with explicit schema versions and source
revisions:

- typed C-SDLC v2 issue, claim, card, dependency, and validation-lane state;
- an ADL v2 execution plan and declared write/protected path sets;
- the global writable-actor WIP limit and serialized integration gates;
- a deterministic planning correlation seed.

Outputs are typed, canonical records:

- `ConductorPlan` with admitted serial/parallel lanes in stable order;
- `TaskAssignment` values containing issue, claim, branch, worktree, scope,
  dependencies, validation lanes, expected outputs, and correlation id;
- `RefusalRecord` values with machine-readable reason codes and evidence refs;
- no side effects beyond returning the record to the caller.

The component must fail closed for missing or malformed cards, stale or absent
claims, unresolved dependencies, cycles, overlapping protected/write paths,
unknown validation lanes, WIP overflow, or ambiguous integration authority.
Review, publication, merge, post-merge validation, and closeout remain
serialized conductor-owned gates and are never delegated as writable shards.

## Planned Product Boundary

After the dependency gate opens, implementation should be isolated under
`adl-v2/crates/adl-workcell-conductor/`. Any workspace-manifest change is a
serialized integration edit and must be added through a typed claim amendment
only after current-main integration. Preparation claims only issue-local C-SDLC
paths and grants no product-write authority.

The public component interface should remain small and pure:

```text
plan(ConductorInput) -> Result<ConductorDecision, ConductorRefusal>
```

No runtime, network, filesystem, task, or GitHub client belongs inside this
component. The #5498 adapter consumes assignments and owns explicit task
operations. #5500 consumes read-only projections. #5502 owns output
convergence and replanning.

## COTS Strategy

- `petgraph = 0.8.3` for cycle detection, topological ordering, and graph
  traversal instead of custom graph algorithms.
- `serde = 1.0.228` and the workspace-compatible `serde_json` for typed,
  versioned records and deterministic fixtures.
- `blake3 = 1.8.5` for content-derived correlation identifiers.
- `thiserror = 2.0.18` for explicit refusal/error types.

No service, paid dependency, new database, task scheduler, or network client is
required.

## Determinism And Security

- Normalize issue ids, dependency edges, path sets, and validation lanes before
  planning; sort all externally observable collections canonically.
- Derive correlation ids from schema version plus canonical input bytes; never
  use wall-clock time or random UUIDs for planning identity.
- Compare normalized repository-relative paths segment-by-segment; reject
  absolute paths, parent traversal, empty paths, and prefix collisions.
- Do not ingest secrets, private transcripts, provider credentials, or raw task
  content.
- Preserve source revision and evidence references without copying unrelated
  lifecycle state into a second store.

## Budgets

- Product implementation: at most 3,000 physical Rust lines.
- Tests and fixtures: at most 3,000 physical lines and fewer than 120 focused
  tests.
- Focused validation: at most 180 seconds on FastWork.
- Complete issue validation: at most 600 seconds on FastWork.
- New direct crates: only the four reviewed COTS dependencies above unless a
  later exact design review approves a replacement.

Exceeding a budget requires an exact-revision review and an explicit typed
exception; the 20K repository ceiling is not authority for growth here.

## Validation Plan

1. Preparation validation proves all six cards, design, diagram, dependency
   gate, COTS declarations, budgets, scope boundaries, and no-product-change
   posture.
2. The dependency gate verifies live merge and ancestry for #5340, #5341,
   #5342, and final gate #5349; retained receipts are reported as audit-only
   evidence.
3. Future focused tests cover deterministic ordering, cycles, stale claims,
   missing cards, unknown lanes, WIP limits, exact and prefix path collisions,
   serialization gates, and content-derived correlation ids.
4. Strict Clippy, format, line/test budgets, diff hygiene, and exact-revision
   subagent review complete before publication.
5. The future conductor lane is retained in the network-denied PVF manifest but
   remains unselected and explicitly deferred until every dependency gate is
   live-merged and ancestral and the product manifest exists. Its Cargo commands use `--offline`
   so a missing same-host cache fails closed instead of reaching the network.
6. Preparation diff hygiene compares the recorded issue base to exact `HEAD`,
   rather than treating a clean working tree as proof of the committed patch.

## Non-Goals

- Product implementation during preparation.
- Task creation, task messaging, cancellation, or context transport.
- Autonomous issue creation, merge, publication, or closeout.
- A second lifecycle database or replacement scheduler.
- Runtime v2 edits, AWS use, provider calls, or network execution.
- Dashboard, convergence, or live-workcell proof owned by later WP-10A issues.

## Stop Conditions

- #5349 or any direct dependency lacks live merge or ancestry.
- A product path overlaps another active typed claim.
- The implementation requires lifecycle or scheduling authority beyond this
  pure planning boundary.
- A proposed dependency duplicates a maintained COTS capability.
- Validation cannot remain deterministic and bounded.
