# Validation Planning Prompt

Template: 1.0.0

Issue: 5558

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5558/retained/design.md

Diagram: .csdlc/issues/5558/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-guidance",
    "proof_role": "Prove active guidance no longer teaches sunset v1 commands",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/test_cli_owner_command_guidance.sh"
    ],
    "parallel_group": "local-focused",
    "defer_reason": null
  },
  {
    "lane": "owner-csdlc",
    "proof_role": "Prove the complete C-SDLC owner lane including Gate 10A",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/run_owner_validation_lane.sh",
      "csdlc"
    ],
    "parallel_group": "local-owner",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/test_cli_owner_command_guidance.sh`
- `bash adl/tools/run_owner_validation_lane.sh csdlc`

## Failure Semantics

Fail closed on any live v1 route, missing Gate 10A proof, or owner-lane failure.

## Handoff

Retain typed evidence before convergence.
