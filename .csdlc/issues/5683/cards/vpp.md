# Validation Planning Prompt

Template: 1.0.0

Issue: 5683

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5683/design.md

Diagram: .csdlc/prepared/issues/5683/diagram.mmd

## Selected Lanes

[
  {
    "lane": "unity-observatory-contract",
    "proof_role": "Prove the repository-owned Unity Observatory contract and guarded publication boundary",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/test_v0916_unity_observatory_contract.sh"
    ],
    "parallel_group": "unity-static",
    "defer_reason": null
  },
  {
    "lane": "unity-observatory-diff-hygiene",
    "proof_role": "Prove bounded source hygiene and absence of patch corruption",
    "acceptance_ids": [
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
    "parallel_group": "unity-static",
    "defer_reason": null
  },
  {
    "lane": "unity-observatory-live-playmode",
    "proof_role": "Prove intended project identity, flagship scene, clean Play Mode behavior, and visually inspected dual-resolution presentation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-10"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "bash",
      "adl/tools/probe_unity_mcp_observatory_alignment.sh"
    ],
    "parallel_group": "unity-live",
    "defer_reason": "The alignment probe proves the prerequisite identity; direct Unity-MCP Play Mode and visual inspection complete the non-deterministic presentation proof."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/test_v0916_unity_observatory_contract.sh`
- `git diff --check`
- `bash adl/tools/probe_unity_mcp_observatory_alignment.sh`

## Failure Semantics

Fail closed on project ambiguity, compile errors, unreviewed visual artifacts, fixture data presented as live, licensed payload publication, player or replacement binary builds, missing dual-resolution proof, or unrecorded tooling anomalies.

## Handoff

Retain typed evidence before convergence.
