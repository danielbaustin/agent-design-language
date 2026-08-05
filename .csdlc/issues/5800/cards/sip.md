# Structured Intent Prompt

Template: 1.0.0

Issue: 5800

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make the supported local Observatory load over browser-trusted HTTPS without warning bypasses.

## Required Outcome

One documented and reproducible local trust flow makes Chrome, Observatory startup, configured URLs, health checks, and Runtime feed access agree on trusted HTTPS.

## Scope

- demos/html-observatory
- adl-runtime/src/local_tls.rs
- adl-runtime/tests/local_tls.rs
- Runtime and Observatory HTTPS configuration and docs
- .csdlc/issues/5800
- .csdlc/evidence/5800

## Authority

- Issue 5800 owns local Observatory certificate trust and HTTPS consistency
- WP-03 owns broader Runtime launch and resilience
- No AWS certificate or hosted production TLS work is authorized

## Assumptions

- none

## Operator Constraints

- Do not bypass browser or TLS verification
- Use one stable supported trust model
- Keep configuration and docs source-grounded
- Never edit tracked work on main
- Use one bounded pre-PR review
