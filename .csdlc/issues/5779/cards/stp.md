# Structured Task Prompt

Template: 1.0.0

Issue: 5779

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement and prove the bounded standalone cleanup and legacy compatibility migration surface required by issue #5779.

## Deliverables

- csdlc-clean binary and library contract
- Compatibility-only legacy receipt index
- Migration parity validator
- Focused cleanup and concurrency tests
- Updated operator inventory and documentation

## Acceptance

1. AC-1: cleanup success, skip, or failure cannot change derived terminal truth
2. AC-2: dirty or unclassified worktrees return cleanup_skipped_dirty with exact paths and are not deleted
3. AC-3: clean registered worktrees are removed idempotently
4. AC-4: removing legacy receipts in a fixture does not change derived terminal resolution
5. AC-5: current v0.91.8 terminal records match the compatibility census without historical rewrites
6. AC-6: focused tests, strict Clippy, and exact-head review pass

## Dependencies

- Issue #5778 and PR #5782 merged
- Derived terminal resolver and Git-common cache
- Legacy terminal receipt corpus

## Inputs

- GitHub issue #5779
- csdlc-v2/src/finish.rs
- csdlc-v2/src/bin/csdlc-closeout.rs
- csdlc-v2/src/readiness.rs
- csdlc-v2/src/store.rs
- .csdlc/evidence/5748/v0918-closeout-prune-results.json

## Non Goals

- No terminal lifecycle redesign beyond consuming #5778
- No deletion of legacy commands in this issue
- No forced cleanup or external archive service
