# Structured Intent Prompt

Template: 1.0.0

Issue: 5470

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make terminal projection and retained receipt updates crash-consistent and recoverable.

## Required Outcome

A terminal reconciliation either durably commits both projection and receipt or leaves a journaled state that deterministically recovers without generation or identity drift.

## Scope

- csdlc-v2/src/store.rs
- csdlc-v2/src/bin/csdlc-closeout.rs
- csdlc-v2/tests
- .csdlc/issues/5470/retained/design.md
- .csdlc/issues/5470/retained/diagram.mmd

## Authority

- The Rust v2 store and receipt journal are the sole durability authority
- Existing terminal receipt identity, rollback, idempotence, and typed reconciliation contracts remain unchanged

## Assumptions

- none

## Operator Constraints

- none
