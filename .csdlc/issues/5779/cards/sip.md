# Structured Intent Prompt

Template: 1.0.0

Issue: 5779

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Separate worktree cleanup from terminal delivery truth while preserving dirty work and legacy audit readability.

## Required Outcome

A standalone csdlc-clean removes only clean exactly identified issue worktrees, skips dirty or drifted topology non-destructively, and indexes legacy receipts as compatibility-only inputs.

## Scope

- Standalone cleanup command and typed result schema
- Dynamic branch and worktree discovery independent of receipt topology
- Legacy terminal receipt compatibility index and migration validation
- Deterministic clean, dirty, missing, relocated, and concurrent cleanup proofs

## Authority

- Cleanup never changes terminal or delivery truth
- Dirty, missing, relocated, or ambiguous worktrees are reported and never force-removed
- Legacy receipts remain immutable compatibility evidence and never become current authority

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 only
- No tracked edits on main
- No AWS or external archive service
- No forced cleanup or deletion of dirty work
- Exact-head independent review before publication
