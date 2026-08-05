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
    "proof_role": "Recompute the exact ObsMem, Runtime v3 observability/proof, fixture, output, source-SHA, argv, runner, trace, and citation bindings in the integration receipt. [preexec_rejection exit=1 diagnostic_sha256=c3ce76747b2b2ce17fa9746fb2794051041bebcd310d9f4ccc7f7a16dd8c5f6e]",
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
    "lane": "memory_palace-macos-native-ci-producer",
    "proof_role": "Run the issue-local receipt producer on a native GitHub Actions macos runner at exact candidate HEAD and retain the complete nextest log, source manifest, and canonical semantic output.",
    "acceptance_ids": [
      "AC-4",
      "AC-9"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 240,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5828/produce-native-receipt.rb",
      "--platform",
      "macos",
      "--receipt",
      ".csdlc/evidence/5828/native-platform/macos.json",
      "--semantic-output",
      ".csdlc/evidence/5828/native-platform/macos-semantic.json"
    ],
    "parallel_group": "5828-native-produce",
    "defer_reason": "Required on a native GitHub Actions macos runner; missing CI proof blocks portability and review readiness."
  },
  {
    "lane": "memory_palace-linux-native-ci-producer",
    "proof_role": "Run the issue-local receipt producer on a native GitHub Actions linux runner at exact candidate HEAD and retain the complete nextest log, source manifest, and canonical semantic output.",
    "acceptance_ids": [
      "AC-4",
      "AC-9"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 240,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5828/produce-native-receipt.rb",
      "--platform",
      "linux",
      "--receipt",
      ".csdlc/evidence/5828/native-platform/linux.json",
      "--semantic-output",
      ".csdlc/evidence/5828/native-platform/linux-semantic.json"
    ],
    "parallel_group": "5828-native-produce",
    "defer_reason": "Required on a native GitHub Actions linux runner; missing CI proof blocks portability and review readiness."
  },
  {
    "lane": "memory_palace-native-ci-receipt-verification",
    "proof_role": "Independently recompute producer, source-manifest, command-log, and semantic-output digests; parse a positive test count; verify GitHub Actions provenance; and require macOS/Linux semantic equivalence at exact candidate HEAD. [preexec_rejection exit=1 diagnostic_sha256=5dff561a38429e71f769ab6ddc31cf4e13684da2496190a046598e1b3cc281db]",
    "acceptance_ids": [
      "AC-4",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5828/validate-native-receipts.rb",
      ".csdlc/evidence/5828/native-platform/macos.json",
      ".csdlc/evidence/5828/native-platform/linux.json"
    ],
    "parallel_group": "5828-native-verify",
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
- `ruby .csdlc/prepared/issues/5828/produce-native-receipt.rb --platform macos --receipt .csdlc/evidence/5828/native-platform/macos.json --semantic-output .csdlc/evidence/5828/native-platform/macos-semantic.json`
- `ruby .csdlc/prepared/issues/5828/produce-native-receipt.rb --platform linux --receipt .csdlc/evidence/5828/native-platform/linux.json --semantic-output .csdlc/evidence/5828/native-platform/linux-semantic.json`
- `ruby .csdlc/prepared/issues/5828/validate-native-receipts.rb .csdlc/evidence/5828/native-platform/macos.json .csdlc/evidence/5828/native-platform/linux.json`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
