# Structured Intent Prompt

Template: 1.0.0

Issue: 5710

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make typed C-SDLC v2 closeout and prune recovery complete, evidence-preserving, deterministic, and usable across already-closed v0.91.8 issues.

## Required Outcome

The typed closeout surface reconciles causally valid terminal head drift, safely cleans only proved generated state, reports legal repair actions for earlier lifecycle phases, and enables every eligible closed worktree to prune.

## Scope

- Typed terminal publication reconciliation
- Typed dirty-worktree classification and safe cleanup before prune
- Read-only closed-issue lifecycle repair classification
- Focused closeout/prune tests
- v0.91.8 closeout recovery evidence and sweep report

## Authority

- Issue #5710 owns only its issue-local records, declared C-SDLC v2 closeout sources/tests, and closeout recovery proof packet
- The existing validate-prune and receipt-backed prune operations remain final removal gates
- No unrelated issue lifecycle record may be changed during implementation validation

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 lifecycle only
- No tracked edits on main
- No force prune or destructive manual cleanup
- No manual card or .csdlc state editing
- Use repo-local request files under .git/csdlc-v2/requests
- Publish only after exact-head subagent review and focused validation
