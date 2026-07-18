# Structured Review Prompt

Template: 1.0.0

Issue: 5440

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/store.rs
csdlc-v2/tests/gate2.rs

## Prompts

- Verify review authority cannot survive a later-phase design change
- Verify both design and diagram digests refresh atomically
- Verify audit and generation truth

## Findings

[
  {
    "id": "P1-projection-drift",
    "severity": "p1",
    "summary": "Reject unrelated card projection drift during design reapproval",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:2d70cf48fc39d5ae900a78c83f82d5cccc20031c:8ff617d72b90bd6a8e3602e9d60685ee8ae8ded4b5da7f4dce176d9d342faf63",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:2d70cf48fc39d5ae900a78c83f82d5cccc20031c:8ff617d72b90bd6a8e3602e9d60685ee8ae8ded4b5da7f4dce176d9d342faf63")

Reviewer: Some("codex-subagent-019f6d31")

Result: pass
