# Structured Intent Prompt

Template: 1.0.0

Issue: 5466

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Allow truthful typed closeout when a merged PR head advanced after initial publication.

## Required Outcome

A final reviewed merged head can be reconciled into publication evidence without weakening exact-revision review or fabricating terminal SHA truth.

## Scope

- csdlc-v2/src/bin/csdlc-publish.rs
- csdlc-v2/tests/gate6.rs

## Authority

- Independent Rust C-SDLC v2 publication and closeout path

## Assumptions

- none

## Operator Constraints

- none
