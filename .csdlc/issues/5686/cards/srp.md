# Structured Review Prompt

Template: 1.0.0

Issue: 5686

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

.csdlc/issues/5662
.csdlc/publication/5662.intent.json
.csdlc/evidence/5686
.csdlc/issues/5686
.csdlc/locks/5686.lock
.csdlc/prepared/issues/5686

## Prompts

- Does the resulting #5662 record exactly match the canonical terminal receipt?
- Is the diff limited to terminal projection and #5686 lifecycle truth?
- Did any implementation or canonical receipt content change?
- Does the PR target current main?

## Findings

[
  {
    "id": "R5686-1",
    "severity": "p2",
    "summary": "Verifier inspected only worktree status and missed committed repair paths.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:ff14deb06b8de6721ae22bf75a1816d4e0f8b156:009d937602b4463bed87c24c9a7f8b5f929a22c2c2ee89e14557d81393289353",
    "route": null
  },
  {
    "id": "R5686-2",
    "severity": "p2",
    "summary": "Broad projection prefixes allowed additional unverified #5662 files.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:ff14deb06b8de6721ae22bf75a1816d4e0f8b156:009d937602b4463bed87c24c9a7f8b5f929a22c2c2ee89e14557d81393289353",
    "route": null
  },
  {
    "id": "R5686-3",
    "severity": "p2",
    "summary": "Parity proof depended on locally reachable retained commit objects.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:ff14deb06b8de6721ae22bf75a1816d4e0f8b156:009d937602b4463bed87c24c9a7f8b5f929a22c2c2ee89e14557d81393289353",
    "route": null
  },
  {
    "id": "R5686-4",
    "severity": "p2",
    "summary": "Parity proof followed mutable origin/main state and would collapse after merge.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:ff14deb06b8de6721ae22bf75a1816d4e0f8b156:009d937602b4463bed87c24c9a7f8b5f929a22c2c2ee89e14557d81393289353",
    "route": null
  },
  {
    "id": "R5686-5",
    "severity": "p1",
    "summary": "Primary parity lane required an untracked Git-common-directory receipt absent from fresh clones.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:ff14deb06b8de6721ae22bf75a1816d4e0f8b156:009d937602b4463bed87c24c9a7f8b5f929a22c2c2ee89e14557d81393289353",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:ff14deb06b8de6721ae22bf75a1816d4e0f8b156:009d937602b4463bed87c24c9a7f8b5f929a22c2c2ee89e14557d81393289353")

Reviewer: Some("subagent:codex-review:019fa160-30d6-7f40-b673-3ae44ea0bd3d")

Result: pass
