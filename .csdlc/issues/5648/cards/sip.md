# Structured Intent Prompt

Template: 1.0.0

Issue: 5648

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Provide a narrow typed CAS-guarded operator operation to revoke an abandoned active claim without waiting for lease expiry or hand-editing canonical state.

## Required Outcome

A schema-visible csdlc-bind revoke route, atomic claim clearing, audit evidence, and focused fail-closed tests.

## Scope

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/bin/csdlc-bind.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/tests

## Authority

- Only an explicit operator-authorized request may revoke a live claim; ordinary expiry recovery and closeout remain unchanged.

## Assumptions

- none

## Operator Constraints

- never write main
- use typed v2 binaries
- no raw gh
- no AWS
- no direct state/card edits
