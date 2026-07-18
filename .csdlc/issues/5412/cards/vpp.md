# Validation Planning Prompt

Template: 1.0.0

Issue: 5412

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5412/design.md

Diagram: .csdlc/prepared/issues/5412/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-authenticity",
    "proof_role": "Focused checkpoint and private-state positive/negative proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 1200,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "identity_memory",
      "--test",
      "private_state"
    ],
    "parallel_group": "runtime-v3-focused",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-real-soak",
    "proof_role": "Explicit bounded 100-cycle guardian soak",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      "adl/tools/run_runtime_v3_guardian_soak.sh"
    ],
    "parallel_group": "runtime-v3-soak",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-loc",
    "proof_role": "Reproducible source count and reviewed disposition",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/report_runtime_v3_loc.sh"
    ],
    "parallel_group": "runtime-v3-focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test identity_memory --test private_state`
- `bash adl/tools/run_runtime_v3_guardian_soak.sh`
- `bash adl/tools/report_runtime_v3_loc.sh`

## Failure Semantics

Fail closed on signature, identity, sequence, head, lineage, soak, or budget uncertainty; preserve opt-in cutover truth.

## Handoff

Retain typed evidence before convergence.
