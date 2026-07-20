# Structured Review Prompt

Template: 1.0.0

Issue: 5306

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

AGENTS.md
.github/workflows/ci.yaml
adl
csdlc-v2

## Prompts

- Verify every removed path is in the approved exact manifest.
- Verify useful retained code and both sunset surfaces remain.
- Verify LoC/test claims are measured rather than budget-driven.

## Findings

[
  {
    "id": "F-5306-1",
    "severity": "p1",
    "summary": "The current csdlc-v2 init skill retained stale guidance to preserve the v1 default after final Gate 10D2 sunset.",
    "actionable": true,
    "in_scope": false,
    "disposition": "out_of_scope",
    "fix_revision": null,
    "route": "Resolved by #5541 / merged PR #5568 at head 8e206363ee75f5517862a03d8cc5dc07efa4128a"
  },
  {
    "id": "F-5306-2",
    "severity": "p1",
    "summary": "The original default workflow routed through deleted pr.sh and lacked an executable guard against restored v1 routes.",
    "actionable": true,
    "in_scope": false,
    "disposition": "out_of_scope",
    "fix_revision": null,
    "route": "Resolved by #5541 / merged PR #5568 at head 8e206363ee75f5517862a03d8cc5dc07efa4128a"
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:2b11155b6a4abaf9348cc1cfd147c9b9c676a56c:75fcce5a37a1155bbf05e41c881b4c74620359254414539645ada01e8b5f1072")

Reviewer: Some("subagent:019f669a-596c-71e2-adb3-bd753875989d")

Result: pass
