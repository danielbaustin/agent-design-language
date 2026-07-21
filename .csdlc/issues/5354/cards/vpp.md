# Validation Planning Prompt

Template: 1.0.0

Issue: 5354

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5354/design.md

Diagram: .csdlc/prepared/issues/5354/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove current-registry six cards, reviewed design/diagram, issue-local scope, COTS, budgets, PVF, no-product-change, and clean typed doctor truth",
    "acceptance_ids": [
      "AC-1",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5354/validate-preparation.rb"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "wp14a-terminal-gate",
    "proof_role": "Fail closed until #5384 is merged, typed closed_out, claim-free, retained-receipt-backed, and ancestral",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5354/check-dependencies.rb"
    ],
    "parallel_group": "execution-gate",
    "defer_reason": "Mandatory before execution; expected to fail while WP-14A remains nonterminal"
  },
  {
    "lane": "integrated-live-demo",
    "proof_role": "Prove fresh-consumer ADL v2 compile through Runtime v3 execution/observation and applicable C-SDLC v2 lifecycle boundaries at accepted revisions",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 8000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5354/run-validation-lane.rb",
      "integrated-live-demo"
    ],
    "parallel_group": "demo",
    "defer_reason": "Mandatory after #5384 terminal admission and exact protected-path amendment; forbidden during preparation"
  },
  {
    "lane": "claim-boundary-matrix",
    "proof_role": "Validate retained citations and proven, blocked, deferred, non-applicable, and non-claim dispositions across the convergence packet and v0.91.8 matrices",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5354/run-validation-lane.rb",
      "claim-boundary-matrix"
    ],
    "parallel_group": "demo",
    "defer_reason": "Mandatory after integrated evidence exists; forbidden during preparation"
  },
  {
    "lane": "complete",
    "proof_role": "Run the complete integrated, negative, redaction, identity, budget, dependency, matrix, and exact-review pre-publication proof",
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
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 10000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5354/run-validation-lane.rb",
      "complete"
    ],
    "parallel_group": "pre-publication",
    "defer_reason": "Mandatory at the exact implementation revision before publication"
  },
  {
    "lane": "post-merge-exact",
    "proof_role": "Re-run dependency ancestry, accepted product identities, integrated proof, claim boundaries, redaction, CI, and consumer evidence after authorized merge",
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
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 10000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5354/run-validation-lane.rb",
      "post-merge-exact"
    ],
    "parallel_group": "post-merge",
    "defer_reason": "Mandatory after authorized serialized merge and before typed closeout"
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/5354/validate-preparation.rb`
- `ruby .csdlc/prepared/issues/5354/check-dependencies.rb`
- `ruby .csdlc/prepared/issues/5354/run-validation-lane.rb integrated-live-demo`
- `ruby .csdlc/prepared/issues/5354/run-validation-lane.rb claim-boundary-matrix`
- `ruby .csdlc/prepared/issues/5354/run-validation-lane.rb complete`
- `ruby .csdlc/prepared/issues/5354/run-validation-lane.rb post-merge-exact`

## Failure Semantics

Fail closed without implementation, public claim, publication, merge, or closeout on incomplete #5384 terminal truth, claim collision, stale or non-ancestral product identity, Runtime v2 use, missing integrated/negative evidence, secret or host-bound output, hard-coded addresses, unsupported claim wording, matrix drift, new dependency, budget breach, deferred proof, stale review, red CI, or absent post-merge validation.

## Handoff

Retain typed evidence before convergence.
