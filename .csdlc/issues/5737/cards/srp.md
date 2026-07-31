# Structured Review Prompt

Template: 1.0.0

Issue: 5737

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

csdlc-v2/src/lifecycle.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/gate2.rs
.csdlc/evidence/5737

## Prompts

- Check that terminal receipt identity is only consulted after a protected-path overlap is detected.
- Check that stale projections whose claim branch/worktree do not match their active checkout are ignored for collision authority.
- Check that authority-only reacquisition preserves card identity and does not bypass typed design reapproval.
- Check that live overlapping claims still fail closed and tests cover the regression without weakening safety.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Only claims matching a registered active checkout have collision authority; stale tracked projections remain retained evidence but no longer reserve paths.

## Review Result

Revision: Some("git-blake3:4079e840d35a287eefbc637718cc6c1dafa4af77:65c11bb45e0642a62bce15abd24f4eb99769ae107ea9359cd9bb2792cdf19182")

Reviewer: Some("codex:gpt-5.5-claim-authority-review")

Result: pass
