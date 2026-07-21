# Structured Output Record

Template: 1.0.0

Issue: 5614

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Replaced the tracked AWS temporary-key-shaped literal with runtime construction from non-secret fragments while preserving the sanitizer assertion.

## Artifacts

- secret-scanning alert #1
- commit 7d302ba36

## Execution

- adl/tools/test_run_aws_spot_ci_profile.sh

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_aws_spot_ci_profile.sh"
    ],
    "purpose": "Prove runtime AWS temporary-key-shaped data is sanitized without any AWS call.",
    "outcome": "passed",
    "evidence_ref": "local exit 0 on commit 7d302ba36"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_aws_spot_artifact_finalize.sh"
    ],
    "purpose": "Prove coupled artifact redaction and finalization behavior remains green.",
    "outcome": "passed",
    "evidence_ref": "local PASS on commit 7d302ba36"
  }
]

## Integration

pr_open

## Publication

Publication: draft

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
