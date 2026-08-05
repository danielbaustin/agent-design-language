# Validation Planning Prompt

Template: 1.0.0

Issue: 5828

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5828/design.md

Diagram: .csdlc/prepared/issues/5828/diagram.mmd

## Selected Lanes

[
  {
    "lane": "memory_palace-runtime-v3",
    "proof_role": "Run the exact Runtime v3 integration target and fail when the selected target contains no tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "memory_palace",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "parallel_group": "5828-core",
    "defer_reason": null
  },
  {
    "lane": "memory-palace-obsmem-trace-binding",
    "proof_role": "Recompute the exact ObsMem, Runtime v3 observability/proof, fixture, output, source-SHA, argv, runner, trace, and citation bindings in the integration receipt.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5828/validate-obsmem-trace-integration.rb",
      ".csdlc/evidence/5828/obsmem-trace-integration-receipt.json"
    ],
    "parallel_group": "5828-integration",
    "defer_reason": null
  },
  {
    "lane": "memory_palace-native-platform-receipts",
    "proof_role": "Recompute and bind exact HEAD, exact test argv, nonzero test count, fixture-tree digest, output digest, runner identity, and native artifact digest for macOS and Linux; require byte-identical semantic output.",
    "acceptance_ids": [
      "AC-4",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5828/validate-native-receipts.rb",
      ".csdlc/evidence/5828/native-platform/macos.json",
      ".csdlc/evidence/5828/native-platform/linux.json"
    ],
    "parallel_group": "5828-platform",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --test memory_palace --no-tests=fail --status-level all`
- `ruby .csdlc/prepared/issues/5828/validate-obsmem-trace-integration.rb .csdlc/evidence/5828/obsmem-trace-integration-receipt.json`
- `ruby .csdlc/prepared/issues/5828/validate-native-receipts.rb .csdlc/evidence/5828/native-platform/macos.json .csdlc/evidence/5828/native-platform/linux.json`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
