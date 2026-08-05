# Structured Intent Prompt

Template: 1.0.0

Issue: 5800

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make the separate local Observatory and Runtime v3 API load through one browser-trusted localhost HTTPS identity without warning or verification bypasses.

## Required Outcome

A documented, reproducible local certificate generation, trust installation, reissue, configuration, startup, and recovery flow makes Chrome, curl, Observatory HTML, Runtime health/readiness, and the Runtime feed agree on verified HTTPS.

## Scope

- adl-runtime/src/local_tls.rs
- adl-runtime/src/bin/adl-runtime-local-tls-bootstrap.rs
- adl-runtime/tests/local_tls.rs
- demos/html-observatory/runtime-v3.config.json
- demos/html-observatory/README.md
- adl/tools/validate_v092_browser_trusted_observatory.mjs

## Authority

- Issue 5800 owns local localhost certificate trust, issuance or reissue, and HTTPS consistency only
- Issue 5820 owns broader Guardian and Runtime launch resilience; overlapping init or TLS files serialize
- Issue 5795 and issue 5837 consume this trusted path and cannot redefine it
- No AWS, ACM, Route53, public-domain, or verification-bypass authority

## Assumptions

- none

## Operator Constraints

- Do not bypass browser or TLS verification
- Use one stable supported trust model
- Keep configuration and docs source-grounded
- Never edit tracked work on main
- Use one bounded pre-PR review
