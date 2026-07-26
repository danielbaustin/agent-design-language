# Structured Task Prompt

Template: 1.0.0

Issue: 5671

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement the Opus 5 profile/setup route and bounded tests in the existing provider modules.

## Deliverables

- provider profile
- setup template
- focused Rust tests
- design and diagram

## Acceptance

1. AC-1: claude:claude-opus-5 resolves to the Anthropic provider kind, endpoint, and canonical model id.
2. AC-2: Existing Claude profiles remain unchanged and profile expansion preserves vendor identity.
3. AC-3: provider setup claude-opus-5 emits the profile and ANTHROPIC_API_KEY route.
4. AC-4: Mocked Anthropic adapter proof verifies claude-opus-5 and the canonical version header.
5. AC-5: Focused Rust validation and exact-head review pass without a live provider call.

## Dependencies

- existing Rust-native Anthropic Messages adapter
- canonical model id claude-opus-5

## Inputs

- adl/src/provider/profiles.rs
- adl/src/provider/mod.rs
- adl/src/cli/provider_cmd.rs
- adl/src/provider/http_family.rs

## Non Goals

- new transport
- live API call
- credential provisioning
- pricing or benchmarking
