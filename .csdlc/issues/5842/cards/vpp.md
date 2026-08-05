# Validation Planning Prompt

Template: 1.0.0

Issue: 5842

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5842/design.md

Diagram: .csdlc/prepared/issues/5842/diagram.mmd

## Selected Lanes

[
  {
    "lane": "feature-matrix-schema",
    "proof_role": "Require a nonempty, complete matrix whose accepted rows carry exact implementation, review, merge, validation, integration, platform, and terminal identity.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
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
      "require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5842/feature-completion-matrix.json')); abort 'rows missing' unless r['rows'].is_a?(Array) && !r['rows'].empty?; keys=%w[owner_issue reviewed_head merge_sha validation_ref integration_ref terminal_ref disposition]; abort 'incomplete row' unless r['rows'].all? { |x| keys.all? { |k| x[k] } }"
    ],
    "parallel_group": "gate",
    "defer_reason": null
  },
  {
    "lane": "prohibited-evidence-negative",
    "proof_role": "Reject fixtures, receipts-only, demo mode, synthetic success, provider substitution, stale review, missing ancestry, and unsupported platform credit.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "ruby",
      "-e",
      "require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5842/negative-cases.json')); abort 'negative cases not rejected' unless r['cases'].is_a?(Array) && !r['cases'].empty? && r['cases'].all? { |x| x['outcome']=='rejected' }"
    ],
    "parallel_group": "negative",
    "defer_reason": null
  },
  {
    "lane": "docs-yaml-link-hygiene",
    "proof_role": "Validate gate docs, YAML/JSON, links, and patch hygiene without broad product tests.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "hygiene",
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
      "5842"
    ],
    "parallel_group": "typed-readback",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby -e require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5842/feature-completion-matrix.json')); abort 'rows missing' unless r['rows'].is_a?(Array) && !r['rows'].empty?; keys=%w[owner_issue reviewed_head merge_sha validation_ref integration_ref terminal_ref disposition]; abort 'incomplete row' unless r['rows'].all? { |x| keys.all? { |k| x[k] } }`
- `ruby -e require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5842/negative-cases.json')); abort 'negative cases not rejected' unless r['cases'].is_a?(Array) && !r['cases'].empty? && r['cases'].all? { |x| x['outcome']=='rejected' }`
- `git diff --check`
- `csdlc-doctor --repo . --issue 5842`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
