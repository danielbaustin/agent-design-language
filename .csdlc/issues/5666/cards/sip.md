# Structured Intent Prompt

Template: 1.0.0

Issue: 5666

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Reduce development latency by making validation, review, PR watching, and lifecycle effort proportional to issue risk.

## Required Outcome

A checked-in fast-lane policy and focused contract proof define how tiny tool/docs/test fixes move quickly without weakening typed C-SDLC v2 authority or exact-head truth.

## Scope

- developer throughput fast-lane policy
- validation routing documentation link and policy reference to the validation selector
- small focused shell contract test for policy invariants
- issue-local C-SDLC v2 lifecycle records

## Authority

- typed C-SDLC v2 remains the lifecycle authority
- fast lane changes validation selection and operator behavior, not product/runtime semantics
- GitHub wait states must be changed-state/blocker reporting only

## Assumptions

- none

## Operator Constraints

- Use FastWork for worktree, temp, and Rust build output
- Do not write on root main
- No AWS execution
- No raw GitHub CLI or connector fallback for issue lifecycle
- Keep the implementation small
