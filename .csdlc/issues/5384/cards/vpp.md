# Validation Planning Prompt

Template: 1.0.0

Issue: 5384

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5384/retained/design.md

Diagram: .csdlc/issues/5384/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "typed-record",
    "proof_role": "Confirm the #5384 typed projection is structurally valid.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-doctor",
      "--issue",
      "5384"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "direct-inputs",
    "proof_role": "Confirm the four direct acceptance inputs are closed and the WP-13 deferral is encoded.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5384/validate_dependency_gate.rb"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "preparation-scope",
    "proof_role": "Confirm preparation writes remain inside #5384 issue-local paths.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 300,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5384/validate_preparation_scope.rb"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Confirm patch whitespace hygiene.",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 100,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `.adl/bin/csdlc-v2/csdlc-doctor --issue 5384`
- `ruby .csdlc/prepared/issues/5384/validate_dependency_gate.rb`
- `ruby .csdlc/prepared/issues/5384/validate_preparation_scope.rb`
- `git diff --check`

## Failure Semantics

Fail closed on missing current-template proof, incomplete dependency topology, absent merged/closed_out/receipt/ancestry evidence, protected-path widening, stale live truth, unsupported claims, unresolved review findings, or any request to implement, publish, merge, use AWS, use Runtime v2, or invoke raw gh.

## Handoff

Retain typed evidence before convergence.
