# Validation Planning Prompt

Template: 1.0.0

Issue: 5849

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5849/design.md

Diagram: .csdlc/prepared/issues/5849/diagram.mmd

## Selected Lanes

[
  {
    "lane": "v093-prerequisite-map",
    "proof_role": "Require every candidate work area to name exact evidence, blocker, follow-on, or non-claim plus owner and acceptance hook.",
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
      "require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5849/v093-prerequisite-map.json')); abort 'empty map' unless r['rows'].is_a?(Array) && !r['rows'].empty?; abort 'incomplete prerequisite' unless r['rows'].all? { |x| x['work_area'] && x['disposition'] && x['owner'] && x['acceptance_hook'] }"
    ],
    "parallel_group": "handoff",
    "defer_reason": null
  },
  {
    "lane": "candidate-status-negative",
    "proof_role": "Reject issue creation, activation, implementation, schedule, legal-personhood, production-authority, and certification overclaims.",
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
      "require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5849/claim-boundary-scan.json')); abort 'handoff blockers' unless r['blockers'].is_a?(Array) && r['blockers'].empty?"
    ],
    "parallel_group": "negative",
    "defer_reason": null
  },
  {
    "lane": "planning-format-link-dependency",
    "proof_role": "Validate candidate YAML/Markdown, links, dependency coverage, evidence identity, owners, and patch hygiene.",
    "acceptance_ids": [
      "AC-3",
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
      "5849"
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

- `ruby -e require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5849/v093-prerequisite-map.json')); abort 'empty map' unless r['rows'].is_a?(Array) && !r['rows'].empty?; abort 'incomplete prerequisite' unless r['rows'].all? { |x| x['work_area'] && x['disposition'] && x['owner'] && x['acceptance_hook'] }`
- `ruby -e require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5849/claim-boundary-scan.json')); abort 'handoff blockers' unless r['blockers'].is_a?(Array) && r['blockers'].empty?`
- `git diff --check`
- `csdlc-doctor --repo . --issue 5849`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
