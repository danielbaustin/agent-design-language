# Validation Planning Prompt

Template: 1.0.0

Issue: 5843

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5843/design.md

Diagram: .csdlc/prepared/issues/5843/diagram.mmd

## Selected Lanes

[
  {
    "lane": "canonical-doc-inventory",
    "proof_role": "Require every inventoried canonical document and release claim to have owner, status, and exact evidence or non-claim disposition.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      "-e",
      "require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5843/canonical-doc-inventory.json')); abort 'inventory missing' unless r['rows'].is_a?(Array) && !r['rows'].empty?; abort 'undispositioned claim' unless r['rows'].all? { |x| x['path'] && x['status'] && x['owner'] }"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "claim-boundary-negative",
    "proof_role": "Reject unsupported release, birthday, provider, platform, privacy, governance, legal, personhood, consciousness, and v0.93 completion claims.",
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
      "require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5843/claim-boundary-scan.json')); abort 'claim blockers remain' unless r['blockers'].is_a?(Array) && r['blockers'].empty?"
    ],
    "parallel_group": "negative",
    "defer_reason": null
  },
  {
    "lane": "docs-format-link-command",
    "proof_role": "Validate changed Markdown, YAML/JSON, relative links, commands, version/WP ownership, redaction, and diff hygiene.",
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
      "5843"
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

- `ruby -e require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5843/canonical-doc-inventory.json')); abort 'inventory missing' unless r['rows'].is_a?(Array) && !r['rows'].empty?; abort 'undispositioned claim' unless r['rows'].all? { |x| x['path'] && x['status'] && x['owner'] }`
- `ruby -e require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5843/claim-boundary-scan.json')); abort 'claim blockers remain' unless r['blockers'].is_a?(Array) && r['blockers'].empty?`
- `git diff --check`
- `csdlc-doctor --repo . --issue 5843`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
