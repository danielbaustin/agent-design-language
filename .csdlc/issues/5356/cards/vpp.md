# Validation Planning Prompt

Template: 1.0.0

Issue: 5356

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5356/design.md

Diagram: .csdlc/prepared/issues/5356/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove six cards, reviewed design/diagram, exact issue-local scope, corpus/matrix, identity/findings/COTS/budgets/PVF/no-product-change and doctor truth",
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
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5356/validate-preparation.rb"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "wp17-terminal-gate",
    "proof_role": "Fail closed until #5360 is merged, typed closed_out, claim-free, retained-receipt-backed, and ancestral",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5356/check-dependencies.rb"
    ],
    "parallel_group": "execution-gate",
    "defer_reason": "Mandatory before review execution; expected to fail while WP-17 is nonterminal"
  },
  {
    "lane": "specialist-review",
    "proof_role": "Run mandatory code/security/tests/docs/architecture/evidence lanes against one exact frozen corpus",
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
    "budget_seconds": 1200,
    "budget_tokens": 12000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5356/run-validation-lane.rb",
      "code"
    ],
    "parallel_group": "specialists",
    "defer_reason": "Mandatory after WP-17 terminal and corpus freeze; forbidden during preparation"
  },
  {
    "lane": "synthesis-review-quality",
    "proof_role": "Synthesize findings-first register and verify severity/disposition, identity, completeness, redaction, provenance, and downstream blockers",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 8000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5356/run-validation-lane.rb",
      "synthesis"
    ],
    "parallel_group": "synthesis",
    "defer_reason": "Mandatory after all six specialist results exist"
  },
  {
    "lane": "complete",
    "proof_role": "Run corpus, all specialist, synthesis, quality, exact identity, typed review, redaction, provenance, and pre-publication proof",
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
    "budget_seconds": 3600,
    "budget_tokens": 16000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5356/run-validation-lane.rb",
      "complete"
    ],
    "parallel_group": "pre-publication",
    "defer_reason": "Mandatory at exact review revision before publication"
  },
  {
    "lane": "post-merge-exact",
    "proof_role": "Recheck WP-17 ancestry, corpus/revision identity, packet/findings, CI, redaction/provenance and downstream WP-19 gate after authorized merge",
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
    "budget_seconds": 3600,
    "budget_tokens": 16000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5356/run-validation-lane.rb",
      "post-merge-exact"
    ],
    "parallel_group": "post-merge",
    "defer_reason": "Mandatory after authorized merge and before closeout"
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 21600

Tokens: 100000

## Commands

- `ruby .csdlc/prepared/issues/5356/validate-preparation.rb`
- `ruby .csdlc/prepared/issues/5356/check-dependencies.rb`
- `ruby .csdlc/prepared/issues/5356/run-validation-lane.rb code`
- `ruby .csdlc/prepared/issues/5356/run-validation-lane.rb synthesis`
- `ruby .csdlc/prepared/issues/5356/run-validation-lane.rb complete`
- `ruby .csdlc/prepared/issues/5356/run-validation-lane.rb post-merge-exact`

## Failure Semantics

Fail closed without review execution, publication, merge, external-review handoff, or closeout on incomplete WP-17 truth, claim collision, stale identity, incomplete corpus, missing lane, undispositioned finding, secret/private/host-bound evidence, unsupported claim, Runtime v2/AWS use, new dependency, budget breach, deferred proof, red CI, or absent post-merge validation.

## Handoff

Retain typed evidence before convergence.
