# Structured Intent Prompt

Template: 1.0.0

Issue: 5671

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Add a first-class Claude Opus 5 provider profile and setup route using the existing Rust-native Anthropic Messages adapter.

## Required Outcome

Profile expansion, setup generation, and focused mocked adapter proof for claude-opus-5.

## Scope

- adl/src/provider/profiles.rs
- adl/src/provider/mod.rs
- adl/src/cli/provider_cmd.rs

## Authority

- The change defines local provider routing only; it does not provision credentials, call Anthropic live, or approve release policy.

## Assumptions

- none

## Operator Constraints

- never write main
- use typed v2 binaries
- no raw gh
- no AWS
- no direct card/state edits
