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
    "id": "F-5464-5",
    "severity": "p2",
    "summary": "Comma- and whitespace-separated multi-tool selections can hide nextest from whole-value matching.",
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

Revision: Some("git-blake3:563c796a1b2db53165a8d055192ed172a5a29f44:64aa68041155f50dad8c84c7ab253ea5786c3ccc4d4508b5f5a0200f2d0682d1")

Reviewer: Some("bounded-subagent-review-5464")

Result: changes_required
