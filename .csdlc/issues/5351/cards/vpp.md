# Validation Planning Prompt

Template: 1.0.0

Issue: 5351

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5351/retained/design.md

Diagram: .csdlc/issues/5351/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove current-registry six cards, reviewed design/diagram, issue-local scope, COTS, budgets, PVF, clean diff, no product change, and typed doctor truth",
    "acceptance_ids": [
      "AC-3",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5351/validate-preparation.rb"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "wp15-terminal-gate",
    "proof_role": "Fail closed until #5354 is merged, typed closed_out, claim-free, retained-receipt-backed, and ancestral",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5351/check-dependencies.rb"
    ],
    "parallel_group": "execution-gate",
    "defer_reason": "Mandatory before execution; expected to fail while WP-15 remains nonterminal"
  },
  {
    "lane": "focused-quality",
    "proof_role": "Run exact focused product, contract, trust, rollback, deletion, demo, and documentation checks before integration",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5351/run-validation-lane.rb",
      "focused-quality"
    ],
    "parallel_group": "quality",
    "defer_reason": "Mandatory after #5354 terminal admission; forbidden during preparation"
  },
  {
    "lane": "integrated-platform",
    "proof_role": "Run the accepted ADL v2, Runtime v3, and C-SDLC v2 integrated platform quality gate and retain exact blocker truth",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 10000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5351/run-validation-lane.rb",
      "integrated-platform"
    ],
    "parallel_group": "quality",
    "defer_reason": "Mandatory after focused gates and exact protected-path amendment; forbidden during preparation"
  },
  {
    "lane": "complete",
    "proof_role": "Run complete dependency, identity, focused, integrated, redaction, budget, blocker, and exact-review proof before publication",
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
    "budget_seconds": 2280,
    "budget_tokens": 12000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5351/run-validation-lane.rb",
      "complete"
    ],
    "parallel_group": "pre-publication",
    "defer_reason": "Mandatory at the exact implementation revision before publication"
  },
  {
    "lane": "post-merge-exact",
    "proof_role": "Re-run dependency ancestry, platform identities, integrated gates, blocker truth, redaction, CI, and WP-17 release predicate after authorized merge",
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
    "budget_seconds": 2280,
    "budget_tokens": 12000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5351/run-validation-lane.rb",
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

- `ruby .csdlc/prepared/issues/5351/validate-preparation.rb`
- `ruby .csdlc/prepared/issues/5351/check-dependencies.rb`
- `ruby .csdlc/prepared/issues/5351/run-validation-lane.rb focused-quality`
- `ruby .csdlc/prepared/issues/5351/run-validation-lane.rb integrated-platform`
- `ruby .csdlc/prepared/issues/5351/run-validation-lane.rb complete`
- `ruby .csdlc/prepared/issues/5351/run-validation-lane.rb post-merge-exact`

## Failure Semantics

Fail closed without execution, public claim, publication, merge, WP-17 release, or closeout on incomplete #5354 terminal truth, claim collision, stale or non-ancestral product identity, Runtime v2 use, failed or missing required proof, hidden blocker, secret or host-bound output, hard-coded address, new dependency, budget breach, deferred validation, stale review, red CI, or absent post-merge proof.

## Handoff

Retain typed evidence before convergence.
