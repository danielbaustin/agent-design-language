# Structured Task Prompt

Template: 1.0.0

Issue: 5778

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement only the first serialized closeout-simplification slice: derived terminal truth and one idempotent finish command with compatibility reads; leave cleanup and deletion to #5779 and #5780.

## Deliverables

- Derived terminal resolver and typed result schemas
- csdlc-finish owner binary
- Exact-head and idempotent finish integration tests
- Logical terminal claim-release compatibility
- Updated operator manifest and focused documentation

## Acceptance

1. AC-1: One implementation issue needs one PR and successful finish creates no tracked post-merge diff or second PR
2. AC-2: Exact-head review, required checks, publication identity, and expected-SHA merge protection remain fail-closed
3. AC-3: Already-merged, interrupted-after-merge, and concurrent finish calls converge idempotently
4. AC-4: Merged, closed-unmerged, and approved no-PR outcomes derive from live GitHub and immutable Git truth
5. AC-5: Terminal remote state makes stale local claims non-blocking without deleting dirty work
6. AC-6: Legacy terminal records and receipts remain readable without rewrite
7. AC-7: Focused Gate 4-7 tests and strict Clippy pass

## Dependencies

- #5627 and PR #5629 complete
- GitHub API and Git graph remain canonical remote authorities
- #5779 and #5780 follow after this issue

## Inputs

- GitHub issue #5778
- #5627 and PR #5629
- #5748 and PR #5777
- csdlc-v2/src/bin/csdlc-closeout.rs
- csdlc-v2/src/readiness.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/merge.rs
- csdlc-v2/src/github.rs
- csdlc-v2/tests/gate7_lifecycle.rs

## Non Goals

- Legacy command deletion before parity
- Standalone cleanup implementation owned by follow-on #5779
- Historical record rewrite
- Runtime, provider, ADL language, or deployment changes
