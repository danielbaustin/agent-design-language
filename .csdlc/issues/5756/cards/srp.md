# Structured Review Prompt

Template: 1.0.0

Issue: 5756

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

adl/src/provider_adapter.rs

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

Revision: Some("git-blake3:92fed26e1ff2031a57d80a014fbef77542da55d8:ec2eef569becd5f45193c2450055559f61aff9585114fd2aaeb989728357f85b")

Reviewer: Some("codex-bounded-review")

Result: pass
