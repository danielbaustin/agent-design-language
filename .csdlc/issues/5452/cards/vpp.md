# Validation Planning Prompt

Template: 1.0.0

Issue: 5452

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5452/retained/design.md

Diagram: .csdlc/issues/5452/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-shell-contract",
    "proof_role": "Prove both mixed-result failure combinations and the successful path",
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
      "adl/tools/test_run_aws_spot_builder_image_validation.sh"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/test_run_aws_spot_builder_image_validation.sh`

## Failure Semantics

Fail closed on any primary validation, summary generation, shell syntax, or focused contract failure.

## Handoff

Retain typed evidence before convergence.
