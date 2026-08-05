# Validation Planning Prompt

Template: 1.0.0

Issue: 5878

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5878/design.md

Diagram: .csdlc/prepared/issues/5878/diagram.mmd

## Selected Lanes

[
  {
    "lane": "exact-child-tests",
    "proof_role": "Run the exact nonzero distributed_guardian integration target at the candidate head.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_guardian",
      "--no-tests=fail"
    ],
    "parallel_group": "child",
    "defer_reason": null
  },
  {
    "lane": "production-distributed-guardian",
    "proof_role": "Launch production Guardians and kernels and prove authenticated API/WSS, partition, fencing, migration, recovery, and shutdown with retained logs.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "bash",
      "adl/tools/validate_v092_distributed_guardian.sh"
    ],
    "parallel_group": "integration",
    "defer_reason": null
  },
  {
    "lane": "native-distributed-receipts",
    "proof_role": "Recompute digest-bound macOS, Linux, and native Windows production receipts from actual command logs and artifacts.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      "adl/tools/validate_v092_distributed_native_receipts.rb"
    ],
    "parallel_group": "native",
    "defer_reason": null
  },
  {
    "lane": "exact-revision-proof-receipt",
    "proof_role": "Reject self-attestation by recomputing exact-head command logs, artifacts, negative cases, and native receipt digests.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5878/validate-proof-receipt.rb"
    ],
    "parallel_group": "receipt",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo nextest run --manifest-path adl-runtime/Cargo.toml --test distributed_guardian --no-tests=fail`
- `bash adl/tools/validate_v092_distributed_guardian.sh`
- `ruby adl/tools/validate_v092_distributed_native_receipts.rb`
- `ruby .csdlc/prepared/issues/5878/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on stale dependencies, path overlap, zero tests, invalid evidence, insecure fallback, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
