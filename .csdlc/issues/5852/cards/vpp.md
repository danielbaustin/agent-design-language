# Validation Planning Prompt

Template: 1.0.0

Issue: 5852

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5852/design.md

Diagram: .csdlc/prepared/issues/5852/diagram.mmd

## Selected Lanes

[
  {
    "lane": "release-evidence-manifest",
    "proof_role": "Require every release claim to bind nonempty exact implementation, validation, review, merge, terminal, artifact hash, residual-risk, and non-claim evidence, then recompute every artifact digest.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5852/validate-release-evidence.rb"
    ],
    "parallel_group": "release",
    "defer_reason": null
  },
  {
    "lane": "ceremony-script-preflight",
    "proof_role": "Prove tag/release ordering, identity checks, dry-run behavior, partial-failure recovery, and duplicate-mutation rejection.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 540,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_release_ceremony.sh"
    ],
    "parallel_group": "ceremony",
    "defer_reason": null
  },
  {
    "lane": "release-identity-negative",
    "proof_role": "Reject red checks, active claims, missing receipts/findings, dirty head, tag/release conflict, partial verification, stale assets, and blind retries with an exercised negative-case packet.",
    "acceptance_ids": [
      "AC-1",
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
      "require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5852/ceremony-negative-cases.json')); c=r['cases']; abort 'negative cases missing' unless c.is_a?(Array) && !c.empty?; abort 'unexercised negative case' unless c.all? { |x| x['observed_exit'].is_a?(Integer) && x['observed_exit'] != 0 && x['stderr_sha256'].is_a?(String) && x['stderr_sha256'].match?(/\\A[0-9a-f]{64}\\z/) }"
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
      "5852"
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

- `ruby .csdlc/prepared/issues/5852/validate-release-evidence.rb`
- `bash adl/tools/test_release_ceremony.sh`
- `ruby -e require 'json'; r=JSON.parse(File.read('.csdlc/evidence/5852/ceremony-negative-cases.json')); c=r['cases']; abort 'negative cases missing' unless c.is_a?(Array) && !c.empty?; abort 'unexercised negative case' unless c.all? { |x| x['observed_exit'].is_a?(Integer) && x['observed_exit'] != 0 && x['stderr_sha256'].is_a?(String) && x['stderr_sha256'].match?(/\A[0-9a-f]{64}\z/) }`
- `csdlc-doctor --repo . --issue 5852`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
