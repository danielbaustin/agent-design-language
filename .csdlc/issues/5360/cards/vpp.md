# Validation Planning Prompt

Template: 1.0.0

Issue: 5360

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5360/retained/design.md

Diagram: .csdlc/issues/5360/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove current-registry six cards, reviewed design/diagram, exact paths, dependency gate, COTS, budgets, PVF, clean diff, zero product changes, and typed doctor truth",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5360/validate-preparation.rb"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "wp16-terminal-gate",
    "proof_role": "Fail closed until #5351 is merged, typed closed_out, claim-free, retained-receipt-backed, and ancestral",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5360/check-dependencies.rb"
    ],
    "parallel_group": "execution-gate",
    "defer_reason": "Mandatory before implementation; expected to fail while WP-16 remains nonterminal"
  },
  {
    "lane": "focused-doc-alignment",
    "proof_role": "Validate exact claim inventory, structured paths, links, classifications, product ownership, budgets, and blocker routing",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5360/run-validation-lane.rb",
      "focused-doc-alignment"
    ],
    "parallel_group": "documentation",
    "defer_reason": "Mandatory after #5351 terminal admission and exact protected-path amendment; forbidden during preparation"
  },
  {
    "lane": "complete",
    "proof_role": "Run complete dependency, identity, structured-document, claim, redaction, budget, blocker, and exact-review proof before publication",
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
    "budget_seconds": 900,
    "budget_tokens": 8000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5360/run-validation-lane.rb",
      "complete"
    ],
    "parallel_group": "pre-publication",
    "defer_reason": "Mandatory at the exact implementation revision before publication"
  },
  {
    "lane": "post-merge-exact",
    "proof_role": "Re-run dependency ancestry, documentation identities, claim truth, redaction, CI, and WP-18 release predicate after authorized merge",
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
    "budget_seconds": 900,
    "budget_tokens": 8000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5360/run-validation-lane.rb",
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

- `ruby .csdlc/prepared/issues/5360/validate-preparation.rb`
- `ruby .csdlc/prepared/issues/5360/check-dependencies.rb`
- `ruby .csdlc/prepared/issues/5360/run-validation-lane.rb focused-doc-alignment`
- `ruby .csdlc/prepared/issues/5360/run-validation-lane.rb complete`
- `ruby .csdlc/prepared/issues/5360/run-validation-lane.rb post-merge-exact`

## Failure Semantics

Fail closed without shared-document or product edits, public claim, publication, merge, WP-18 release, or closeout on incomplete #5351 terminal truth, claim collision, stale or contradictory identity, unsupported release claim, Runtime v2 use, failed or missing required proof, hidden blocker, secret or host-bound output, hard-coded address, new dependency, budget breach, deferred validation, stale review, red CI, or absent post-merge proof.

## Handoff

Retain typed evidence before convergence.
