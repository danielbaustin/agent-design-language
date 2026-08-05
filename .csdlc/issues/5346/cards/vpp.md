# Validation Planning Prompt

Template: 1.0.0

Issue: 5346

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5346/retained/design.md

Diagram: .csdlc/issues/5346/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove six cards, design/diagram, dependency/disjointness gates, protected paths, COTS, budgets, PVF, no-deferral, and clean-root boundaries",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5346/validate-preparation.rb"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "terminal-and-manifest-gate",
    "proof_role": "Fail closed unless all terminal receipts are merged, claim-free, ancestral, rollback-valid, and both deletion manifests are disjoint",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5346/check-dependencies.rb"
    ],
    "parallel_group": "execution-gate",
    "defer_reason": "Run when deletion execution is requested; it must fail closed while any dependency or manifest is incomplete"
  },
  {
    "lane": "eligibility-before-deletion",
    "proof_role": "Recompute denominator, execute existing csdlc-eligibility, verify exact path ownership, and fail closed before any deletion unless the decision is eligible",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5346/run-validation-lane.rb",
      "eligibility-before-deletion"
    ],
    "parallel_group": "deletion",
    "defer_reason": "Mandatory after all gates pass and exact product paths are added to the typed claim"
  },
  {
    "lane": "complete-post-deletion",
    "proof_role": "Prove workspace, consumers, schemas, fixtures, demos, links, install, selector rollback, Runtime v3/C-SDLC v2 boundaries, LoC, dependencies, tests, and evidence",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 10000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5346/run-validation-lane.rb",
      "complete-post-deletion"
    ],
    "parallel_group": "post-deletion",
    "defer_reason": "Mandatory at the exact deletion revision before publication"
  },
  {
    "lane": "post-merge-exact",
    "proof_role": "Re-run manifest identity, dependency ancestry, reduction accounting, full validation, serialized-merge state, and exact revision proof before closeout",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 3600,
    "budget_tokens": 12000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5346/run-validation-lane.rb",
      "post-merge-exact"
    ],
    "parallel_group": "post-merge",
    "defer_reason": "Mandatory after authorized serialized merge and before typed closeout"
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 21600

Tokens: 100000

## Commands

- `ruby .csdlc/prepared/issues/5346/validate-preparation.rb`
- `ruby .csdlc/prepared/issues/5346/check-dependencies.rb`
- `ruby .csdlc/prepared/issues/5346/run-validation-lane.rb eligibility-before-deletion`
- `ruby .csdlc/prepared/issues/5346/run-validation-lane.rb complete-post-deletion`
- `ruby .csdlc/prepared/issues/5346/run-validation-lane.rb post-merge-exact`

## Failure Semantics

Fail closed without deletion, publication, merge, or closeout on missing or non-ancestral terminal evidence, active dependency claims, manifest overlap, unowned paths, eligibility rejection, rollback-window violation, denominator drift, deletion below 80 percent, deferred proof, stale review, red CI, or absent post-merge validation.

## Handoff

Retain typed evidence before convergence.
