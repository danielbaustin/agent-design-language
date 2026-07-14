# Structured Review Prompt

Template: 1.0.0

Issue: 5353

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/lifecycle.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/gate2.rs

## Prompts

- Verify issue-local paths cannot create a false existing-record condition.
- Verify both design and diagram digests refresh atomically.
- Verify tests do not widen into ADL or Runtime code.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:445608da09701fd594851196ef891dfbab424671:3bc51cc26064257bf5414f455894263db3e70b962c7853a63ac1fbc30a13dd2d")

Reviewer: Some("subagent-reviewer")

Result: pass
