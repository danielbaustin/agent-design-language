# Validation Planning Prompt

Template: 1.0.0

Issue: 5823

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5823/design.md

Diagram: .csdlc/prepared/issues/5823/diagram.mmd

## Selected Lanes

[
  {
    "lane": "portable-runner-contract",
    "proof_role": "Compile and run the new portable runner crate's exact contract target, proving its typed request/result, provenance, adapter boundary, artifacts, redaction, timeout, and cleanup semantics.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "tools/remote_validation/Cargo.toml",
      "--test",
      "contract"
    ],
    "parallel_group": "runner-contract",
    "defer_reason": null
  },
  {
    "lane": "aws-adapter-negative",
    "proof_role": "Exercise unreachable provider, stale revision, malformed output, timeout, cancellation, redaction, and cleanup failures without a live paid run.",
    "acceptance_ids": [
      "AC-3",
      "AC-5",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 400,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_run_aws_spot_remote_validation_lane.sh"
    ],
    "parallel_group": "adapter-fixtures",
    "defer_reason": null
  },
  {
    "lane": "nessus-and-local-fallback",
    "proof_role": "Prove the existing owned-runner adapter and same-profile local/no-network fallback contract.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 400,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_run_nessus_remote_validation.sh"
    ],
    "parallel_group": "adapter-fixtures",
    "defer_reason": null
  },
  {
    "lane": "native-platform-contract-matrix",
    "proof_role": "Require native live Linux and macOS receipts plus live-native or explicitly non-native fixture Windows proof; reject blocked rows.",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5823/validate-platform-matrix.rb"
    ],
    "parallel_group": "platform",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace errors and support exact-revision review.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path tools/remote_validation/Cargo.toml --test contract`
- `bash adl/tools/test_run_aws_spot_remote_validation_lane.sh`
- `bash adl/tools/test_run_nessus_remote_validation.sh`
- `ruby .csdlc/prepared/issues/5823/validate-platform-matrix.rb`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
