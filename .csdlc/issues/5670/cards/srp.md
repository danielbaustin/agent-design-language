# Structured Review Prompt

Template: 1.0.0

Issue: 5670

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

.github/workflows/ci.yaml
adl/tools/run_authoritative_coverage_lane.sh
adl/tools/test_run_authoritative_coverage_lane.sh
adl/tools/test_ci_runtime_contracts.sh
adl/tools/test_ci_path_policy.sh
.csdlc/evidence/5670
.csdlc/issues/5670
.csdlc/prepared/issues/5670

## Prompts

- Does the workflow make hosted coverage faster by parallelizing producers while keeping one authoritative final gate?
- Can any shard or aggregation path silently skip required coverage evidence?
- Are run ids, profile roots, output roots, logs, and artifacts isolated per shard?
- Do focused contracts prove topology and failure behavior without overfitting?
- Is the change proportional and free of AWS/product/runtime scope?

## Findings

[
  {
    "id": "F-5670-1",
    "severity": "p1",
    "summary": "Report-only aggregation could include compile-only or build-script profraw before shard import.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:faa3dfdee98540cdf9ad020e08368702693e9839:e5a41237348e8b4632ab5812b319319a68e66c149cba2912488e6d95d0adbb7c",
    "route": null
  },
  {
    "id": "F-5670-2",
    "severity": "p2",
    "summary": "Non-full PR-fast coverage used the four-shard topology and shard_count provenance.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:faa3dfdee98540cdf9ad020e08368702693e9839:e5a41237348e8b4632ab5812b319319a68e66c149cba2912488e6d95d0adbb7c",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted GitHub Actions artifact fan-in has not run locally; the workflow and contract tests cover shape and CI must provide hosted integration proof after publication.

## Review Result

Revision: Some("git-blake3:faa3dfdee98540cdf9ad020e08368702693e9839:e5a41237348e8b4632ab5812b319319a68e66c149cba2912488e6d95d0adbb7c")

Reviewer: Some("subagent:019f9c5d-654d-7af1-ab7a-f205c27ca0b2")

Result: pass
