# Structured Intent Prompt

Template: 1.0.0

Issue: 5487

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Repair and reapprove retained design artifacts for closed-out issues through a typed atomic route.

## Required Outcome

A closed-out issue can receive an explicitly authorized, hash-bound design/diagram repair whose receipt, cards, projections, and audit record update atomically and can be materialized by reconcile-terminal.

## Scope

- csdlc-v2/src/store.rs
- csdlc-v2/src/bin/csdlc-closeout.rs
- csdlc-v2/src/model.rs
- csdlc-v2/tests
- .csdlc/issues/5487/retained/design.md
- .csdlc/issues/5487/retained/diagram.mmd
- .csdlc/issues/5467/retained/design.md
- .csdlc/issues/5467/retained/diagram.mmd

## Authority

- Rust v2 closeout/store code is the sole mutation authority
- Repair authority must be explicit and hash-bound
- Terminal receipts remain immutable except through the typed repair transaction

## Assumptions

- none

## Operator Constraints

- none
