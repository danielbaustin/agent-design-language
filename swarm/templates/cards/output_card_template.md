# ADL Output Card

Task ID:
Run ID:
Version:
Title:
Branch:
Status: NOT_STARTED | IN_PROGRESS | DONE | FAILED

Execution:
- Actor:
- Model:
- Provider:
- Start Time:
- End Time:

## Summary

## Artifacts produced
- 

## Actions taken
- 

## Validation
- Tests / checks run:
- Results:

## Verification Summary
```yaml
verification_summary:
  validation:
    status: PASS | FAIL | PARTIAL | NOT_RUN
    checks_run:
      - ""
  determinism:
    status: PASS | FAIL | PARTIAL | NOT_RUN
    replay_verified: true | false | unknown
    ordering_guarantees_verified: true | false | unknown
  security_privacy:
    status: PASS | FAIL | PARTIAL | NOT_RUN
    secrets_leakage_detected: true | false | unknown
    prompt_or_tool_arg_leakage_detected: true | false | unknown
    absolute_path_leakage_detected: true | false | unknown
  artifacts:
    status: PASS | FAIL | PARTIAL | NOT_RUN
    required_artifacts_present: true | false | unknown
    schema_changes:
      present: true | false | unknown
      approved: true | false | not_applicable | unknown
```

## Determinism Evidence
- Determinism tests executed:
- Replay verification (same inputs -> same artifacts/order):
- Ordering guarantees (sorting / tie-break rules used):
- Artifact stability notes:

## Security / Privacy Checks
- Secret leakage scan performed:
- Prompt / tool argument redaction verified:
- Absolute path leakage check:
- Sandbox / policy invariants preserved:

## Replay Artifacts
- Trace bundle path(s):
- Run artifact root:
- Replay command used for verification:
- Replay result:

## Artifact Verification
- Required artifacts present:
- Artifact schema/version checks:
- Hash/byte-stability checks:
- Missing/optional artifacts and rationale:

## Decisions / Deviations

## Follow-ups / Deferred work
