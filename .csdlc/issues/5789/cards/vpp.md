# Validation Planning Prompt

Template: 1.0.0

Issue: 5789

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5789/design.md

Diagram: .csdlc/prepared/issues/5789/diagram.mmd

## Selected Lanes

[
  {
    "lane": "observatory-browser-smoke",
    "proof_role": "Browser-level default, explicit Runtime v3, controls, links, export, and messaging proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 8000,
    "argv": [
      "bash",
      "adl/tools/test_v0917_html_observatory_integrated_proof.sh"
    ],
    "parallel_group": "observatory-focused",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-observatory-focused",
    "proof_role": "Runtime v3 readiness/feed/WebSocket/write-path regression proof",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "observatory"
    ],
    "parallel_group": "runtime-focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash adl/tools/test_v0917_html_observatory_integrated_proof.sh`
- `cargo test --locked --manifest-path adl/Cargo.toml observatory`

## Failure Semantics

Fail closed on stale retained data presented as live truth, ungoverned operator sends, hidden auth bypass, unresolved WebSocket/control contradiction, process-liveness mismatch, or validation that does not exercise the checked-in page.

## Handoff

Retain typed evidence before convergence.
