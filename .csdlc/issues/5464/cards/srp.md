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
    "id": "F-5464-4",
    "severity": "p2",
    "summary": "Quoted installer scalars and fully inline YAML steps escape line-oriented install-action inventory.",
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

Revision: Some("git-blake3:aaad5ad6b7ce63f267f0dde29b911f9db4861626:eb60f68256fa7d3558fa00e01cbd78d67bda47d2ca66c74606828a3cd2194e5d")

Reviewer: Some("bounded-subagent-review-5464")

Result: changes_required
