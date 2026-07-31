# Structured Output Record

Template: 1.0.0

Issue: 5719

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Mapped podcast launch/studio demo surfaces into a focused podcast_launch_packet validation lane and taught ci_path_policy to consume that lane without selecting full hosted coverage. Added a #5716-like regression fixture proving podcast static/demo page paths skip full coverage while preserving the coverage aggregator and existing Rust/tooling behavior.

## Artifacts

- adl/tools/test_ci_path_policy.sh
- adl/tools/test_ci_runtime_contracts.sh

## Execution

- .csdlc/issues/5719
- .csdlc/locks/5719.lock
- .csdlc/prepared/issues/5719
- adl/config/validation_lane_selector.v0.91.6.json
- adl/tools/ci_path_policy.sh
- adl/tools/test_ci_path_policy.sh

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_ci_path_policy.sh"
    ],
    "purpose": "Prove podcast launch/studio page changes select focused podcast validation and do not require full hosted runtime plus workspace coverage.",
    "outcome": "passed",
    "evidence_ref": "ci-path-policy-podcast-launch-contract.log"
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
