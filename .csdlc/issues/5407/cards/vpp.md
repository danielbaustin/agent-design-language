# Validation Planning Prompt

Template: 1.0.0

Issue: 5407

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5407/retained/design.md

Diagram: .csdlc/issues/5407/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "patch-integrity",
    "proof_role": "Prove the documentation patch has no whitespace or conflict-marker defects",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "logging-scope-truth",
    "proof_role": "Confirm validation-manager is named as the implemented build-action-log producer",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 200,
    "argv": [
      "rg",
      "-n",
      "validation_manager.py --run",
      "docs/tooling/BUILD_ACTION_LOGS.md"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "typed-v2-authority",
    "proof_role": "Confirm the CLI taxonomy names Gate 10D2 typed-v2 authority",
    "acceptance_ids": [
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 200,
    "argv": [
      "rg",
      "-n",
      "Gate 10D2|csdlc-v2",
      "docs/tooling/ADL_PLATFORM_CLI_BINARY_TAXONOMY.md"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "closeout-matrix-integrity",
    "proof_role": "Prove the retained snapshot covers all eleven closed children and merged PRs",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 200,
    "argv": [
      "jq",
      "-e",
      ".entries | length == 11 and all(.[]; .issue_state == \"CLOSED\" and .pr_state == \"MERGED\" and (.checks | length > 0))",
      "docs/reviews/v0.91.7/tools-5407/github-closeout-snapshot-5036.json"
    ],
    "parallel_group": "evidence",
    "defer_reason": null
  },
  {
    "lane": "performance-boundary",
    "proof_role": "Confirm #5037 explicitly declines an unproved material hosted-speedup claim",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 200,
    "argv": [
      "rg",
      "-n",
      "not claim|No material|not proven",
      "docs/reviews/v0.91.7/tools-5407/TOOLS_RELIABILITY_CLOSEOUT_5036.md"
    ],
    "parallel_group": "evidence",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `git diff --check`
- `rg -n validation_manager.py --run docs/tooling/BUILD_ACTION_LOGS.md`
- `rg -n Gate 10D2|csdlc-v2 docs/tooling/ADL_PLATFORM_CLI_BINARY_TAXONOMY.md`
- `jq -e .entries | length == 11 and all(.[]; .issue_state == "CLOSED" and .pr_state == "MERGED" and (.checks | length > 0)) docs/reviews/v0.91.7/tools-5407/github-closeout-snapshot-5036.json`
- `rg -n not claim|No material|not proven docs/reviews/v0.91.7/tools-5407/TOOLS_RELIABILITY_CLOSEOUT_5036.md`

## Failure Semantics

Fail closed on active v1 guidance, omitted sprint children, unsupported build-action-log claims, or unproved hosted performance claims.

## Handoff

Retain typed evidence before convergence.
