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
    "lane": "review-packet-manifest",
    "proof_role": "Validate exact target, packet object inventory/digest, issue/PR/typed identity, and included/excluded/unknown/redacted surfaces.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      "-e",
      "require 'json'; r=JSON.parse(File.read('docs/reviews/v0.92/internal-review-5846/packet-manifest.json')); abort 'target missing' unless r['target_sha']; abort 'corpus missing' unless r['paths'].is_a?(Array) && !r['paths'].empty?"
    ],
    "parallel_group": "packet",
    "defer_reason": null
  },
  {
    "lane": "specialist-findings-schema",
    "proof_role": "Require all specialist lanes and evidence-backed stable findings with explicit disagreement/duplicate accounting.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      "-e",
      "require 'json'; r=JSON.parse(File.read('docs/reviews/v0.92/internal-review-5846/findings.json')); abort 'lanes incomplete' unless r['lanes_complete']==true; abort 'bad finding' unless r.fetch('findings',[]).all? { |f| %w[id severity evidence invariant owner disposition].all? { |k| f[k] } }"
    ],
    "parallel_group": "findings",
    "defer_reason": null
  },
  {
    "lane": "review-redaction-negative",
    "proof_role": "Reject secrets, private paths, hidden local state, unsupported severities, stale packet identity, and incomplete reviewer lanes.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      "-e",
      "require 'json'; r=JSON.parse(File.read('docs/reviews/v0.92/internal-review-5846/validation.json')); abort 'review validation blockers' unless r['blockers'].is_a?(Array) && r['blockers'].empty?"
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

- `ruby -e require 'json'; r=JSON.parse(File.read('docs/reviews/v0.92/internal-review-5846/packet-manifest.json')); abort 'target missing' unless r['target_sha']; abort 'corpus missing' unless r['paths'].is_a?(Array) && !r['paths'].empty?`
- `ruby -e require 'json'; r=JSON.parse(File.read('docs/reviews/v0.92/internal-review-5846/findings.json')); abort 'lanes incomplete' unless r['lanes_complete']==true; abort 'bad finding' unless r.fetch('findings',[]).all? { |f| %w[id severity evidence invariant owner disposition].all? { |k| f[k] } }`
- `ruby -e require 'json'; r=JSON.parse(File.read('docs/reviews/v0.92/internal-review-5846/validation.json')); abort 'review validation blockers' unless r['blockers'].is_a?(Array) && r['blockers'].empty?`
- `csdlc-doctor --repo . --issue 5846`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
