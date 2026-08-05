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
    "lane": "live-closeout-universe",
    "proof_role": "Rebuild the full issue denominator and reconcile nonempty live GitHub checks/reviews/PR identity with typed phase, SOR, receipt, claim, worktree, and evidence truth. [preexec_rejection exit=1 diagnostic_sha256=5c85c87de40a7f5dac8aaabc8f0899abec3d0779786282599d12dde488b85982]",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 360,
    "budget_tokens": 3500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5850/validate-closeout-plan.rb",
      "universe"
    ],
    "parallel_group": "universe",
    "defer_reason": null
  },
  {
    "lane": "closeout-dag",
    "proof_role": "Require the complete issue and ceremony node universe and prove the closeout DAG is acyclic. [preexec_rejection exit=1 diagnostic_sha256=f2670f8fa894ad6b58f473fbb07582ce13a61ba48f370b509a1a6bb5a91f8ec0]",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1200,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5850/validate-closeout-plan.rb",
      "dag"
    ],
    "parallel_group": "dag",
    "defer_reason": null
  },
  {
    "lane": "reconstructed-closeout-negatives",
    "proof_role": "Start from accepted rows, mutate exactly one declared field, and require the real classifier to produce exactly the expected blocker. [preexec_rejection exit=1 diagnostic_sha256=5c4dc336757c494ab542fb90a64a27c32357fac189b6a24ebdec10097f4047be]",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 360,
    "budget_tokens": 3500,
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
    "proof_role": "Validate the exact six-card bundle and design approval.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
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
