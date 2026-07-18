# Validation Planning Prompt

Template: 1.0.0

Issue: 5413

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5413/design.md

Diagram: .csdlc/prepared/issues/5413/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-focused",
    "proof_role": "Prove parity, feed authorization, and weather behavior at the owner crate boundary.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "parallel_group": "local-runtime",
    "defer_reason": null
  },
  {
    "lane": "observatory-live-client",
    "proof_role": "Prove a real client consumes an authenticated running HTTPS Runtime v3 endpoint.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      "adl/tools/test_v0917_html_observatory_integrated_proof.sh"
    ],
    "parallel_group": "local-live",
    "defer_reason": null
  },
  {
    "lane": "release-evidence",
    "proof_role": "Validate the complete corrected #5277-#5286 evidence packet.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "python3",
      "-m",
      "json.tool",
      "docs/architecture/runtime_v3_observatory_consumption_5286.v1.json"
    ],
    "parallel_group": "local-docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml`
- `bash adl/tools/test_v0917_html_observatory_integrated_proof.sh`
- `python3 -m json.tool docs/architecture/runtime_v3_observatory_consumption_5286.v1.json`

## Failure Semantics

Fail closed on missing binaries, unauthenticated feed access, mocked live proof, stale weather without explicit state, or incomplete release evidence; downgrade claims rather than infer success.

## Handoff

Retain typed evidence before convergence.
