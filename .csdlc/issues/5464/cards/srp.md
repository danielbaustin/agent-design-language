# Structured Review Prompt

Template: 1.0.0

Issue: 5464

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

.github/workflows/ci.yaml
adl/tools/test_ci_runtime_contracts.sh

## Prompts

- Are all nextest install steps updated?
- Does every step fail closed instead of falling back?
- Does the static contract detect partial or future drift?
- Is the hosted warning genuinely absent?

## Findings

[
  {
    "id": "F-5464-1",
    "severity": "p2",
    "summary": "Unnamed, quoted, or inline YAML nextest steps can bypass the block-only static contract.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:6f72414262eb807200379440c13580aa43651f02:1fd9200c4b6534ed30f7ec17d25546ae8ec0cd2b1aa8707fd708f25c0a012a49")

Reviewer: Some("bounded-subagent-review-5464")

Result: changes_required
