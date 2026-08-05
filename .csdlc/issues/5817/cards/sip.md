# Structured Intent Prompt

Template: 1.0.0

Issue: 5817

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Convert the reviewed v0.92 candidate planning package into current canonical milestone and executable issue-wave truth.

## Required Outcome

Canonical version and milestone docs agree, prerequisites are evidence-reconciled, the final bounded issue wave is opened without duplicates, and every opened issue has six valid typed cards.

## Scope

- README.md
- docs/README.md
- adl/Cargo.toml
- adl/Cargo.lock
- docs/milestones/v0.92

## Authority

- Issue 5817 owns v0.92 planning activation and issue-wave setup only
- Child issues retain implementation authority
- Issue 5815 owns the reviewed migration plan and WP-02 issue 5819 owns repository-migration execution

## Assumptions

- none

## Operator Constraints

- Never write tracked work on main
- Use current repository and GitHub evidence rather than chat recollection
- Open no duplicate issue
- Use one bounded pre-PR review
- Do not execute child WPs
