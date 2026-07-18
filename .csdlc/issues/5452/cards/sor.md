# Structured Output Record

Template: 1.0.0

Issue: 5452

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Made retained summary publication atomic and fail-closed after the initial review.

## Artifacts

- adl/tools/run_aws_spot_builder_image_validation.sh
- adl/tools/test_run_aws_spot_builder_image_validation.sh
- adl/tools/run_aws_spot_builder_image_validation.sh
- adl/tools/test_run_aws_spot_builder_image_validation.sh

## Execution

- Capture validation status without aborting before summary generation
- Record validation status in the retained summary and return non-zero for either failed stage
- Add success, mixed-result, and combined-failure regressions through executable fakes
- Remove stale final summary evidence before regeneration
- Write through a sibling temporary file and atomically publish only complete JSON
- Suppress the proof marker and clean partial temporary output when summary generation fails
- Add regression coverage for stale final evidence and partial temporary output

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_aws_spot_builder_image_validation.sh"
    ],
    "purpose": "Prove primary success plus summary failure, primary failure plus summary success, combined failure, and successful execution",
    "outcome": "passed",
    "evidence_ref": "PASS test_run_aws_spot_builder_image_validation"
  },
  {
    "command": [
      "bash",
      "-n",
      "adl/tools/run_aws_spot_builder_image_validation.sh",
      "adl/tools/test_run_aws_spot_builder_image_validation.sh"
    ],
    "purpose": "Prove both touched shell scripts parse and the patch has clean whitespace",
    "outcome": "passed",
    "evidence_ref": "bash -n and git diff --check passed"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_aws_spot_ci_profile.sh"
    ],
    "purpose": "Prove the repaired wrapper remains correctly selected and invoked by the Spot CI profile contract",
    "outcome": "passed",
    "evidence_ref": "local focused contract run exited 0"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_aws_spot_builder_image_validation.sh"
    ],
    "purpose": "Prove stale final evidence removal, partial-temp cleanup, proof-marker suppression, mixed-result precedence, and successful publication",
    "outcome": "passed",
    "evidence_ref": "PASS test_run_aws_spot_builder_image_validation"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_aws_spot_ci_profile.sh"
    ],
    "purpose": "Check the adjacent Spot CI profile contract without remote or AWS execution",
    "outcome": "failed",
    "evidence_ref": "pre-existing base mismatch: test expects CapEff/NoNewPrivs/permission-probe and gh checks absent from the checked-in builder validator"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_aws_spot_ci_profile.sh"
    ],
    "purpose": "Check the adjacent Spot CI profile contract without remote or AWS execution",
    "outcome": "skipped_non_goal",
    "evidence_ref": "pre-existing base mismatch: test expects CapEff/NoNewPrivs/permission-probe and gh checks absent from the checked-in builder validator"
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
