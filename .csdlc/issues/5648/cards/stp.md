# Structured Task Prompt

Template: 1.0.0

Issue: 5648

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement and validate the typed active-claim revoke operation.

## Deliverables

- versioned revoke request schema
- typed csdlc-bind route
- atomic store operation
- focused tests
- design and diagram

## Acceptance

1. AC-1: Revoke request is versioned, schema-visible, and requires exact issue/generation/digest/claim identity.
2. AC-2: Explicit operator authority and non-empty reason are mandatory.
3. AC-3: Successful revoke atomically clears only the matching claim and appends audit evidence without changing phase.
4. AC-4: Stale, mismatched, or incomplete requests fail closed without mutation.
5. AC-5: Focused tests and strict Clippy pass with no AWS, raw gh, shell/Python lifecycle, or network behavior.

## Dependencies

- csdlc-v2 lifecycle/store and typed bind route

## Inputs

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/bin/csdlc-bind.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/lib.rs

## Non Goals

- issue closure
- terminal closeout
- branch/worktree deletion
- product/runtime changes
- automatic claim stealing
