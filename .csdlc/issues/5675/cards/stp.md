# Structured Task Prompt

Template: 1.0.0

Issue: 5675

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement and validate the two hosted provider adapter routes and their bounded tests.

## Deliverables

- Kimi hosted route
- MiniMax hosted route
- bounded error and budget handling
- focused tests
- live probe evidence

## Acceptance

1. AC-1: Kimi and MiniMax dispatch through adl-provider-adapter with bearer credentials.
2. AC-2: Requests use bounded provider-compatible model, message, and token fields.
3. AC-3: MiniMax billing/error envelopes and Kimi insufficient-balance responses are typed and non-retryable where appropriate.
4. AC-4: Focused tests prove auth, request shape, response extraction, redaction, and billing classification.
5. AC-5: Live probes reach both providers through the adapter and retain truthful credit-failure evidence when accounts cannot execute.

## Dependencies

- existing Rust provider adapter
- approved Moonshot and MiniMax credential files

## Inputs

- adl/src/provider_adapter.rs
- adl/src/provider/profiles.rs
- adl/src/provider_communication.rs

## Non Goals

- new provider transports
- credential provisioning
- AWS execution
- shell/Python lifecycle code
