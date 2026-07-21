# Structured Task Prompt

Template: 1.0.0

Issue: 5614

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Replace the literal with runtime construction and retain exact sanitizer assertions.

## Deliverables

- Scanner-safe fixture
- Passing focused redaction tests
- Resolved alert #1 after merge

## Acceptance

1. AC-1: no tracked literal matches the temporary AWS access-key ID pattern
2. AC-2: runtime fixture still exercises access-key redaction
3. AC-3: focused sanitizer tests pass
4. AC-4: alert #1 is resolved as test-only after merge

## Dependencies

- Secret-scanning alert #1

## Inputs

- adl/tools/test_run_aws_spot_ci_profile.sh
- adl/tools/test_aws_spot_artifact_finalize.sh

## Non Goals

- No AWS calls
- No history rewrite
- No production sanitizer changes
