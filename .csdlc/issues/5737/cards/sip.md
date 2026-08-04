# Structured Intent Prompt

Template: 1.0.0

Issue: 5737

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Keep claim authority scans precise and recoverable when unrelated terminal records, inactive stale projections, or dormant design inputs are stale.

## Required Outcome

Non-overlapping init, bind, and reacquire ignore unrelated stale terminal identities and inactive stale projections; dormant claim authority can be reacquired before typed design reapproval; live overlaps still fail closed.

## Scope

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate2.rs

## Authority

- C-SDLC v2 lifecycle authority remains in the Rust binaries and typed state store.
- The patch changes claim-scan filtering, terminal-check ordering, and authority-record replacement only; it does not weaken live claim collision, checkout, lease, or CAS checks.

## Assumptions

- none

## Operator Constraints

- never touch or switch primary main
- use typed v2 binaries
- do not hand-edit rendered cards
- do not use /private/tmp
- report protected-path collisions immediately
