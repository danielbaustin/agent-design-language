# Validation Planning Prompt

Template: 1.0.0

Issue: 5464

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5464/design.md

Diagram: .csdlc/prepared/issues/5464/diagram.mmd

## Selected Lanes

[
  {
    "lane": "ci-runtime-contract",
    "proof_role": "Prove canonical nextest pin, version, and fallback policy",
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
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "hosted-warning-proof",
    "proof_role": "Prove unsupported-binary and cargo-binstall fallback warnings are absent",
    "acceptance_ids": [
      "AC-4"
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

Fail closed on floating or stale pins, partial replacement, missing fallback none, contract failure, or any hosted fallback warning.

## Handoff

Retain typed evidence before convergence.
