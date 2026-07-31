# Structured Intent Prompt

Template: 1.0.0

Issue: 5692

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make the repository policy and typed publication path require every implementation PR body to include the correct GitHub closing keyword for its issue, normally Closes #<issue>.

## Required Outcome

A small policy/tooling change that prevents publication of implementation PRs without an actual GitHub closing keyword and documents the rule in AGENTS.md.

## Scope

- Root AGENTS.md implementation PR publication rule
- C-SDLC v2 publication request and remote PR body linkage validation
- Focused publication verifier tests

## Authority

- Use typed C-SDLC v2 lifecycle and repo-native GitHub tooling only
- Do not rewrite the broader workflow or closeout model
- Asynchronous typed closeout remains separate and nonblocking

## Assumptions

- none

## Operator Constraints

- Never edit main
- Never use /private/tmp
- Keep artifacts in the issue worktree
- PR body for this issue must include Closes #5692
