# Structured Task Prompt

Template: 1.0.0

Issue: 5737

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Record and publish the product/test patch for issue #5737 from the existing worktree.

## Deliverables

- claim-scan terminal check ordering fix
- inactive stale projection filtering
- authority-only reacquisition record replacement
- focused Gate 2 regression coverage
- strict Clippy proof
- exact-head review
- ready PR closing #5737

## Acceptance

1. AC-1: Unrelated stale terminal identity cannot block non-overlapping init, bind, or reacquire.
2. AC-2: A dormant issue with changed design inputs can reacquire authority and then use typed design reapproval.
3. AC-3: Real overlapping live claims still fail closed.
4. AC-4: Stale projections whose claim branch/worktree do not match their active checkout do not act as collision authority.
5. AC-5: Focused Gate 2 tests and strict Clippy pass.

## Dependencies

- existing C-SDLC v2 claim and store APIs
- repo-native v2 binaries from adl-wp-5107 target/debug

## Inputs

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate2.rs

## Non Goals

- changing card templates
- weakening live protected-path collision behavior
- introducing shell lifecycle wrappers
- modifying primary main
