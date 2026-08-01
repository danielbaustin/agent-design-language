# Structured Review Prompt

Template: 1.0.0

Issue: 5756

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl/src/provider_adapter.rs and .csdlc issue 5756 lifecycle artifacts

## Prompts

- Verify the shared non-2xx classifier can no longer map bare 1008 text to ProviderBillingBlocked for non-MiniMax providers.
- Verify MiniMax structured status_code 1008 remains non-retryable ProviderBillingBlocked.
- Verify tests cover OpenAI, Anthropic, DeepSeek, and generic provider responses containing 1008 without relying on live credentials.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
