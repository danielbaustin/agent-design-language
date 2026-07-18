# Structured Output Record

Template: 1.0.0

Issue: 5512

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Replace the invalid mixed coverage expression with valid ADL and Runtime v3 owning-crate expressions only for the bounded bridge route.

## Artifacts

- adl/tools/run_pr_fast_coverage_lane.sh
- adl/tools/test_run_pr_fast_coverage_lane.sh

## Execution

- Detect the already bounded Runtime v3 and CSM bridge selector family
- Run the ADL coverage invocation with only CSM, long-lived-agent, and csmctl selectors
- Run auth, supervision, and topology selectors only through adl-runtime
- Use the exact failed GitHub expression as the regression fixture

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_pr_fast_coverage_lane.sh"
    ],
    "purpose": "Prove owning-crate filter splitting, foreign-selector exclusion, and composed summaries",
    "outcome": "passed",
    "evidence_ref": "local:5512-exact-29644007246-expression-contract-pass"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_path_policy.sh"
    ],
    "purpose": "Prove the narrow repair preserves existing validation routing contracts",
    "outcome": "passed",
    "evidence_ref": "local:5512-ci-path-policy-contract-pass"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "purpose": "Prove the CI contract accepts the reviewed owning-crate coverage variable",
    "outcome": "passed",
    "evidence_ref": "local:5512-ci-runtime-contract-pass"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
