# Structured Task Prompt

Template: 1.0.0

Issue: 5710

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement and prove the smallest typed recovery surface that closes the identified terminal reconciliation and dirty-prune gaps, then run the live v0.91.8 sweep after merge.

## Deliverables

- Typed closeout reconciliation request/result contract
- Typed prune-preparation classification and cleanup contract
- Closed-issue lifecycle repair classifier
- Focused deterministic tests
- Tracked recovery proof packet and post-merge sweep report

## Acceptance

1. AC-1: #5691-style recorded-head versus merged-head drift is accepted only with exact repository, PR, branch, and ancestry proof
2. AC-2: unrelated or ambiguous terminal revisions fail closed
3. AC-3: safe cleanup removes only explicit generated categories or byte-equivalent retained evidence
4. AC-4: tracked lifecycle drift, source files, and unknown paths are never silently removed
5. AC-5: the operation is repeatable and existing validate-prune/prune gates remain authoritative
6. AC-6: closed earlier-phase issues receive a truthful next-typed-action classification without inferred transitions
7. AC-7: focused tests and exact-head review pass
8. AC-8: the live v0.91.8 sweep reports reconciled, pruned, and blocked issues separately

## Dependencies

- Current C-SDLC v2 closeout receipt and prune implementation
- Existing closed v0.91.8 issue/worktree inventory
- Live merged PR truth from the typed GitHub owner binary

## Inputs

- GitHub issue #5710
- csdlc-v2/src/bin/csdlc-closeout.rs
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/git.rs
- csdlc-v2/tests/gate7_lifecycle.rs
- Issue #5691 and PR #5694 reconciliation evidence
- Closed v0.91.8 worktree dirty-state inventory

## Non Goals

- No force removal of dirty worktrees
- No v1 wrapper restoration
- No broad repository cleanup
- No inferred review, publication, readiness, or terminal success
