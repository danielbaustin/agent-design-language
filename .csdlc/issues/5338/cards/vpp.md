# Validation Planning Prompt

Template: 1.0.0

Issue: 5338

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5338/design.md

Diagram: .csdlc/prepared/issues/5338/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove all six canonical cards, issue-local artifacts, dependency stop, protected paths, stable identity, COTS, budgets, and FastWork boundary are present and internally consistent",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5338/validate-preparation.rb"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "compiler-focused",
    "proof_role": "Prove typed plan lowering, deterministic resolution and expansion, failure cases, and inert ExecutionPlan contracts",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 120,
    "budget_tokens": 6000,
    "argv": [
      ".csdlc/prepared/issues/5338/validate-compiler.sh",
      "focused"
    ],
    "parallel_group": "compiler-local",
    "defer_reason": "Execute only after #5339 is merged and typed closed_out and the compiler crate exists"
  },
  {
    "lane": "compiler-quality",
    "proof_role": "Prove warning-free clean-room compiler source and tests",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      ".csdlc/prepared/issues/5338/validate-compiler.sh",
      "quality"
    ],
    "parallel_group": "compiler-local",
    "defer_reason": "Execute only after #5339 is merged and typed closed_out and the compiler crate exists"
  },
  {
    "lane": "deterministic-replay",
    "proof_role": "Prove golden stable identities, equivalent-input permutations, repeated clean-process replay, canonical plan bytes, stable diagnostics, and identity locality",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 6000,
    "argv": [
      ".csdlc/prepared/issues/5338/validate-compiler.sh",
      "determinism"
    ],
    "parallel_group": "compiler-determinism",
    "defer_reason": "Execute only after #5339 is merged and typed closed_out and its landed fixture map is available"
  },
  {
    "lane": "compiler-budgets",
    "proof_role": "Enforce the exact direct COTS set, forbidden dependency families, 3500 implementation LoC, 3500 test/fixture LoC, and full deterministic validation within 600 seconds",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      ".csdlc/prepared/issues/5338/validate-compiler.sh",
      "budgets"
    ],
    "parallel_group": "compiler-budget",
    "defer_reason": "Execute at the implementation revision with measured dependency, LoC, and latency evidence"
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/5338/validate-preparation.rb`
- `.csdlc/prepared/issues/5338/validate-compiler.sh focused`
- `.csdlc/prepared/issues/5338/validate-compiler.sh quality`
- `.csdlc/prepared/issues/5338/validate-compiler.sh determinism`
- `.csdlc/prepared/issues/5338/validate-compiler.sh budgets`

## Failure Semantics

Fail closed without implementation, publication, merge, or closeout on a false or stale #5339 dependency signal, ambiguous resolution or expansion semantics, identity collision, nondeterministic plan or diagnostic bytes, silent fixture skip, execution-authority leak, forbidden dependency, unsupported budget variance, stale review, red CI, absent merge authorization, or incomplete post-merge typed evidence.

## Handoff

Retain typed evidence before convergence.
