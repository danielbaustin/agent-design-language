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
    "lane": "terminal-universe-schema",
    "proof_role": "Require every v0.92 row to carry complete GitHub, typed, SOR, receipt, claim, worktree, dependency, classification, owner, and next-action fields.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      "-e",
      "require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5850/issue-universe.json')); k=%w[issue github_state typed_phase sor_state receipt_state claim_state worktree_state classification owner next_action]; abort 'empty universe' unless r['rows'].is_a?(Array) && !r['rows'].empty?; abort 'incomplete row' unless r['rows'].all? { |x| k.all? { |y| x.key?(y) } }"
    ],
    "parallel_group": "universe",
    "defer_reason": null
  },
  {
    "lane": "closeout-dag",
    "proof_role": "Prove the terminal and ceremony sequence is complete, acyclic, owner-bound, and preserves typed finish/release/cleanup authority.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      "-e",
      "require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5850/closeout-dag.json')); abort 'cycle or owner gap' unless r['acyclic']==true && r['unowned'].is_a?(Array) && r['unowned'].empty?"
    ],
    "parallel_group": "dag",
    "defer_reason": null
  },
  {
    "lane": "terminal-negative-cases",
    "proof_role": "Reject stale, red, missing-review/receipt, active-claim, dirty, partial-release, duplicate-retry, unknown, and unowned scenarios.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      "-e",
      "require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5850/negative-cases.json')); abort 'negative cases not blocked' unless r['cases'].is_a?(Array) && r['cases'].all? { |x| x['outcome']=='blocked' }"
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

- `ruby -e require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5850/issue-universe.json')); k=%w[issue github_state typed_phase sor_state receipt_state claim_state worktree_state classification owner next_action]; abort 'empty universe' unless r['rows'].is_a?(Array) && !r['rows'].empty?; abort 'incomplete row' unless r['rows'].all? { |x| k.all? { |y| x.key?(y) } }`
- `ruby -e require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5850/closeout-dag.json')); abort 'cycle or owner gap' unless r['acyclic']==true && r['unowned'].is_a?(Array) && r['unowned'].empty?`
- `ruby -e require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5850/negative-cases.json')); abort 'negative cases not blocked' unless r['cases'].is_a?(Array) && r['cases'].all? { |x| x['outcome']=='blocked' }`
- `csdlc-doctor --repo . --issue 5850`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
