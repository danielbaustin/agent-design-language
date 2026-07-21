# Validation Planning Prompt

Template: 1.0.0

Issue: 5614

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5614/design.md

Diagram: .csdlc/prepared/issues/5614/diagram.mmd

## Selected Lanes

[
  {
    "lane": "spot-redaction-contract",
    "proof_role": "Prove runtime access-key redaction without a source literal and support post-merge alert resolution",
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
      "adl/tools/test_run_aws_spot_ci_profile.sh"
    ],
    "parallel_group": "security-fixture",
    "defer_reason": null
  },
  {
    "lane": "artifact-finalize-contract",
    "proof_role": "Prove coupled artifact sanitization remains green",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/test_aws_spot_artifact_finalize.sh"
    ],
    "parallel_group": "security-fixture",
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
- `bash adl/tools/test_aws_spot_artifact_finalize.sh`

## Failure Semantics

Fail closed if a tracked literal remains or either sanitizer test fails.

## Handoff

Retain typed evidence before convergence.
