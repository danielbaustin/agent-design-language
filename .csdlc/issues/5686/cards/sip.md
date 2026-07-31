# Structured Intent Prompt

Template: 1.0.0

Issue: 5686

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Land the canonical receipt-matching #5662 terminal projection in main through a governed repair PR.

## Required Outcome

Current main gains the exact #5662 closed-out projection without implementation changes, receipt mutation, or direct writes on main.

## Scope

- Tracked .csdlc issue projection for #5662
- Tracked publication intent for #5662
- Issue-local lifecycle records for #5686

## Authority

- #5686 may project immutable #5662 terminal truth but may not change #5662 implementation behavior
- The canonical closeout receipt is read-only authority
- Main is changed only by reviewed PR merge

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 lifecycle only
- No tracked writes on main
- FastWork issue-bound worktree only
- No raw gh or browser mutation
- No binary builds
- Capture tooling anomalies in issues
