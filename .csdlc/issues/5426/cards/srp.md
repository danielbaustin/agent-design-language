# Structured Review Prompt

Template: 1.0.0

Issue: 5426

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/cards.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/gate7_lifecycle.rs
docs/reviews/v0.91.7/csdlc-v2-5426

## Prompts

- Is validation identity narrow and deterministic?
- Do pass-after-waiting and fail-after-pass behave correctly?
- Is append-only evidence preserved?
- Are all terminal validation call sites consistent?

## Findings

[
  {
    "id": "F-5426-1",
    "severity": "p1",
    "summary": "Terminal closeout bypassed latest validation state",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:d1ed65343d71bcf55274affb1fb731e1dd9ad2a9:e809f3c46311951ca0ca80d86fbc9e6665adbdf7549fbddfb7c07d365a257ba2",
    "route": null
  },
  {
    "id": "F-5426-2",
    "severity": "p2",
    "summary": "Merged regression used an invalid closed observation and did not reach the repaired guard",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:d1ed65343d71bcf55274affb1fb731e1dd9ad2a9:e809f3c46311951ca0ca80d86fbc9e6665adbdf7549fbddfb7c07d365a257ba2",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:d1ed65343d71bcf55274affb1fb731e1dd9ad2a9:e809f3c46311951ca0ca80d86fbc9e6665adbdf7549fbddfb7c07d365a257ba2")

Reviewer: Some("subagent-huygens")

Result: pass
