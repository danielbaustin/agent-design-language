# Structured Intent Prompt

Template: 1.0.0

Issue: 5755

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Fix the two accepted #5664 terminal-closeout security blockers so #5664 can be closed out truthfully.

## Required Outcome

Runtime v3 protocol adapters have a current authenticated TLS client-identity boundary or equivalent, Runtime control rejects oversized inbound bodies at the route boundary, and focused tests prove both negative paths.

## Scope

- adl-runtime-kernel/src/protocol_adapters.rs
- adl-runtime-kernel/tests/protocol_adapters.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/tests/control.rs
- .csdlc/evidence/5755

## Authority

- Issue #5755 owns the repair.
- Issue #5664 terminal closeout consumes the result after merge.
- No AWS or external infrastructure authority.

## Assumptions

- none

## Operator Constraints

- use /Volumes/FastWork
- no AWS
- no tracked product edits on main
