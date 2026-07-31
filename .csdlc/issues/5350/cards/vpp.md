# Validation Planning Prompt

Template: 1.0.0

Issue: 5350

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5350/retained/design.md

Diagram: .csdlc/issues/5350/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "subject-and-corpus-verification",
    "proof_role": "Verify both exact subjects, dependency receipts or truthful squash integration, corpus bundle, evidence envelopes, portable stream hashes, and command policy",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5350/validate-parity.sh",
      "subjects"
    ],
    "parallel_group": "parity-local",
    "defer_reason": null
  },
  {
    "lane": "exact-shadow-comparison",
    "proof_role": "Run and compare every corpus case, behavior, repetition, equivalence group, and difference group with only explicit reviewed normalization",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 8000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5350/validate-parity.sh",
      "compare"
    ],
    "parallel_group": "parity-local",
    "defer_reason": null
  },
  {
    "lane": "runtime-workcell-overlay",
    "proof_role": "Verify exact Runtime ten-group and WP-10A live evidence, reject non-live credit, and preserve #5361 downstream direction",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5350/validate-parity.sh",
      "overlays"
    ],
    "parallel_group": "parity-overlay",
    "defer_reason": null
  },
  {
    "lane": "parity-complete",
    "proof_role": "Enforce zero blockers or unclassified rows, strict lint, exact COTS and scope, 1500/2000 LoC budgets, 120-test ceiling, deterministic rerun, no network, and one final exact-revision review",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 12000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5350/validate-parity.sh",
      "complete"
    ],
    "parallel_group": "parity-final",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash .csdlc/prepared/issues/5350/validate-parity.sh subjects`
- `bash .csdlc/prepared/issues/5350/validate-parity.sh compare`
- `bash .csdlc/prepared/issues/5350/validate-parity.sh overlays`
- `bash .csdlc/prepared/issues/5350/validate-parity.sh complete`

## Failure Semantics

Fail closed without parity credit, publication, acceptance, soak, cutover, or deletion on identity drift, invalid corpus/evidence, unknown command or normalization, missing case/group/overlay, unclassified mismatch, non-live Runtime credit, absent WP-10A proof, forbidden dependency, budget failure, or deferred acceptance proof.

## Handoff

Retain typed evidence before convergence.
