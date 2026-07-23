# Structured Intent Prompt

Template: 1.0.0

Issue: 5624

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make guarded prune accept the supported issue-local worktree sentinel only for the exact safe checkout.

## Required Outcome

Guarded prune uses repository-rooted canonical identity, accepts truthful issue-local terminal topology, rejects ambiguous or unsafe checkouts, and never rewrites retained terminal receipts.

## Scope

- Canonical terminal worktree resolution in csdlc-v2 readiness
- Focused library and command-level prune regressions
- Issue-local typed lifecycle and validation evidence

## Authority

- Issue 5624 owns only readiness.rs, focused prune tests, and issue-local records
- Terminal records and retained receipts remain immutable authority
- Issue 5340, Runtime code, and unrelated lifecycle behavior remain outside scope

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 only
- Work only in /Volumes/FastWork/adl-wp-5624
- No raw gh, AWS, Runtime changes, #5340 mutation, or root-main edits
- No approval pauses; complete the authorized issue end to end
