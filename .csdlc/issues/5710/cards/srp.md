# Structured Review Prompt

Template: 1.0.0

Issue: 5710

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/bin/csdlc-closeout.rs
csdlc-v2/src/git.rs
csdlc-v2/src/lifecycle.rs
csdlc-v2/tests/gate7_lifecycle.rs
docs/milestones/v0.91.8/review/closeout_recovery
.csdlc/evidence/5710
.csdlc/issues/5710

## Prompts

- Can terminal reconciliation ever accept a different repository, PR, branch, or unrelated revision?
- Can prune preparation remove tracked lifecycle drift, source files, unknown paths, or unretained evidence?
- Are cleanup and reconciliation idempotent and audit-preserving?
- Does the classifier report legal next actions without mutating lifecycle state?
- Do focused tests cover the #5691 drift and representative dirty-worktree classes?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
