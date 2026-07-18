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
    "id": "F-5464-2",
    "severity": "p2",
    "summary": "Unversioned nextest and cargo-nextest tool aliases escape the @-based inventory.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5464-3",
    "severity": "p3",
    "summary": "The alternate cargo-nextest alias has no negative fixture.",
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

Revision: Some("git-blake3:e2574403da44cda7f6e5f7ec31a2b92a97147e81:f8549e0b1836ee40d5245827e32b1ca8791e6b0c56bdadf7840f3d5463f14b2a")

Reviewer: Some("bounded-subagent-review-5464")

Result: changes_required
