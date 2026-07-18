# Structured Review Prompt

Template: 1.0.0

Issue: 5514

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

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
    "fix_revision": "git-blake3:6787d12a21a81e99bd441590ccd113c220a43a60:f517ce58bc10d33d441404b4b40a108b50b0a65f5369223270115e4a66c98abc",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- PR #5504 remains the final changed-source coverage proof after the tooling repair merges.

## Review Result

Revision: Some("git-blake3:6787d12a21a81e99bd441590ccd113c220a43a60:f517ce58bc10d33d441404b4b40a108b50b0a65f5369223270115e4a66c98abc")

Reviewer: Some("subagent:019f755e-caed-72b0-b9fb-21366bf78332")

Result: pass
