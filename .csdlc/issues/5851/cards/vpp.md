# Validation Planning Prompt

Template: 1.0.0

Issue: 5851

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5851/design.md

Diagram: .csdlc/prepared/issues/5851/diagram.mmd

## Selected Lanes

[
  {
    "lane": "independent-universe-dag-comparison",
    "proof_role": "Rebuild and compare the expected issue/PR/receipt/claim/worktree/release universe and dependency DAG.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      "-e",
      "require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5851/universe-comparison.json')); abort 'comparison mismatch' unless r['missing'].is_a?(Array) && r['missing'].empty? && r['duplicates'].empty? && r['cycles'].empty? && r['unowned'].empty?"
    ],
    "parallel_group": "review",
    "defer_reason": null
  },
  {
    "lane": "handoff-boundary-review",
    "proof_role": "Validate v0.93 evidence/blocker/owner/acceptance coverage, candidate status, and governance/security/legal/certification non-claims.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      "-e",
      "require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5851/handoff-review.json')); abort 'handoff review blockers' unless r['blockers'].is_a?(Array) && r['blockers'].empty?"
    ],
    "parallel_group": "review",
    "defer_reason": null
  },
  {
    "lane": "closeout-ceremony-negative",
    "proof_role": "Reject missing/stale/red/active-claim/absent-receipt/dirty/partial/duplicate/premature-closeout and activation scenarios.",
    "acceptance_ids": [
      "AC-4",
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
      "require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5851/negative-cases.json')); abort 'negative case escaped' unless r['cases'].is_a?(Array) && r['cases'].all? { |x| x['outcome']=='blocked' }"
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
      "5851"
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

- `ruby -e require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5851/universe-comparison.json')); abort 'comparison mismatch' unless r['missing'].is_a?(Array) && r['missing'].empty? && r['duplicates'].empty? && r['cycles'].empty? && r['unowned'].empty?`
- `ruby -e require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5851/handoff-review.json')); abort 'handoff review blockers' unless r['blockers'].is_a?(Array) && r['blockers'].empty?`
- `ruby -e require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5851/negative-cases.json')); abort 'negative case escaped' unless r['cases'].is_a?(Array) && r['cases'].all? { |x| x['outcome']=='blocked' }`
- `csdlc-doctor --repo . --issue 5851`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
