# Validation Planning Prompt

Template: 1.0.0

Issue: 5357

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5357/design.md

Diagram: .csdlc/prepared/issues/5357/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove six current typed cards, exact non-overlapping document scope, canonical-document existence and links, structured-file validity, stale-truth exclusions, undispatched handoff state, budgets, diff hygiene, and typed doctor truth",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
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
      ".csdlc/prepared/issues/5357/validate-preparation.rb"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "wp18-terminal-gate",
    "proof_role": "Prove closed first-pass #5356 plus merged final second-pass #5791 after residual coding, current-main integration, and ancestry to the exact external-review target",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5357/check-dependencies.rb"
    ],
    "parallel_group": "execution-gate",
    "defer_reason": "Mandatory before corpus freeze; expected to fail until #5791 merges and current main is integrated"
  },
  {
    "lane": "corpus-dispatch-preflight",
    "proof_role": "Build and verify the immutable all-v0.91.8 exact-revision corpus and dispatch receipt while enforcing identity, independence, redaction, and non-self-inclusion",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5357/run-validation-lane.rb",
      "corpus-dispatch-preflight"
    ],
    "parallel_group": "review",
    "defer_reason": "Mandatory after #5791 merge, current-main integration, and operator freeze; forbidden during documentation preparation"
  },
  {
    "lane": "review-output-contract",
    "proof_role": "Validate findings-first severity order, exact evidence, evidence/inference/author-decision separation, residual risk, and typed synthesis mapping",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5357/run-validation-lane.rb",
      "review-output-contract"
    ],
    "parallel_group": "review",
    "defer_reason": "Mandatory after external reviewer output exists; forbidden before dispatch"
  },
  {
    "lane": "complete",
    "proof_role": "Run final-review ancestry, canonical-doc, link, structured-file, corpus, dispatch, output, redaction, exact-review, and publication preflight",
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
    "budget_tokens": 9000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5357/run-validation-lane.rb",
      "complete"
    ],
    "parallel_group": "pre-publication",
    "defer_reason": "Mandatory at the exact reviewed result revision before publication"
  },
  {
    "lane": "post-merge-exact",
    "proof_role": "Re-run target ancestry, canonical corpus/receipt/output digests, redaction, typed synthesis, CI, and WP-20 handoff after authorized merge",
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
    "budget_tokens": 9000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5357/run-validation-lane.rb",
      "post-merge-exact"
    ],
    "parallel_group": "post-merge",
    "defer_reason": "Mandatory after authorized serialized merge; terminal bookkeeping remains separate"
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/5357/validate-preparation.rb`
- `ruby .csdlc/prepared/issues/5357/check-dependencies.rb`
- `ruby .csdlc/prepared/issues/5357/run-validation-lane.rb corpus-dispatch-preflight`
- `ruby .csdlc/prepared/issues/5357/run-validation-lane.rb review-output-contract`
- `ruby .csdlc/prepared/issues/5357/run-validation-lane.rb complete`
- `ruby .csdlc/prepared/issues/5357/run-validation-lane.rb post-merge-exact`

## Failure Semantics

Fail closed without dispatch, finding acceptance, publication, merge, WP-20 release, or closeout on incomplete #5356 terminal truth, handoff mutation, claim collision, stale/non-ancestral target, mutable or incomplete corpus/receipt identity, undisclosed reviewer control, malformed output, evidence/inference confusion, secret or host-bound data, new dependency, budget breach, deferred gate, stale review, red CI, or absent post-merge proof.

## Handoff

Retain typed evidence before convergence.
