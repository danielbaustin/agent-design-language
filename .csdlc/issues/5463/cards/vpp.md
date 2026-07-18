# Validation Planning Prompt

Template: 1.0.0

Issue: 5463

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5463/design.md

Diagram: .csdlc/prepared/issues/5463/diagram.mmd

## Selected Lanes

[
  {
    "lane": "ci-runtime-contract",
    "proof_role": "Prove canonical pins and deprecated-SHA absence",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "hosted-annotation-proof",
    "proof_role": "Prove Node 20 deprecation annotations are absent",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 1000,
    "argv": [
      "gh",
      "pr",
      "checks",
      "PR"
    ],
    "parallel_group": "github-hosted",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/test_ci_runtime_contracts.sh`
- `gh pr checks PR`

## Failure Semantics

Fail closed on floating pins, partial replacement, contract failure, incompatible action inputs, or any hosted Node 20 deprecation annotation.

## Handoff

Retain typed evidence before convergence.
