# Validation Planning Prompt

Template: 1.0.0

Issue: 5846

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5846/design.md

Diagram: .csdlc/prepared/issues/5846/diagram.mmd

## Selected Lanes

[
  {
    "lane": "packet-and-specialist-roster",
    "proof_role": "Require the exact six-lane specialist roster, reviewer-authored digest-bound reports at the packet target SHA, complete finding reconciliation, and coverage-backed rationale for every zero-finding lane.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 720,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5846/validate-internal-review.rb"
    ],
    "parallel_group": "review",
    "defer_reason": null
  },
  {
    "lane": "review-redaction-negative",
    "proof_role": "Reject secrets, private paths, hidden local state, unsupported severities, stale packet identity, and incomplete reviewer lanes from retained validation evidence.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      "-e",
      "require 'json'; r=JSON.parse(File.read('docs/reviews/v0.92/internal-review-5846/validation.json')); abort 'review validation blockers' unless r['blockers'].is_a?(Array) && r['blockers'].empty?; abort 'negative checks absent' unless r['negative_checks'].is_a?(Array) && !r['negative_checks'].empty?"
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
      "5846"
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

- `ruby .csdlc/prepared/issues/5846/validate-internal-review.rb`
- `ruby -e require 'json'; r=JSON.parse(File.read('docs/reviews/v0.92/internal-review-5846/validation.json')); abort 'review validation blockers' unless r['blockers'].is_a?(Array) && r['blockers'].empty?; abort 'negative checks absent' unless r['negative_checks'].is_a?(Array) && !r['negative_checks'].empty?`
- `csdlc-doctor --repo . --issue 5846`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
