# Validation Planning Prompt

Template: 1.0.0

Issue: 5615

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5615/retained/design.md

Diagram: .csdlc/issues/5615/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "ci-path-policy-contract",
    "proof_role": "Prove exact metadata-only, C-SDLC v2 Rust, Runtime, workspace, and mixed routing",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/test_ci_path_policy.sh"
    ],
    "parallel_group": "ci-contracts",
    "defer_reason": null
  },
  {
    "lane": "ci-runtime-contract",
    "proof_role": "Prove standalone job materialization, stable aggregate wiring, and unchanged required check names",
    "acceptance_ids": [
      "AC-2",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "parallel_group": "ci-contracts",
    "defer_reason": null
  },
  {
    "lane": "portable-cargo-wrapper",
    "proof_role": "Prove declared external root, FastWork preference, exported Cargo state, and fail-closed refusal",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5615/validate_portable_wrapper_contract.sh"
    ],
    "parallel_group": "ci-contracts",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-standalone",
    "proof_role": "Prove the actual standalone C-SDLC v2 crate with tests, formatting, and strict Clippy using external Cargo state",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5615/run_csdlc_v2_standalone.sh"
    ],
    "parallel_group": "csdlc-v2",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/test_ci_path_policy.sh`
- `bash adl/tools/test_ci_runtime_contracts.sh`
- `bash .csdlc/prepared/issues/5615/validate_portable_wrapper_contract.sh`
- `bash .csdlc/prepared/issues/5615/run_csdlc_v2_standalone.sh`

## Failure Semantics

Fail closed on ambiguous routing, missing selected standalone proof, aggregate mismatch, mixed-route suppression, unwritable external Cargo state, or focused/hosted validation failure.

## Handoff

Retain typed evidence before convergence.
