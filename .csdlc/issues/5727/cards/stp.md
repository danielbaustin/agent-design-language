# Structured Task Prompt

Template: 1.0.0

Issue: 5727

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Add only the typed reacquisition operation, dormant doctor classification, and focused proof required by #5727.

## Deliverables

- typed reacquire request and result
- csdlc-bind reacquire command route
- dormant-record doctor classification and next action
- focused claim lifecycle tests
- #5354 reproduction proof

## Acceptance

1. AC-1: A resumable nonterminal record with claim null is valid dormant state.
2. AC-2: Typed compare-and-swap reacquisition validates issue, generation, digest, actor, purpose, branch, worktree, lease timestamps, and protected paths.
3. AC-3: Reacquisition rejects overlap with other live protected-path claims.
4. AC-4: Released and expired claims are reacquired without lifecycle rewind or audit loss.
5. AC-5: Read-only inspection does not require an active claim while mutations still do.
6. AC-6: Focused tests cover release, expiry, overlap, stale identity, invalid binding, and append-only audit.
7. AC-7: Prepared issue #5354 returns to doctor PASS through the typed operation.

## Dependencies

- none

## Inputs

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/bin/csdlc-bind.rs
- csdlc-v2/tests/gate2.rs
- csdlc-v2/tests/gate7_lifecycle.rs
- .csdlc/issues/5354/index.json

## Non Goals

- new lifecycle phases
- compatibility wrappers
- receipt or closeout gating for claims
- direct mutation of #5354 state
- WP-15 implementation
