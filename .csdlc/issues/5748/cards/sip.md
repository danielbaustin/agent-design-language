# Structured Intent Prompt

Template: 1.0.0

Issue: 5748

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Recover truthful typed terminal state for every closed v0.91.8 issue excluded from the clean #5746 projection wave.

## Required Outcome

Classify every live GitHub-closed version:v0.91.8 issue against exact merged-PR and C-SDLC v2 evidence; materialize receipt-backed closed_out projections for every issue whose evidence permits terminal truth, and preserve an explicit fail-closed exception instead of inventing a receipt for any issue with unresolved exact-head implementation, review, authority, or eligibility blockers.

## Scope

- typed receipt recovery in issue-local worktrees
- receipt-backed terminal projection materialization in the dedicated #5748 worktree
- special disposition and retained-artifact repairs explicitly named by issue #5748
- issue-local evidence under .csdlc/evidence/5748

## Authority

- Issue-local worktrees own receipt creation and repair; #5748 materializes only validated retained authority.
- Generated cards, index records, and terminal receipts change only through typed C-SDLC v2 operations.
- Do not modify or prune the active #5746 worktree, and never write tracked changes on main; receipt-backed #5746 projection reconciliation is permitted only on the dedicated #5748 closeout branch.

## Assumptions

- none

## Operator Constraints

- never write tracked changes on main
- do not modify or prune the active #5746 issue worktree; receipt-backed terminal reconciliation on the dedicated #5748 closeout branch is allowed
- use typed C-SDLC v2 for every lifecycle mutation
- preserve dirty worktrees
- do not use AWS; GitHub observations must be read-only and every GitHub mutation must use the typed C-SDLC v2 route
