# Structured Review Prompt

Template: 1.0.0

Issue: 5861

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2
docs/architecture/csdlc-v2
docs/tooling/C_SDLC_V2_ISSUE_PREPARATION_AND_BINDING_RUNBOOK.md
.csdlc/issues/5861
.csdlc/evidence/5861
.csdlc/prepared/issues/5861

## Prompts

- Does every state and command have one truthful authority and next operation?
- Can preparation remain fully editable and claim-free without weakening bind-time overlap safety?
- Are receipt digest fields semantic, complete, and stable under non-semantic tracker changes?
- Are bind, release, recovery, and compensation linearizable and ownership-safe at every crash point?
- Does migration preserve valid active claims and offer a bounded audited repair for ambiguity?
- Do batch outcomes preserve child truth without claiming all-or-nothing atomicity?
- Can the coupled legacy route be deleted after focused parity without hidden compatibility behavior?

## Findings

[
  {
    "id": "CLAUDE-WINDOWS-SYNC-CFG",
    "severity": "p1",
    "summary": "Windows directory sync used an early return that left unreachable code under strict warnings.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:ae98c6f4957fa905194a80ff028fbccb64909ff5:309e94bac7fca2cf0df97aef93a323bd360f056e1215ebc2e842669205db9f61",
    "route": null
  },
  {
    "id": "CLAUDE-ROOT-BOUND-RETRY-TOPOLOGY",
    "severity": "p1",
    "summary": "In-place bound retry did not symmetrically verify the durable intent branch and registered root worktree.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:ae98c6f4957fa905194a80ff028fbccb64909ff5:309e94bac7fca2cf0df97aef93a323bd360f056e1215ebc2e842669205db9f61",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Windows cross-target compilation was not available locally; cfg-split code was reviewed by Gemini and strict host all-target lint passed.
- Two final Claude rerun calls returned empty provider output at HTTP 200; the preceding successful Claude findings were remediated and the post-fix Gemini and independent reviews reported no actionable findings.

## Review Result

Revision: Some("git-blake3:ae98c6f4957fa905194a80ff028fbccb64909ff5:309e94bac7fca2cf0df97aef93a323bd360f056e1215ebc2e842669205db9f61")

Reviewer: Some("multi-model:claude-opus-5+gemini-3.1-pro+subagent")

Result: pass
