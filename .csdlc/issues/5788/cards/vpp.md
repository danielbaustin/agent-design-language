# Validation Planning Prompt

Template: 1.0.0

Issue: 5788

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5788/design.md

Diagram: .csdlc/prepared/issues/5788/diagram.mmd

## Selected Lanes

[
  {
    "lane": "owner-binary-install-contract",
    "proof_role": "Prove current inventory and exact lockfile preservation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      "adl/tools/test_owner_binary_install.sh"
    ],
    "parallel_group": "tooling-contracts",
    "defer_reason": null
  },
  {
    "lane": "owner-validation-contract",
    "proof_role": "Prove validation plans use lock-preserving current targets",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      "adl/tools/test_owner_validation_lane.sh"
    ],
    "parallel_group": "tooling-contracts",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/test_owner_binary_install.sh`
- `bash adl/tools/test_owner_validation_lane.sh`

## Failure Semantics

Restore only invocation-created lock drift to exact pre-invocation bytes and return nonzero with the affected path.

## Handoff

Retain typed evidence before convergence.
