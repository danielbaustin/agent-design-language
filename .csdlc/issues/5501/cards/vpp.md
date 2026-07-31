# Validation Planning Prompt

Template: 1.0.0

Issue: 5501

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5501/retained/design.md

Diagram: .csdlc/issues/5501/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove six-card, design, diagram, live-merge dependency, scope, COTS, budget, PVF, and zero-product-change preparation truth",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5501/validate-preparation.rb"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "dependency-terminal-gate",
    "proof_role": "Fail closed until #5349, #5499, #5498, #5500, and #5502 live merged heads are ancestral to the execution revision; typed closeout and retained receipts remain audit-only",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5501/check-dependencies.rb"
    ],
    "parallel_group": "execution-gate",
    "defer_reason": "Run only when execution is requested; it must fail closed until every dependency has live merge plus ancestry truth"
  },
  {
    "lane": "live-manifest",
    "proof_role": "Validate two-to-four real shard identities, disjoint claims and paths, bounded context, negative case, dashboard observation plan, serialized gates, timing plan, and fair baseline before task creation",
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
    "budget_seconds": 120,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5501/run-validation-lane.rb",
      "live-manifest"
    ],
    "parallel_group": "execution",
    "defer_reason": "Mandatory after terminal dependencies and before live task creation; not selected during preparation"
  },
  {
    "lane": "live-two-shard",
    "proof_role": "Execute and retain the real admitted multi-task workcell with negative refusal, live observations, review, convergence, and serialized integration truth",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 12000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5501/run-validation-lane.rb",
      "live-two-shard"
    ],
    "parallel_group": "live-proof",
    "defer_reason": "Mandatory execution proof after dependency admission; forbidden during preparation"
  },
  {
    "lane": "baseline-comparison",
    "proof_role": "Run equivalent bounded single-agent work and retain elapsed, coordination, failure, retry, and comparability evidence",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 8000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5501/run-validation-lane.rb",
      "baseline-comparison"
    ],
    "parallel_group": "live-proof",
    "defer_reason": "Mandatory execution evidence after the live workcell; forbidden during preparation"
  },
  {
    "lane": "post-merge-exact",
    "proof_role": "Re-run dependency ancestry, evidence identity, redaction, comparison, CI, exact review, and consumer handoff after authorized merge",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 3300,
    "budget_tokens": 10000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5501/run-validation-lane.rb",
      "post-merge-exact"
    ],
    "parallel_group": "post-merge",
    "defer_reason": "Mandatory after authorized merge and before closeout"
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/5501/validate-preparation.rb`
- `ruby .csdlc/prepared/issues/5501/check-dependencies.rb`
- `ruby .csdlc/prepared/issues/5501/run-validation-lane.rb live-manifest`
- `ruby .csdlc/prepared/issues/5501/run-validation-lane.rb live-two-shard`
- `ruby .csdlc/prepared/issues/5501/run-validation-lane.rb baseline-comparison`
- `ruby .csdlc/prepared/issues/5501/run-validation-lane.rb post-merge-exact`

## Failure Semantics

Fail closed without task execution, publication, merge, closeout, or consumer handoff on incomplete dependencies, claim collision, fewer than two real disjoint writable shards, stale or unbounded context, manual dashboard assertions, missing negative case, unreviewed output, ambiguous convergence, unserialized integration, incomparable baseline, secret/path leakage, red CI, stale review, deferred proof, or budget breach.

## Handoff

Retain typed evidence before convergence.
