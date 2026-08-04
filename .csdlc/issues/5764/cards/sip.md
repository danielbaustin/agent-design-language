# Structured Intent Prompt

Template: 1.0.0

Issue: 5764

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make Runtime v3 and HTML Observatory overnight monitoring use canonical endpoints and report weather freshness/readiness truth without ambiguity.

## Required Outcome

A bounded Runtime v3/operator-observatory repair where documented probes, API routes, and watcher checks agree on live health, readiness, observability readiness, and stale weather handling.

## Scope

- Runtime v3 control/readiness route behavior
- Runtime v3 route-focused tests
- Runtime/OpenAPI route documentation if route inventory changes
- HTML Observatory README/operator probe instructions

## Authority

- Use typed C-SDLC v2 lifecycle and repo-native tools
- Do not use AWS
- Do not change root main
- Do not expand browser mutation authority or Runtime v3 cutover scope

## Assumptions

- none

## Operator Constraints

- Use FastWork worktree
- Keep validation focused
- Leave currently running runtime/observatory services running unless explicitly instructed
