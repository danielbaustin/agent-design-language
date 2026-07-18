# Structured Review Prompt

Template: 1.0.0

Issue: 5440

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

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
    "fix_revision": "git-blake3:77fafde96e9681b1b4871d1c033de532adc7df32:b4bda44216c140ff6b6356a8296c4d906b28e279471c9e883bbd4f6002fa7cf5",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:77fafde96e9681b1b4871d1c033de532adc7df32:b4bda44216c140ff6b6356a8296c4d906b28e279471c9e883bbd4f6002fa7cf5")

Reviewer: Some("codex-subagent-019f6d31")

Result: pass
