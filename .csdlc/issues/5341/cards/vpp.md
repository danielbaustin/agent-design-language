# Validation Planning Prompt

Template: 1.0.0

Issue: 5341

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5341/retained/design.md

Diagram: .csdlc/issues/5341/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "dependency-gate",
    "proof_role": "Prove #5340, #5342, and #5591 are merged, typed closed_out, receipt-retained, and merged-SHA ancestral to current origin/main before product work",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5341/run_validation_lane.rb",
      "dependency-gate"
    ],
    "parallel_group": "dependency-control",
    "defer_reason": null
  },
  {
    "lane": "mapping-unit",
    "proof_role": "Prove stable plan, engine-event, work-identity, canonical-payload, result, error, correlation, provenance, digest, and trust mapping",
    "acceptance_ids": [
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
      ".csdlc/prepared/issues/5341/run_validation_lane.rb",
      "mapping-unit"
    ],
    "parallel_group": "adapter-behavior",
    "defer_reason": null
  },
  {
    "lane": "canonical-ingress-integration",
    "proof_role": "Prove the adapter uses the real public #5591 canonical ingress and preserves bounded success, saturation, closure, unsupported, conflict, and execution-failed outcomes",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5341/run_validation_lane.rb",
      "canonical-ingress-integration"
    ],
    "parallel_group": "adapter-integration",
    "defer_reason": null
  },
  {
    "lane": "negative-authority",
    "proof_role": "Reject malformed, non-canonical, unverified, tampered, authority-escalating, direct-Runtime, retry, signing, Runtime v2, C-SDLC, AWS, transport, credential, hard-coded-address, and owner-path bypasses",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5341/run_validation_lane.rb",
      "negative-authority"
    ],
    "parallel_group": "adapter-negative",
    "defer_reason": null
  },
  {
    "lane": "complete-adapter-suite",
    "proof_role": "Run all adapter targets, features, integration tests, and doctests as the complete local regression gate",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5341/run_validation_lane.rb",
      "complete-adapter-suite"
    ],
    "parallel_group": "adapter-full",
    "defer_reason": null
  },
  {
    "lane": "strict-quality",
    "proof_role": "Prove exact adapter formatting and strict warning-free all-target all-feature code with FastWork build output",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 420,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5341/run_validation_lane.rb",
      "strict-quality"
    ],
    "parallel_group": "adapter-quality",
    "defer_reason": null
  },
  {
    "lane": "inventory-and-boundary",
    "proof_role": "Prove locked COTS inventory, exact source/test/module/test-count budgets, single-crate scope, and absence of forbidden Runtime v2, Runtime-internal, C-SDLC, AWS, transport, credential, and hard-coded-address references",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5341/run_validation_lane.rb",
      "inventory-and-boundary"
    ],
    "parallel_group": "adapter-quality",
    "defer_reason": null
  },
  {
    "lane": "exact-revision-truth",
    "proof_role": "Prove clean issue diff, typed doctor truth, exact reviewed revision, green publication and merge identity, post-merge ancestry, terminal receipt, and safe prune readiness",
    "acceptance_ids": [
      "AC-1",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 270,
    "budget_tokens": 2500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5341/run_validation_lane.rb",
      "exact-revision-truth"
    ],
    "parallel_group": "lifecycle-control",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/5341/run_validation_lane.rb dependency-gate`
- `ruby .csdlc/prepared/issues/5341/run_validation_lane.rb mapping-unit`
- `ruby .csdlc/prepared/issues/5341/run_validation_lane.rb canonical-ingress-integration`
- `ruby .csdlc/prepared/issues/5341/run_validation_lane.rb negative-authority`
- `ruby .csdlc/prepared/issues/5341/run_validation_lane.rb complete-adapter-suite`
- `ruby .csdlc/prepared/issues/5341/run_validation_lane.rb strict-quality`
- `ruby .csdlc/prepared/issues/5341/run_validation_lane.rb inventory-and-boundary`
- `ruby .csdlc/prepared/issues/5341/run_validation_lane.rb exact-revision-truth`

## Failure Semantics

Fail closed on any non-terminal dependency, missing or malformed receipt, non-ancestral merged SHA, preview-contract assumption, claim collision, shared or owner-path write, authority escalation, nondeterministic mapping, error suppression, trust mutation, Runtime v2 or AWS dependency, listener or credential scope, undeclared COTS, LoC/test/module/time budget breach, skipped or degraded PVF lane, stale or actionable review, red or pending CI, exact-head mismatch, missing post-merge proof, incomplete typed closeout receipt, or unsafe prune.

## Handoff

Retain typed evidence before convergence.
