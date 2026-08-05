# Structured Output Record

Template: 1.0.0

Issue: 5678

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Repair the tracked Opus review runbook and add a source-backed structured CLI drift check.

## Artifacts

- docs/tooling/OPUS_REVIEW_RUNBOOK.md
- adl/tools/test_opus_review_runbook.sh

## Execution

- Replace stale flag-form invocation with --request/--out/--log JSON invocation
- Document exact-head review evidence and provider identity truth boundaries
- Add jq-backed runbook contract test and current adapter help verification

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_opus_review_runbook.sh"
    ],
    "purpose": "Run the focused runbook contract suite",
    "outcome": "passed",
    "evidence_ref": "opus-runbook-contract.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Run whitespace validation",
    "outcome": "passed",
    "evidence_ref": "opus-runbook-diff-hygiene.log"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
