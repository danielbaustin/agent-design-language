# Structured Review Prompt

Template: 1.0.0

Issue: 5727

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

csdlc-v2/src/bin/csdlc-bind.rs
csdlc-v2/src/doctor.rs
csdlc-v2/src/lib.rs
csdlc-v2/src/lifecycle.rs
csdlc-v2/src/schema.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/gate2.rs

## Prompts

- Can any reacquisition path bypass compare-and-swap identity or protected-path collision checks?
- Does dormant doctor classification remain read-only and keep mutations claim-gated?
- Are phase preservation and append-only audit history proven for released and expired claims?
- Does the #5354 reproduction use only the typed binary without direct record edits?

## Findings

[
  {
    "id": "5727-review-p1-cross-worktree-reacquire",
    "severity": "p1",
    "summary": "Reacquisition initially used checkout-local collision authority.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:8f74f48e75af861c517e6c8e67d1e4c9f6e6ff3d:ecf8559558a51c5f4cc0088ec66151963b29d245d5959192a1f9055db5a8aeaf",
    "route": null
  },
  {
    "id": "5727-review-p1-recover-bypass",
    "severity": "p1",
    "summary": "Expired recovery initially bypassed shared collision authority.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:8f74f48e75af861c517e6c8e67d1e4c9f6e6ff3d:ecf8559558a51c5f4cc0088ec66151963b29d245d5959192a1f9055db5a8aeaf",
    "route": null
  },
  {
    "id": "5727-review-p1-wrong-checkout-recovery",
    "severity": "p1",
    "summary": "Expired recovery initially did not validate the actual checkout identity.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:8f74f48e75af861c517e6c8e67d1e4c9f6e6ff3d:ecf8559558a51c5f4cc0088ec66151963b29d245d5959192a1f9055db5a8aeaf",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:8f74f48e75af861c517e6c8e67d1e4c9f6e6ff3d:ecf8559558a51c5f4cc0088ec66151963b29d245d5959192a1f9055db5a8aeaf")

Reviewer: Some("codex-subagent:/root/review_5727")

Result: pass
