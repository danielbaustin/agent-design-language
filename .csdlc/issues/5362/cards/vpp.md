# Validation Planning Prompt

Template: 1.0.0

Issue: 5362

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5362/design.md

Diagram: .csdlc/prepared/issues/5362/diagram.mmd

## Selected Lanes

[
  {
    "lane": "typed-csdlc-doctor-5362",
    "proof_role": "Diagnose the #5362 typed issue packet, claim, card projections, and design/diagram digest consistency",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5362"
    ],
    "parallel_group": "wp21-preparation-hygiene",
    "defer_reason": null
  },
  {
    "lane": "typed-csdlc-validate-5362",
    "proof_role": "Run the focused typed validation/finalize surface for #5362 preparation if the packet remains bound and non-executing",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1500,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      ".",
      "--request",
      ".csdlc/prepared/issues/5362/validate-preparation.json"
    ],
    "parallel_group": "wp21-preparation-hygiene",
    "defer_reason": null
  },
  {
    "lane": "focused-hygiene",
    "proof_role": "Run git diff --check, allowed-path checks, dependency-ledger JSON parse, and forbidden-scope scans",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "manual-focused-hygiene",
      "issue-5362-only"
    ],
    "parallel_group": "wp21-preparation-hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 5362`
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate --root . --request .csdlc/prepared/issues/5362/validate-preparation.json`
- `manual-focused-hygiene issue-5362-only`

## Failure Semantics

Fail closed on missing live predecessor merge, stale ancestry, unsupported feature disposition, or v0.92 overclaim.

## Handoff

Retain typed evidence before convergence.
