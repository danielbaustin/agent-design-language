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
    "lane": "canonical-doc-release-truth",
    "proof_role": "Parse every inventoried JSON/YAML document, resolve every relative Markdown link, execute declared command checks, enforce v0.92/WP ownership, and reject machine-local paths or credential-like text.",
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
      ".csdlc/prepared/issues/5843/validate-doc-release-truth.rb"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "claim-boundary-negative",
    "proof_role": "Require the retained claim-boundary scanner to identify no unsupported release, birthday, provider, platform, privacy, governance, legal, personhood, consciousness, or v0.93 completion claims.",
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
      "require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5843/claim-boundary-scan.json')); abort 'claim blockers remain' unless r['blockers'].is_a?(Array) && r['blockers'].empty?; abort 'scan corpus missing' unless r['scanned_paths'].is_a?(Array) && !r['scanned_paths'].empty? && r['scanned_paths'].all? { |p| File.file?(p) }"
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

- `ruby .csdlc/prepared/issues/5843/validate-doc-release-truth.rb`
- `ruby -e require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5843/claim-boundary-scan.json')); abort 'claim blockers remain' unless r['blockers'].is_a?(Array) && r['blockers'].empty?; abort 'scan corpus missing' unless r['scanned_paths'].is_a?(Array) && !r['scanned_paths'].empty? && r['scanned_paths'].all? { |p| File.file?(p) }`
- `csdlc-doctor --repo . --issue 5843`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
