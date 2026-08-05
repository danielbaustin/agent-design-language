# Validation Planning Prompt

Template: 1.0.0

Issue: 5850

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5850/design.md

Diagram: .csdlc/prepared/issues/5850/diagram.mmd

## Selected Lanes

[
  {
    "lane": "derived-terminal-universe",
    "proof_role": "Derive the expected v0.92 issue universe from canonical wave authority and compare every row with live GitHub, typed phase, receipt, claim, and registered-worktree truth.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5850/validate-closeout-plan.rb",
      "universe"
    ],
    "parallel_group": "universe",
    "defer_reason": null
  },
  {
    "lane": "derived-closeout-dag",
    "proof_role": "Reconstruct and topologically validate the exact finish, claim release, cleanup, WP-29, WP-30, umbrella-closeout, and v0.93-acceptance sequence.",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 1800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5850/validate-closeout-plan.rb",
      "dag"
    ],
    "parallel_group": "dag",
    "defer_reason": null
  },
  {
    "lane": "exercised-terminal-negatives",
    "proof_role": "Mutate the derived row contract for every stale, red, missing-review/receipt, active-claim, dirty, partial, duplicate, unknown, and unowned case and require the gate to block.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 1800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5850/validate-closeout-plan.rb",
      "negative"
    ],
    "parallel_group": "negative",
    "defer_reason": null
  },
  {
    "lane": "typed-card-doctor",
    "proof_role": "Validate the exact rendered six-card bundle, cross-card references, digests, statuses, and canonical issue record.",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5850"
    ],
    "parallel_group": "typed-readback",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/prepared/issues/5850/validate-closeout-plan.rb universe`
- `ruby .csdlc/prepared/issues/5850/validate-closeout-plan.rb dag`
- `ruby .csdlc/prepared/issues/5850/validate-closeout-plan.rb negative`
- `csdlc-doctor --repo . --issue 5850`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
