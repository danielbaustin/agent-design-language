# Structured Task Prompt

Template: 1.0.0

Issue: 5658

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement the bounded C-SDLC v2 lifecycle-root repair and prove it with focused Rust tests.

## Deliverables

- Bound-worktree lifecycle materialization fix
- Fail-closed guard for unsafe primary-main lifecycle writes
- Regression covering ignored .csdlc state in a newly-created worktree

## Acceptance

1. AC-1: csdlc-bind materializes or verifies issue record, cards, lock, prepared artifacts, and evidence root in the declared bound worktree
2. AC-2: write attempts against primary main fail closed unless explicitly bootstrap/read-only
3. AC-3: bound-worktree doctor, validate, review, publish, and closeout use the same repository root as implementation
4. AC-4: a regression covers a newly-created worktree where .csdlc paths are ignored or absent
5. AC-5: claim, lock, and exact-revision protections remain strict

## Dependencies

- Issue #5655 typed GitHub action surface remains preserved

## Inputs

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate7_lifecycle.rs
- root AGENTS.md workflow rules
- GitHub issue #5658

## Non Goals

- No manual copying of existing issue records
- No cleanup of root #5671 artifacts in this issue
- No shell/Python lifecycle wrapper
- No weakening claim, lock, or exact-revision protections
- No AWS
