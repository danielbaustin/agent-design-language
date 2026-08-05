# Structured Review Prompt

Template: 1.0.0

Issue: 5692

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

AGENTS.md
csdlc-v2/src/bin/csdlc-publish.rs
csdlc-v2/src/github.rs
csdlc-v2/src/publication.rs
csdlc-v2/tests/gate6.rs

## Prompts

- Review only AGENTS.md closing-keyword policy wording, csdlc-v2 publication body validation, and focused tests. Findings first; no workflow rewrite.

## Findings

[
  {
    "id": "5692-P1-pending-duplicate-check-run-freshness",
    "severity": "p1",
    "summary": "Duplicate check-run selection must let a newer pending duplicate supersede a stale completed duplicate even when started_at is absent.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:45707a038c79fdbc9a1b60382a6e40bec12f9b4f:c0156b85618fdfc1b5cda630da3c40dcc940c199e62252a6b0125556b4497173",
    "route": null
  },
  {
    "id": "5692-P2-stale-publication-intent-after-recovery",
    "severity": "p2",
    "summary": "Recovered implemented state must not carry a stale publication intent pinned to an older head.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:45707a038c79fdbc9a1b60382a6e40bec12f9b4f:c0156b85618fdfc1b5cda630da3c40dcc940c199e62252a6b0125556b4497173",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- GitHub CI must rerun on the republished exact head before merge readiness can be recorded again.
- Publication will regenerate .csdlc/publication/5692.intent.json for the reviewed exact head.

## Review Result

Revision: Some("git-blake3:45707a038c79fdbc9a1b60382a6e40bec12f9b4f:c0156b85618fdfc1b5cda630da3c40dcc940c199e62252a6b0125556b4497173")

Reviewer: Some("bounded-subagent:019fa486-fda3-7992-b8d6-1a804c046ffd")

Result: pass
