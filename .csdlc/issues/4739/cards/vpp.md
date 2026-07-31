# Validation Planning Prompt

Template: 1.0.0

Issue: 4739

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/4739/design.md

Diagram: .csdlc/prepared/issues/4739/diagram.mmd

## Selected Lanes

[
  {
    "lane": "unity-mcp-alignment-unit",
    "proof_role": "Execute the dedicated no-Unity parser, mismatch, redaction, and read-only failure fixtures",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/test_v0916_unity_mcp_alignment_unit.sh"
    ],
    "parallel_group": "unity-mcp-static",
    "defer_reason": null
  },
  {
    "lane": "unity-mcp-alignment-contract",
    "proof_role": "Prove repository shell contract and focused static integration for the alignment surface",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_v0916_unity_observatory_contract.sh"
    ],
    "parallel_group": "unity-mcp-static",
    "defer_reason": null
  },
  {
    "lane": "unity-mcp-selector-registration",
    "proof_role": "Prove issue-owned paths select the focused non-Unity MCP alignment lane",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_select_validation_lanes.sh"
    ],
    "parallel_group": "unity-mcp-static",
    "defer_reason": null
  },
  {
    "lane": "unity-mcp-diff-hygiene",
    "proof_role": "Prove bounded text and script hygiene for the issue-owned diff",
    "acceptance_ids": [
      "AC-4",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "unity-mcp-static",
    "defer_reason": null
  },
  {
    "lane": "unity-mcp-live-read-only",
    "proof_role": "Execute the repository-owned probe against the intended project and retain exact read-only alignment or blocker truth",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "-c",
      "test -n \"${ADL_UNITY_PROJECT_PATH:-}\" && bash adl/tools/probe_unity_mcp_observatory_alignment.sh --project \"$ADL_UNITY_PROJECT_PATH\""
    ],
    "parallel_group": "unity-mcp-live",
    "defer_reason": "Set ADL_UNITY_PROJECT_PATH to the intended canonical project and run only when that Unity project is available; the repository-relative probe path remains fixed and reviewable."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/test_v0916_unity_mcp_alignment_unit.sh`
- `bash adl/tools/test_v0916_unity_observatory_contract.sh`
- `bash adl/tools/test_select_validation_lanes.sh`
- `git diff --check`
- `bash -c test -n "${ADL_UNITY_PROJECT_PATH:-}" && bash adl/tools/probe_unity_mcp_observatory_alignment.sh --project "$ADL_UNITY_PROJECT_PATH"`

## Failure Semantics

Fail closed on project or endpoint ambiguity, fixed-port assumptions, cloud fallback, missing read-only proof, secret exposure, broad process scans, adjacent Unity scope, or unsupported readiness claims.

## Handoff

Retain typed evidence before convergence.
