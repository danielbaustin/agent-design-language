# Validation Planning Prompt

Template: 1.0.0

Issue: 5848

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5848/design.md

Diagram: .csdlc/prepared/issues/5848/diagram.mmd

## Selected Lanes

[
  {
    "lane": "finding-universe-disposition",
    "proof_role": "Require complete provenance-preserving rows and exact owner, scope, disposition, evidence, fix/review/PR/merge fields where applicable.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 2500,
    "argv": [
      "ruby",
      "-e",
      "require 'json'; r=JSON.parse(File.read('docs/reviews/v0.92/remediation-5848/dispositions.json')); abort 'empty finding universe' unless r['rows'].is_a?(Array) && !r['rows'].empty?; keys=%w[id source severity evidence owner disposition]; abort 'incomplete row' unless r['rows'].all? { |x| keys.all? { |k| x[k].is_a?(String) && !x[k].strip.empty? } }"
    ],
    "parallel_group": "disposition",
    "defer_reason": null
  },
  {
    "lane": "open-stale-unauthorized-negative",
    "proof_role": "Reject open actionable, stale fix/review SHA, unmerged, failed, missing-proof, unowned follow-on, or unauthorized accepted-risk rows.",
    "acceptance_ids": [
      "AC-3",
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
      "require 'json'; r=JSON.parse(File.read('docs/reviews/v0.92/remediation-5848/validation.json')); abort 'remediation blockers remain' unless r['blockers'].is_a?(Array) && r['blockers'].empty?"
    ],
    "parallel_group": "negative",
    "defer_reason": null
  },
  {
    "lane": "affected-quality-regression",
    "proof_role": "Execute every affected WP-22 row validator and every impacted release-claim validator at the exact remediation target SHA; require an explicit no-impact disposition when no release claim changed.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5848/validate-remediation-regressions.rb"
    ],
    "parallel_group": "regression",
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
      "5848"
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

- `ruby -e require 'json'; r=JSON.parse(File.read('docs/reviews/v0.92/remediation-5848/dispositions.json')); abort 'empty finding universe' unless r['rows'].is_a?(Array) && !r['rows'].empty?; keys=%w[id source severity evidence owner disposition]; abort 'incomplete row' unless r['rows'].all? { |x| keys.all? { |k| x[k].is_a?(String) && !x[k].strip.empty? } }`
- `ruby -e require 'json'; r=JSON.parse(File.read('docs/reviews/v0.92/remediation-5848/validation.json')); abort 'remediation blockers remain' unless r['blockers'].is_a?(Array) && r['blockers'].empty?`
- `ruby .csdlc/prepared/issues/5848/validate-remediation-regressions.rb`
- `csdlc-doctor --repo . --issue 5848`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
