# Structured Intent Prompt

Template: 1.0.0

Issue: 5756

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Scope MiniMax billing classification so bare 1008 substrings in other hosted provider non-2xx responses do not become ProviderBillingBlocked.

## Required Outcome

MiniMax code 1008 remains non-retryable ProviderBillingBlocked, while OpenAI, Anthropic, DeepSeek, and generic provider bodies containing 1008 retain their normal provider-specific classification and retry behavior.

## Scope

- adl/src/provider_adapter.rs shared hosted HTTP failure classification
- focused provider adapter regression tests

## Authority

- typed C-SDLC v2 binaries own lifecycle state
- no AWS use
- no /private/tmp
- no #5748 inspection or dependency

## Assumptions

- none

## Operator Constraints

- work only in /Volumes/FastWork/adl-wp-5756
- do not write tracked files on primary main
- publish a ready PR with Closes #5756
- do not block on asynchronous closeout
