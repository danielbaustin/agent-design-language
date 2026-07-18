# Validation Planning Prompt

Template: 1.0.0

Issue: 5467

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5467/retained/design.md

Diagram: .csdlc/issues/5467/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "backend-snapshot-local",
    "proof_role": "Local contract and backend-routing proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      "adl/tools/test_run_aws_spot_ci_profile.sh"
    ],
    "parallel_group": "ci-contract-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/test_run_aws_spot_ci_profile.sh`

## Failure Semantics

Fail closed when the workflow snapshot changes, any assertion is bypassed, or backend input is invalid.

## Handoff

Retain typed evidence before convergence.
