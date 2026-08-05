# Validation Planning Prompt

Template: 1.0.0

Issue: 5862

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5862/design.md

Diagram: .csdlc/prepared/issues/5862/diagram.mmd

## Selected Lanes

[
  {
    "lane": "live-wave-contract",
    "proof_role": "Verify exact mapping, approved records, null claims, dependency graph, exclusive paths, and the WP-04.16 integration-proof gate.",
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
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5862/validate-implementation-wave.rb"
    ],
    "parallel_group": "planning",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/5862/validate-implementation-wave.rb`

## Failure Semantics

Fail closed on missing children, denominator drift, active preparation claims, dependency bypass, path overlap, or self-attested integration.

## Handoff

Retain typed evidence before convergence.
