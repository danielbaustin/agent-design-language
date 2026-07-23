# Structured Intent Prompt

Template: 1.0.0

Issue: 5500

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Prepare the existing milestone dashboard to become a secure read-only operator projection for distributed C-SDLC workcell and Runtime v3 Observatory state without acquiring mutation or lifecycle authority.

## Required Outcome

After #5498 and final WP-09 gate #5349 are terminal, one mobile-capable dashboard deterministically presents typed retained workcell state plus bounded live Runtime v3 observations with explicit provenance, freshness, and authority labels.

## Scope

- extend docs/tooling/milestone-dashboard instead of creating a new framework
- typed generated workcell snapshot normalization and rendering
- bounded authenticated Runtime v3 Observatory composition
- mobile layout, deterministic fixtures, security, freshness, and non-authority proof

## Authority

- C-SDLC v2 records and owner binaries remain lifecycle and claim authority
- GitHub remains PR, check, and review authority
- Runtime v3 remains runtime observation authority
- #5500 is a read-only projection and never mutates tasks, issues, claims, branches, PRs, reviews, merges, or closeout
- #5502 exclusively owns output convergence and deterministic replanning

## Assumptions

- none

## Operator Constraints

- Use installed typed C-SDLC v2 binaries and semantic card operations only
- Keep root main clean; all tracked #5500 work stays in its dedicated issue worktree
- Do not use raw gh, AWS, credentials, provider calls, Runtime v2, product implementation, publication, or PR creation during preparation
- Do not implement until #5498 and #5349 are live-merged on origin/main and ancestral to the execution base; typed closeout receipts are audit evidence only
- Use /Volumes/FastWork for build and validation output
- Run one bounded implementation review immediately before PR; do not add preparation review churn
