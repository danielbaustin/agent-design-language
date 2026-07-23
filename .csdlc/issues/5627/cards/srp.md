# Structured Review Prompt

Template: 1.0.0

Issue: 5627

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src
csdlc-v2/tests
csdlc-v2/operator/skills

## Prompts

- Can validation failure alter index, cards, audit, or generation?
- Can review record accept stale or out-of-scope evidence without assignment?
- Can direct publication record a different repository, branch, SHA, or draft state?
- Does active-draft compatibility remain bounded to existing draft records?
- Do command and artifact measurements match the executable four-command proof?

## Findings

[
  {
    "id": "prebuilt-provenance",
    "severity": "p1",
    "summary": "Do not stamp arbitrary prebuilt binaries as the current Git revision.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:19eddfa3e94e8ad09168ca4e30faec7471b23c18:25036cb4a79276db85bb971bc68038990fee02b589bdab628191d44eabd04296",
    "route": null
  },
  {
    "id": "gate10a-evidence",
    "severity": "p2",
    "summary": "Retain the exact Gate 10A CI-repair validation result.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:19eddfa3e94e8ad09168ca4e30faec7471b23c18:25036cb4a79276db85bb971bc68038990fee02b589bdab628191d44eabd04296",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:19eddfa3e94e8ad09168ca4e30faec7471b23c18:25036cb4a79276db85bb971bc68038990fee02b589bdab628191d44eabd04296")

Reviewer: Some("operator:codex-5627-integrated-disposition")

Result: pass
