# Issue 5547 Design

## Purpose

Issue #5547 dispositions two retained WP-20 review residuals from #4645 before
external review proceeds:

- IR-4645-011: C-SDLC review/publication revision identity currently accepts a
  review scope while the reviewed revision is derived from whole-tree state.
- IR-4645-012: several large modules concentrate ownership across lifecycle,
  persistence, runtime, provider, scheduler, and review responsibilities.

## Decision Surface

The C-SDLC identity work must choose one truthful contract:

1. Scope-aware identity: revision computation honors declared review pathspecs.
2. Whole-tree identity: the product intentionally advertises whole-tree revision
   identity and stops implying scoped hashes.

The implementation must be fail-closed. If code changes are too risky for
v0.91.7, #5547 records an exact v0.91.8 residual and does not claim the
identity defect is fixed.

## Ownership Split Surface

The ownership split deliverable is a plan, not a refactor. It must map the
large modules named by IR-4645-012 to behavior-first split seams, required
validation lanes, and safe defer boundaries:

- `adl/src/long_lived_agent.rs`
- `adl/src/csm_runtime_api.rs`
- `adl/src/scheduler.rs`
- `adl/src/provider_adapter.rs`
- `csdlc-v2/src/store.rs`

No behavior moves are authorized unless the operator widens the issue.

## Implementation Plan

1. Inspect `csdlc-v2/src/git.rs`, `csdlc-v2/src/publication.rs`, and the
   review/publish entrypoints to verify current revision identity behavior.
2. Decide whether v0.91.7 should implement scope-aware identity or document
   whole-tree identity as the intentional contract.
3. Apply the smallest code/doc change matching that decision, or record an
   exact v0.91.8-bound residual if implementation is deferred.
4. Write the ownership-first split plan for IR-4645-012 under
   `docs/reviews/v0.91.7/review-fixes-5547/`.
5. Run focused C-SDLC validation and, for code changes, focused Rust tests
   around publication/review identity.

## Invariants

- #5547 implementation happens only in the bound issue worktree.
- Review identity claims must match executable behavior.
- Deferred residuals must remain explicit and not be described as fixed.
- The ownership split plan must describe behavior-preserving migration order,
  validation lanes, and non-goals.
