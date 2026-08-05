# Validation Planning Prompt

Template: 1.0.0

Issue: 4741

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/4741/retained/design.md

Diagram: .csdlc/issues/4741/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "unity-editor-liveness-unit",
    "proof_role": "Prove mode selection, semantic progress, generic classifier routing, and cleanup without launching Unity",
    "acceptance_ids": [
      "AC-1",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/test_v0916_unity_observatory_local_runtime_consumption_unit.sh"
    ],
    "parallel_group": "unity-liveness-static",
    "defer_reason": null
  },
  {
    "lane": "unity-editor-liveness-contract",
    "proof_role": "Prove Observatory wrapper contract and issue-owned path integration",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_v0916_unity_observatory_contract.sh"
    ],
    "parallel_group": "unity-liveness-static",
    "defer_reason": null
  },
  {
    "lane": "unity-editor-selector-registration",
    "proof_role": "Prove the validation selector chooses the focused Unity liveness lane for issue-owned wrapper paths",
    "acceptance_ids": [
      "AC-10",
      "AC-11"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_select_validation_lanes.sh"
    ],
    "parallel_group": "unity-liveness-static",
    "defer_reason": null
  },
  {
    "lane": "unity-editor-live-or-staged-proof",
    "proof_role": "Retain one exact editor-mode or staged-batch outcome with semantic progress evidence",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-9"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      "adl/tools/test_v0916_unity_observatory_local_runtime_consumption.sh"
    ],
    "parallel_group": "unity-liveness-live",
    "defer_reason": "Run only after deterministic wrapper and selector lanes pass and the operator-selected project is available; retain an exact fail-closed result when no safe mode exists."
  },
  {
    "lane": "unity-editor-diff-hygiene",
    "proof_role": "Prove bounded text and shell hygiene",
    "acceptance_ids": [
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "unity-liveness-static",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/test_v0916_unity_observatory_local_runtime_consumption_unit.sh`
- `bash adl/tools/test_v0916_unity_observatory_contract.sh`
- `bash adl/tools/test_select_validation_lanes.sh`
- `bash adl/tools/test_v0916_unity_observatory_local_runtime_consumption.sh`
- `git diff --check`

## Failure Semantics

Fail closed on ambiguous project ownership, undeclared proof mode, arbitrary total-runtime ceilings, non-semantic log activity, unsafe staging, locally built owner binaries, adjacent Unity scope, or unsupported readiness claims.

## Handoff

Retain typed evidence before convergence.
