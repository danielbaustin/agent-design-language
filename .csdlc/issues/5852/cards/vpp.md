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
    "lane": "exact-release-manifest",
    "proof_role": "Bind every release claim to exact reviewed head, ancestral merge, semantic review and terminal evidence, recomputed hashes, notes, checklist, handoff, risks, and non-claims.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5852/validate-release-evidence.rb",
      "manifest"
    ],
    "parallel_group": "manifest",
    "defer_reason": null
  },
  {
    "lane": "ceremony-preflight",
    "proof_role": "Require every milestone issue terminal and claim-free, rerun the ceremony tests, and run exact-head v0.92 preflight.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 360,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5852/validate-release-evidence.rb",
      "ceremony"
    ],
    "parallel_group": "ceremony",
    "defer_reason": null
  },
  {
    "lane": "ceremony-negative-rerun",
    "proof_role": "Rerun the real ceremony test suite and require observed dirty, branch, tag, duplicate, and partial-state rejection with fresh output digest.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5852/validate-release-evidence.rb",
      "negative"
    ],
    "parallel_group": "negative",
    "defer_reason": null
  },
  {
    "lane": "live-release-readback",
    "proof_role": "Prove annotated tag target, GitHub release publication, exact notes, complete assets, ancestry, and terminal claim-free milestone truth.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5852/validate-release-evidence.rb",
      "post-publication"
    ],
    "parallel_group": "post-release",
    "defer_reason": null
  },
  {
    "lane": "typed-card-doctor",
    "proof_role": "Validate the exact six-card bundle and design approval.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
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

- `ruby .csdlc/prepared/issues/5852/validate-release-evidence.rb manifest`
- `ruby .csdlc/prepared/issues/5852/validate-release-evidence.rb ceremony`
- `ruby .csdlc/prepared/issues/5852/validate-release-evidence.rb negative`
- `ruby .csdlc/prepared/issues/5852/validate-release-evidence.rb post-publication`
- `csdlc-doctor --repo . --issue 5852`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
