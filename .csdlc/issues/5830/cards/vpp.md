# Validation Planning Prompt

Template: 1.0.0

Issue: 5830

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5830/design.md

Diagram: .csdlc/prepared/issues/5830/diagram.mmd

## Selected Lanes

[
  {
    "lane": "cognitive_profile-runtime-v3",
    "proof_role": "Run the exact Runtime v3 integration target and fail when the selected target contains no tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
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
      "cognitive_profile",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "parallel_group": "5830-core",
    "defer_reason": null
  },
  {
    "lane": "cognitive_profile-native-platform-receipts",
    "proof_role": "Recompute and bind exact HEAD, exact test argv, nonzero test count, fixture-tree digest, output digest, runner identity, and native artifact digest for macOS and Linux; require byte-identical semantic output.",
    "acceptance_ids": [
      "AC-4",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5830/validate-native-receipts.rb",
      ".csdlc/evidence/5830/native-platform/macos.json",
      ".csdlc/evidence/5830/native-platform/linux.json"
    ],
    "parallel_group": "5830-platform",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --test cognitive_profile --no-tests=fail --status-level all`
- `ruby .csdlc/prepared/issues/5830/validate-native-receipts.rb .csdlc/evidence/5830/native-platform/macos.json .csdlc/evidence/5830/native-platform/linux.json`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
