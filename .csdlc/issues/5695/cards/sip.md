# Structured Intent Prompt

Template: 1.0.0

Issue: 5695

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make csdlc-pr-state distinguish stale-base ancestry from pending GitHub merge-policy states.

## Required Outcome

Explicit mergeability classification preserves behind as stale_base, blocked and unstable as waiting, dirty as conflicted, and unknown as waiting with focused proof.

## Scope

- csdlc-v2/src/github.rs
- csdlc-v2/src/github.rs tests
- issue-local typed cards and evidence

## Authority

- Issue 5695 owns mergeability classification and focused tests only
- csdlc-merge remains fail-closed and GitHub remains merge authority

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 only
- No main checkout edits
- No AWS
- No provider or runtime changes
- Do not reopen issue 5683
