# Validation Planning Prompt

Template: 1.0.0

Issue: 5847

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5847/design.md

Diagram: .csdlc/prepared/issues/5847/diagram.mmd

## Selected Lanes

[
  {
    "lane": "external-packet-digest",
    "proof_role": "Recompute SHA-256 over every sorted packet path and reject missing objects, stale packet identity, or target drift.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5847/validate-external-review.rb",
      "packet"
    ],
    "parallel_group": "packet",
    "defer_reason": null
  },
  {
    "lane": "handoff-redaction-authority-negative",
    "proof_role": "Reject secrets/private state, invalid reviewer authority, unsafe links or commands, mutable packet identity, and unsupported approval language.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      "-e",
      "require 'json'; r=JSON.parse(File.read('docs/reviews/v0.92/external-review-5847/validation.json')); abort 'handoff blockers' unless r['blockers'].is_a?(Array) && r['blockers'].empty?; abort 'authority checks absent' unless r['authority_checks'].is_a?(Array) && !r['authority_checks'].empty?"
    ],
    "parallel_group": "negative",
    "defer_reason": null
  },
  {
    "lane": "received-report-integrity",
    "proof_role": "Require a reviewer-authored report path, report digest, reviewer identity, packet digest, target SHA, and complete findings index.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5847/validate-external-review.rb",
      "report"
    ],
    "parallel_group": "receive",
    "defer_reason": "Run only after the operator-authorized reviewer response is actually received."
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
      "5847"
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

- `ruby .csdlc/prepared/issues/5847/validate-external-review.rb packet`
- `ruby -e require 'json'; r=JSON.parse(File.read('docs/reviews/v0.92/external-review-5847/validation.json')); abort 'handoff blockers' unless r['blockers'].is_a?(Array) && r['blockers'].empty?; abort 'authority checks absent' unless r['authority_checks'].is_a?(Array) && !r['authority_checks'].empty?`
- `ruby .csdlc/prepared/issues/5847/validate-external-review.rb report`
- `csdlc-doctor --repo . --issue 5847`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
