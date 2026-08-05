# Structured Task Prompt

Template: 1.0.0

Issue: 5861

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Redesign and implement only issue publication, preparation, sealing, binding, release, migration, and their immediate typed proof surfaces.

## Deliverables

- Draft, prepared, execution-ready, binding, and bound state contracts
- Canonical csdlc-issue, csdlc-prepare, and csdlc-bind command routes
- Immutable generation manifest and semantic readiness receipt
- Repository-wide path reservation and binding intent recovery protocol
- Audited legacy migration and repair path
- Focused Rust concurrency, crash, drift, batch, and migration tests
- Updated operator skills and architecture documentation

## Acceptance

1. Created issues are visibly draft and no file or status prose can imply readiness
2. Sync produces design, diagram, and six editable typed cards without binding or a claim
3. Seal rejects placeholders, invalid schemas, stale dependencies, missing owned paths, and non-proving validation plans while preserving the last complete prepared generation
4. Seal emits a semantic-digest and dependency-vector pinned receipt and doctor reports ready true with next operation bind
5. Substantive typed edits invalidate the receipt and return to prepared without requiring network sync
6. Bind derives owner and Git topology from issue and session context and uses a recoverable binding-to-bound protocol
7. Repository-wide path reservation gives overlapping concurrent binds exactly one winner before any loser Git mutation
8. Owner release serializes with bind and removes only intent-proven unstarted Git artifacts
9. Batch preparation preserves per-child truth, detects cycles and intra-batch overlaps, and cannot overstate umbrella readiness
10. Legacy preparation claims migrate with reversible audit evidence while valid active claims remain intact and ambiguity routes to one typed repair action
11. Focused Rust tests cover forged readiness, semantic drift, dependency drift, generation CAS, crash windows, concurrency, release, compensation, batch truth, migration, and doctor routing
12. After parity, coupled init, reserve, and reacquire behavior is deprecated and deleted without wrappers or compatibility retries

## Dependencies

- Issue 5860 current v0.92 execution-readiness repair evidence as characterization input only
- Current csdlc-v2 public schemas and session ledger contracts
- Current Git worktree and repository binding registry behavior

## Inputs

- AGENTS.md
- csdlc-v2/src
- csdlc-v2/tests
- csdlc-v2/operator
- docs/architecture/csdlc-v2
- .csdlc/prepared/issues/5861/design.md
- .csdlc/prepared/issues/5861/diagram.mmd
- .csdlc/prepared/issues/5861/provider-review-disposition.md

## Non Goals

- Removing issue-bound worktrees or overlap protection
- Combining implementation, review, publication, merge, or closeout into preparation
- Changing unrelated ADL Runtime or product behavior
- Making WP-01 depend on this v0.92 sidecar
- Supporting cross-host mutation without a proven shared lock authority
