# Validation Planning Prompt

Template: 1.0.0

Issue: 5728

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5728/retained/design.md

Diagram: .csdlc/issues/5728/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "merged_docs_patch",
    "proof_role": "Exact committed-patch and bounded recovery validation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 200,
    "argv": [
      "git",
      "diff",
      "--check",
      "f62e36f1a70cae3adee71c715a3f5456df08f917^",
      "f62e36f1a70cae3adee71c715a3f5456df08f917"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `git diff --check f62e36f1a70cae3adee71c715a3f5456df08f917^ f62e36f1a70cae3adee71c715a3f5456df08f917`

## Failure Semantics

Fail closed on any identity, review, publication, or terminal-evidence mismatch.

## Handoff

Retain typed evidence before convergence.
