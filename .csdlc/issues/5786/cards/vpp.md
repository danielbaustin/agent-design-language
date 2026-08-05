# Validation Planning Prompt

Template: 1.0.0

Issue: 5786

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5786/design.md

Diagram: .csdlc/prepared/issues/5786/diagram.mmd

## Selected Lanes

[
  {
    "lane": "deletion-inventory",
    "proof_role": "Prove exhaustive dispositions, owners, replacement references, exceptions, and the pinned deletion denominator.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      "-e",
      "require 'json'; d=JSON.parse(File.read('.csdlc/evidence/5786/deletion-manifest.json')); abort 'empty inventory' unless d['rows'].is_a?(Array) && !d['rows'].empty?; abort 'unowned row' unless d['rows'].all? { |r| r['path'] && r['disposition'] && r['owner'] }"
    ],
    "parallel_group": "inventory",
    "defer_reason": null
  },
  {
    "lane": "replacement-parity-negative",
    "proof_role": "Prove supported ADL v2/Runtime v3/C-SDLC v2 behavior, failure, rollback, artifact, and trace parity for removed bands.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 10000,
    "argv": [
      "bash",
      "adl/tools/run_owner_validation_lane.sh",
      "all"
    ],
    "parallel_group": "parity",
    "defer_reason": null
  },
  {
    "lane": "thin-cli-clean-install-platform",
    "proof_role": "Prove the supported thin CLI and clean installation on the declared macOS/Linux release platforms with no Runtime v2 route.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/install_owner_binaries.sh"
    ],
    "parallel_group": "platform",
    "defer_reason": "Run locally and in required platform CI only after the deletion candidate is implemented."
  },
  {
    "lane": "diff-and-stale-reference-hygiene",
    "proof_role": "Reject malformed patches and surviving active references to deleted legacy owners.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
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
      "5786"
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

- `ruby -e require 'json'; d=JSON.parse(File.read('.csdlc/evidence/5786/deletion-manifest.json')); abort 'empty inventory' unless d['rows'].is_a?(Array) && !d['rows'].empty?; abort 'unowned row' unless d['rows'].all? { |r| r['path'] && r['disposition'] && r['owner'] }`
- `bash adl/tools/run_owner_validation_lane.sh all`
- `bash adl/tools/install_owner_binaries.sh`
- `git diff --check`
- `csdlc-doctor --repo . --issue 5786`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
