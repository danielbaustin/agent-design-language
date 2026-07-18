# Structured Review Prompt

Template: 1.0.0

Issue: 5514

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl/tools/run_pr_fast_coverage_lane.sh
adl/tools/test_run_pr_fast_coverage_lane.sh

## Prompts

- Does the ADL partition preserve every valid selector from the canonical expression?
- Can a foreign selector reach either workspace?
- Does exact matching still reject near matches?
- Does the regression prove command completeness and summary composition?

## Findings

[
  {
    "id": "F-5514-1",
    "severity": "p1",
    "summary": "The initial partition placed 17 cli::csm_cmd tests under the ADL library test binary, selecting none of them.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:b77a18257d67e50de64bf7563eb054213d7e005f:bd4f3f160b735d68364732e8ee79f362b61517c81590b29cf35db96c62900ce4",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- PR #5504 remains the final changed-source coverage proof after the tooling repair merges.

## Review Result

Revision: Some("git-blake3:b77a18257d67e50de64bf7563eb054213d7e005f:bd4f3f160b735d68364732e8ee79f362b61517c81590b29cf35db96c62900ce4")

Reviewer: Some("subagent:019f755e-caed-72b0-b9fb-21366bf78332")

Result: pass
